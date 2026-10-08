// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

const REGISTRY_HASH: [u8; 32] = [39, 142, 243, 157, 197, 241, 207, 98, 55, 223, 157, 157, 96, 98, 94, 96, 81, 234, 142, 146, 6, 107, 252, 72, 53, 157, 77, 135, 121, 99, 227, 217];
pub mod eos_cli_config_gen;
pub mod avd_design;
pub const REGISTRY: ::validation::archive::ModelRegistry = ::validation::archive::ModelRegistry {
    root_model: <avd_design::AvdDesign<'static> as ::validation::archive::ArchiveModel>::DESCRIPTOR,
    hash: REGISTRY_HASH,
};
