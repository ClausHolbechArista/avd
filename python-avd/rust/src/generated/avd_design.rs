// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

const REGISTRY_HASH: [u8; 32] = [151, 17, 249, 66, 215, 189, 118, 112, 67, 86, 164, 87, 192, 177, 210, 164, 164, 183, 88, 184, 255, 210, 87, 172, 226, 74, 111, 39, 79, 88, 254, 20];
/// Exact combined schema archive required for generated schema-node identities.
pub const SCHEMA_HASH: [u8; 32] = [141, 89, 111, 211, 104, 121, 183, 4, 169, 75, 177, 21, 115, 57, 187, 91, 194, 96, 84, 162, 84, 181, 71, 38, 86, 98, 136, 7, 93, 202, 106, 102];
pub mod eos_config;
pub mod avd_design;
pub const REGISTRY: ::validated_data::ModelRegistry = ::validated_data::ModelRegistry {
    root_model: <avd_design::AvdDesign<'static> as ::validated_data::ArchiveModel>::DESCRIPTOR,
    hash: REGISTRY_HASH,
};
