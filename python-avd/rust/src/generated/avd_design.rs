// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

const REGISTRY_HASH: [u8; 32] = [231, 60, 16, 52, 74, 250, 73, 141, 244, 163, 198, 139, 242, 19, 249, 63, 181, 186, 193, 112, 221, 74, 85, 17, 129, 4, 152, 223, 74, 54, 251, 45];
pub mod eos_cli_config_gen;
pub mod avd_design;
pub const REGISTRY: ::validated_data::ModelRegistry = ::validated_data::ModelRegistry {
    root_model: <avd_design::AvdDesign<'static> as ::validated_data::ArchiveModel>::DESCRIPTOR,
    hash: REGISTRY_HASH,
};
