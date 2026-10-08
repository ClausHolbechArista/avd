// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        model types("types", 0) -> item::Types<'a>;
        model platforms("platforms", 1) -> item::Platforms<'a>;
        model uplink_interfaces("uplink_interfaces", 2) -> item::UplinkInterfaces<'a>;
        model mlag_interfaces("mlag_interfaces", 3) -> item::MlagInterfaces<'a>;
        scalar mlag_interfaces_speed("mlag_interfaces_speed", 4) -> &'a str;
        model downlink_interfaces("downlink_interfaces", 5) -> item::DownlinkInterfaces<'a>;
        scalar uplink_interface_speed("uplink_interface_speed", 6) -> &'a str;
    }
}

pub mod item {

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Types {
            scalar item (0) -> &'a str;
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Platforms {
            scalar item (0) -> &'a str;
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct UplinkInterfaces {
            scalar item (0) -> &'a str;
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct MlagInterfaces {
            scalar item (0) -> &'a str;
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct DownlinkInterfaces {
            scalar item (0) -> &'a str;
        }
    }
}
