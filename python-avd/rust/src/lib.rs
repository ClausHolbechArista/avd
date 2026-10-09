// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! AVD-owned Python extension for validated-data publication and views.

use std::path::PathBuf;
use std::sync::Arc;

use avdschema::Store;
use pyo3::exceptions::PyIndexError;
use pyo3::exceptions::PyRuntimeError;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::PyBool;
use pyo3::types::PyInt;
use pyo3::types::PyString;
use pyo3::types::PyTuple;
use validated_data::DataStore;
use validated_data::DictHandle;
use validated_data::ListHandle;
use validated_data::PrimaryKeyValue;
use validated_data::ValueHandle;
use validation::archive::validate_json_to_archive;

#[allow(dead_code, non_snake_case, reason = "generated API exceeds the current call graph and preserves schema key spelling")]
#[rustfmt::skip]
mod generated {
    pub mod avd_design;
}

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

/// Private owner-preserving handle passed to generated Python model classes.
#[pyclass(
    name = "_ValueHandle",
    module = "pyavd._rust",
    frozen,
    skip_from_py_object
)]
#[derive(Clone, Debug)]
struct PyValueHandle(ValueHandle);

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
struct PyOpaqueData(ValueHandle);

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

/// Generic immutable dictionary view used as the base of generated Python model classes.
#[pyclass(
    name = "_DictView",
    module = "pyavd._rust",
    subclass,
    frozen,
    skip_from_py_object
)]
#[derive(Clone, Debug)]
struct PyDictView(DictHandle);

#[pymethods]
impl PyDictView {
    #[new]
    fn new(handle: PyRef<'_, PyValueHandle>) -> PyResult<Self> {
        handle
            .0
            .as_dict()
            .map(Self)
            .ok_or_else(|| PyTypeError::new_err("value is not a dictionary"))
    }

    fn _get_field(&self, py: Python<'_>, slot: u32) -> PyResult<Py<PyAny>> {
        value_to_python(py, self.0.field(slot))
    }
}

/// Generic immutable list view used as the base of generated Python collection classes.
#[pyclass(
    name = "_ListView",
    module = "pyavd._rust",
    subclass,
    frozen,
    skip_from_py_object
)]
#[derive(Clone, Debug)]
struct PyListView(ListHandle);

#[pymethods]
impl PyListView {
    #[new]
    fn new(handle: PyRef<'_, PyValueHandle>) -> PyResult<Self> {
        handle
            .0
            .as_list()
            .map(Self)
            .ok_or_else(|| PyTypeError::new_err("value is not a list"))
    }

    fn __len__(&self) -> usize {
        self.0.len()
    }

    fn _get_item(&self, py: Python<'_>, index: isize) -> PyResult<Py<PyAny>> {
        let index = normalize_index(index, self.0.len())?;
        value_to_python(py, self.0.get(index))
    }

    fn _get_by_primary_key(
        &self,
        py: Python<'_>,
        components: &Bound<'_, PyTuple>,
    ) -> PyResult<Py<PyAny>> {
        let components = extract_primary_key(components)?;
        let borrowed = borrow_primary_key(&components);
        value_to_python(py, self.0.get_by_primary_key(&borrowed))
    }

    fn _contains_primary_key(&self, components: &Bound<'_, PyTuple>) -> PyResult<bool> {
        let components = extract_primary_key(components)?;
        Ok(self
            .0
            .get_by_primary_key(&borrow_primary_key(&components))
            .is_some())
    }
}

#[derive(Debug)]
enum OwnedPrimaryKeyValue {
    Bool(bool),
    Int(i64),
    Str(String),
}

fn extract_primary_key(components: &Bound<'_, PyTuple>) -> PyResult<Vec<OwnedPrimaryKeyValue>> {
    components
        .iter()
        .map(|component| {
            if component.is_instance_of::<PyBool>() {
                return component.extract().map(OwnedPrimaryKeyValue::Bool);
            }
            if component.is_instance_of::<PyInt>() {
                return component.extract().map(OwnedPrimaryKeyValue::Int);
            }
            if component.is_instance_of::<PyString>() {
                return component.extract().map(OwnedPrimaryKeyValue::Str);
            }
            Err(PyTypeError::new_err(
                "primary-key components must be bool, int, or str",
            ))
        })
        .collect()
}

fn borrow_primary_key(components: &[OwnedPrimaryKeyValue]) -> Vec<PrimaryKeyValue<'_>> {
    components
        .iter()
        .map(|component| match component {
            OwnedPrimaryKeyValue::Bool(value) => PrimaryKeyValue::Bool(*value),
            OwnedPrimaryKeyValue::Int(value) => PrimaryKeyValue::Int(*value),
            OwnedPrimaryKeyValue::Str(value) => PrimaryKeyValue::Str(value),
        })
        .collect()
}

fn normalize_index(index: isize, len: usize) -> PyResult<usize> {
    let len = isize::try_from(len).map_err(|error| PyIndexError::new_err(error.to_string()))?;
    let normalized = if index < 0 { len + index } else { index };
    if !(0..len).contains(&normalized) {
        return Err(PyIndexError::new_err("list index out of range"));
    }
    usize::try_from(normalized).map_err(|error| PyIndexError::new_err(error.to_string()))
}

fn value_to_python(py: Python<'_>, value: Option<ValueHandle>) -> PyResult<Py<PyAny>> {
    let Some(value) = value else {
        return Ok(py
            .import("pyavd._utils.undefined")?
            .getattr("Undefined")?
            .unbind());
    };
    if value.is_null() {
        return Ok(py.None());
    }
    if let Some(value) = value.as_bool() {
        return Ok(value.into_pyobject(py)?.to_owned().unbind().into_any());
    }
    if let Some(value) = value.as_i64() {
        return Ok(value.into_pyobject(py)?.unbind().into_any());
    }
    if let Some(value) = value.as_str() {
        return Ok(PyString::new(py, value).unbind().into_any());
    }
    Ok(Py::new(py, PyValueHandle(value))?.into_any())
}

/// Validate and coerce one host's AVD Design inputs and publish an rkyv archive when valid.
#[pyfunction]
fn archive_avd_design(
    input_json: &str,
    destination: PathBuf,
    schema_archive: PathBuf,
) -> PyResult<PublicationResult> {
    let schemas = Store::from_file(&schema_archive)
        .map_err(|error| PyRuntimeError::new_err(error.to_string()))?;
    let publication = validate_json_to_archive(
        &schemas,
        "avd_design",
        input_json,
        &destination,
        generated::avd_design::REGISTRY,
    )
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

/// Open one compatible AVD Design data archive and return its private root handle.
#[pyfunction(name = "_open_avd_design_handle")]
fn open_avd_design_handle(archive: PathBuf, schema_archive: PathBuf) -> PyResult<PyValueHandle> {
    let schemas = Store::from_file(&schema_archive)
        .map_err(|error| PyRuntimeError::new_err(error.to_string()))?;
    let data = Arc::new(
        DataStore::from_file(
            &archive,
            schemas.archive_hash(),
            generated::avd_design::REGISTRY,
        )
        .map_err(|error| PyRuntimeError::new_err(error.to_string()))?,
    );
    data.root_as::<generated::avd_design::avd_design::AvdDesign<'_>>()
        .map_err(|error| PyRuntimeError::new_err(error.to_string()))?;
    Ok(PyValueHandle(data.root_handle()))
}

fn to_json(value: &impl serde::Serialize) -> PyResult<String> {
    serde_json::to_string(value).map_err(|error| PyRuntimeError::new_err(error.to_string()))
}

/// AVD Rust extension.
#[pymodule]
mod _rust {
    #[pymodule_export]
    use super::PublicationResult;
    #[pymodule_export]
    use super::PyDictView;
    #[pymodule_export]
    use super::PyListView;
    #[pymodule_export]
    use super::PyOpaqueData;
    #[pymodule_export]
    use super::PyValueHandle;
    #[pymodule_export]
    use super::archive_avd_design;
    #[pymodule_export]
    use super::open_avd_design_handle;
}
