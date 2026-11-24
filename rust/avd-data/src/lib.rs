// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! AVD data models and schema-compatible validated-data publication and access.
//!
//! Generated models expose borrowed Rust views over immutable data. Validation and coercion
//! precede publication; opening retains the data and schema mappings for subsequent access.
//! The `python` feature adds binding metadata to the same models, without changing their
//! Rust accessors, descriptors, or archive representation. Default builds require no Python.
//!
//! [`open_avd_design`] checks archive integrity and compatibility. [`AvdDesignData::root`]
//! additionally checks the generated structural and requiredness contracts before exposing
//! typed accessors. Borrow the root once per operation; descendants reuse its checked backing.

#![deny(missing_docs)]

use std::path::Path;
use std::sync::Arc;

use avdschema::Store;
use data_views::DataStore;

#[allow(
    missing_docs,
    dead_code,
    non_snake_case,
    reason = "Generated schema models preserve schema spelling and expose the complete model catalog"
)]
#[rustfmt::skip]
mod generated;

pub use generated::{REGISTRY, SCHEMA_HASH, avd_design, eos_config};
pub use validation::archive::ArchivePublication;

/// Failure to load matching schemas, publish inputs, or access an AVD data archive.
#[derive(Debug, derive_more::Display, derive_more::From)]
pub enum Error {
    /// The schema store could not be opened or its archive was invalid.
    #[display("{_0}")]
    Schema(avdschema::StoreError),
    /// The data archive could not be opened or satisfy the generated model contract.
    #[display("{_0}")]
    Archive(data_views::ArchiveError),
    /// Validation setup or archive publication failed.
    #[display("{_0}")]
    Publication(validation::archive::PublicationError),
    /// Generated schema-node identities belong to a different combined schema store.
    #[display("schema archive does not match the generated data models")]
    SchemaMismatch {
        /// Hash of the combined schema used to generate these models.
        expected: [u8; 32],
        /// Hash of the combined schema supplied by the caller.
        found: [u8; 32],
    },
}

impl std::error::Error for Error {}

/// Owners of compatible AVD Design data and schema mappings.
///
/// Views borrow these owners; owned handles can retain them across API boundaries without
/// copying the underlying collections. The schema mapping also supports Python default lookup.
#[derive(Debug)]
pub struct AvdDesignData {
    schemas: Arc<Store>,
    data: Arc<DataStore>,
}

impl AvdDesignData {
    /// Check the root's field shapes, primary keys and requiredness, returning a borrowed view.
    ///
    /// This walk runs for each root request. Retain the returned view while traversing children
    /// to reuse its guarantees without repeating the check.
    pub fn root(&self) -> Result<avd_design::AvdDesign<'_>, Error> {
        Ok(self.data.root_as::<avd_design::AvdDesign<'_>>()?)
    }

    /// Shared owner for raw immutable access and owned child handles.
    pub fn data(&self) -> &Arc<DataStore> {
        &self.data
    }

    /// Shared owner of the exact schema store paired with the generated models.
    pub fn schemas(&self) -> &Arc<Store> {
        &self.schemas
    }
}

/// Validate and coerce one host's AVD Design inputs, publishing only when validation succeeds.
///
/// Input and validation diagnostics remain structured in the result. Invalid inputs are a normal
/// result with `published == false`; schema-loading and publication failures return an error.
pub fn archive_avd_design(
    input_json: &str,
    destination: &Path,
    schema_archive: &Path,
) -> Result<ArchivePublication, Error> {
    let schemas = Store::from_file(schema_archive)?;
    check_schema(&schemas)?;
    Ok(validation::archive::validate_json_to_archive(
        &schemas,
        "avd_design",
        input_json,
        destination,
        REGISTRY,
    )?)
}

/// Open matching data and schema archives without materializing their collections.
///
/// Opening checks file integrity, the generated schema fingerprint, the model registry and
/// validation policy. Call [`AvdDesignData::root`] to check and obtain the typed root view.
/// Mapped files must remain immutable; publication replaces the destination atomically rather
/// than modifying an inode retained by existing readers.
pub fn open_avd_design(archive: &Path, schema_archive: &Path) -> Result<AvdDesignData, Error> {
    let schemas = Arc::new(Store::from_file(schema_archive)?);
    check_schema(&schemas)?;
    let data = Arc::new(DataStore::from_file(
        archive,
        schemas.archive_hash(),
        REGISTRY,
    )?);
    Ok(AvdDesignData { schemas, data })
}

fn check_schema(schemas: &Store) -> Result<(), Error> {
    let found = schemas.archive_hash();
    if found != SCHEMA_HASH {
        return Err(Error::SchemaMismatch {
            expected: SCHEMA_HASH,
            found,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_mismatch_retains_both_fingerprints() {
        let schemas = Store::from_json(r#"{"fixture":{"type":"dict"}}"#).expect("valid fixture");
        match check_schema(&schemas) {
            Err(Error::SchemaMismatch { expected, found }) => {
                assert_eq!(expected, SCHEMA_HASH);
                assert_eq!(found, schemas.archive_hash());
                assert_ne!(expected, found);
            }
            other => panic!("expected schema mismatch, got {other:?}"),
        }
    }
}
