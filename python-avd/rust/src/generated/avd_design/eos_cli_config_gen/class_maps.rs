// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Pbr {
        model item (0) -> pbr::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod pbr {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            model ip("ip", 1) -> item::Ip<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Ip {
                scalar access_group("access_group", 0) -> &'a str;
            }
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Qos {
        model item (0) -> qos::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod qos {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar vlan("vlan", 1) -> &'a str;
            scalar cos("cos", 2) -> &'a str;
            model ip("ip", 3) -> item::Ip<'a>;
            model ipv6("ipv6", 4) -> item::Ipv6<'a>;
            scalar dscp("dscp", 5) -> &'a str;
            scalar ecn("ecn", 6) -> &'a str;
        }
    }

    pub mod item {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Ip {
                scalar access_group("access_group", 0) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Ipv6 {
                scalar access_group("access_group", 0) -> &'a str;
            }
        }
    }
}
