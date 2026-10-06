// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! AVD-owned Python extension for validated-data publication and views.

use std::path::PathBuf;

use avdschema::Store;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
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

fn to_json(value: &impl serde::Serialize) -> PyResult<String> {
    serde_json::to_string(value).map_err(|error| PyRuntimeError::new_err(error.to_string()))
}

/// AVD Rust extension.
#[pymodule]
mod _rust {
    #[pymodule_export]
    use super::PublicationResult;
    #[pymodule_export]
    use super::archive_avd_design;
}
