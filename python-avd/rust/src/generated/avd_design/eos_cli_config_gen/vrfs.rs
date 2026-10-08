// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar name("name", 0) -> &'a str;
        scalar description("description", 1) -> &'a str;
        scalar rd("rd", 2) -> &'a str;
        scalar ip_routing("ip_routing", 3) -> bool;
        scalar ipv6_routing("ipv6_routing", 4) -> bool;
        scalar ip_routing_ipv6_interfaces("ip_routing_ipv6_interfaces", 5) -> bool;
        model metadata("metadata", 6) -> item::Metadata<'a>;
    }
}

pub mod item {

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
