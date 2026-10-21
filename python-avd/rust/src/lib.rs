// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! Python adapters for AVD data publication and native immutable model access.
//!
//! Domain models, schema compatibility and archive operations belong to `avd-data`.
//! This extension registers Python classes and converts Rust results at the Python boundary.

use std::path::PathBuf;
use std::sync::Arc;

use data_views_py::PyValueHandle;
use data_views_py::ViewHandle;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

/// Result of validating and conditionally publishing one host archive.
#[pyclass(get_all, module = "pyavd._rust")]
#[derive(Debug)]
struct PublicationResult {
    /// Published path, or none when validation rejected the input.
    destination: Option<PathBuf>,
    /// JSON encoded input parse diagnostics.
    input_diagnostics_json: String,
    /// JSON encoded validation errors.
    errors_json: String,
    /// JSON encoded validation warnings.
    warnings_json: String,
    /// JSON encoded coercion and informational diagnostics.
    infos_json: String,
}

/// Immutable custom-config payload, opaque to Python field access.
///
/// Retains the archive owner. JSON materialization is explicit for legacy merger and facts
/// serialization boundaries; Rust consumers continue to use typed relaxed views.
#[pyclass(
    name = "OpaqueData",
    module = "pyavd._rust",
    frozen,
    skip_from_py_object
)]
#[derive(Clone, Debug)]
struct PyOpaqueData(ViewHandle);

#[pymethods]
impl PyOpaqueData {
    #[new]
    fn new(handle: PyRef<'_, PyValueHandle>) -> Self {
        Self(handle.0.clone())
    }

    fn __bool__(&self) -> bool {
        !self.0.is_empty()
    }

    fn to_json(&self) -> String {
        self.0.to_json()
    }
}

/// Validate and coerce one host's AVD Design inputs and publish an rkyv archive when valid.
#[pyfunction]
fn archive_avd_design(
    input_json: &str,
    destination: PathBuf,
    schema_archive: PathBuf,
) -> PyResult<PublicationResult> {
    let publication = avd_data::archive_avd_design(input_json, &destination, &schema_archive)
        .map_err(|error| PyRuntimeError::new_err(error.to_string()))?;
    let published_destination = publication.published.then_some(destination);
    Ok(PublicationResult {
        destination: published_destination,
        input_diagnostics_json: to_json(&publication.input_diagnostics)?,
        errors_json: to_json(&publication.validation.errors)?,
        warnings_json: to_json(&publication.validation.warnings)?,
        infos_json: to_json(&publication.validation.infos)?,
    })
}

/// Open a compatible AVD Design archive and return its checked, immutable root model.
#[pyfunction]
fn open_avd_design(
    py: Python<'_>,
    archive: PathBuf,
    schema_archive: PathBuf,
) -> PyResult<Py<PyAny>> {
    let opened = avd_data::open_avd_design(&archive, &schema_archive)
        .map_err(|error| PyRuntimeError::new_err(error.to_string()))?;
    opened
        .root()
        .map_err(|error| PyRuntimeError::new_err(error.to_string()))?;
    let module = py
        .import("pyavd._rust")?
        .getattr("_validated_data")?
        .cast_into::<pyo3::types::PyModule>()?;
    data_views_py::wrap_with_schema(
        &module,
        "AVDDesign",
        opened.data().root_handle(),
        Arc::clone(opened.schemas()),
    )
}

fn to_json(value: &impl serde::Serialize) -> PyResult<String> {
    serde_json::to_string(value).map_err(|error| PyRuntimeError::new_err(error.to_string()))
}

/// AVD Rust extension.
#[pymodule]
mod _rust {
    use pyo3::prelude::*;

    #[pymodule_init]
    fn initialize(module: &Bound<'_, PyModule>) -> PyResult<()> {
        let py = module.py();
        let native = PyModule::new(py, "pyavd._validated_data")?;
        let opaque = py.get_type::<super::PyOpaqueData>();
        let undefined = py.import("pyavd._utils.undefined")?.getattr("Undefined")?;
        data_views_py::install_models::<avd_data::avd_design::AvdDesign<'static>>(
            &native, &opaque, &undefined,
        )?;
        native.add("OpaqueData", &opaque)?;
        native.add_function(wrap_pyfunction!(super::open_avd_design, &native)?)?;
        py.import("sys")?
            .getattr("modules")?
            .set_item("pyavd._validated_data", &native)?;
        module.add("_validated_data", native)?;
        Ok(())
    }
    #[pymodule_export]
    use super::PublicationResult;
    #[pymodule_export]
    use super::PyOpaqueData;
    #[pymodule_export]
    use super::archive_avd_design;
}
