// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar id("id", 0) -> i64;
        scalar name("name", 1) -> &'a str;
        scalar state("state", 2) -> &'a str;
        model address_locking("address_locking", 3) -> item::AddressLocking<'a>;
        model trunk_groups("trunk_groups", 4) -> item::TrunkGroups<'a>;
        model e_tree("e_tree", 5) -> item::ETree<'a>;
        model private_vlan("private_vlan", 6) -> item::PrivateVlan<'a>;
        model metadata("metadata", 7) -> item::Metadata<'a>;
    }
}

pub mod item {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct AddressLocking {
            model address_family("address_family", 0) -> address_locking::AddressFamily<'a>;
            scalar ipv4_enforcement_disabled("ipv4_enforcement_disabled", 1) -> bool;
        }
    }

    pub mod address_locking {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct AddressFamily {
                scalar ipv4("ipv4", 0) -> bool;
                scalar ipv6("ipv6", 1) -> bool;
            }
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct TrunkGroups {
            scalar item (0) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct ETree {
            scalar leaf_role("leaf_role", 0) -> bool;
            scalar remote_leaf_host_drop("remote_leaf_host_drop", 1) -> bool;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct PrivateVlan {
            scalar field_type("type", 0) -> &'a str;
            scalar primary_vlan("primary_vlan", 1) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Metadata {
            model tenants("tenants", 0) -> metadata::Tenants<'a>;
        }
    }

    pub mod metadata {

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Tenants {
                scalar item (0) -> &'a str;
            }
        }
    }
}
