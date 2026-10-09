// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

const REGISTRY_HASH: [u8; 32] = [40, 184, 133, 228, 150, 200, 184, 251, 67, 154, 0, 249, 107, 107, 214, 16, 243, 46, 117, 223, 159, 192, 134, 141, 138, 170, 81, 35, 95, 33, 97, 130];
pub mod eos_cli_config_gen;
pub mod avd_design;
pub const REGISTRY: ::validated_data::ModelRegistry = ::validated_data::ModelRegistry {
    root_model: <avd_design::AvdDesign<'static> as ::validated_data::ArchiveModel>::DESCRIPTOR,
    hash: REGISTRY_HASH,
};
