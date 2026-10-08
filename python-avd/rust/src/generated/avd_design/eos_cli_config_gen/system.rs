// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct ControlPlane {
        model tcp_mss("tcp_mss", 0) -> control_plane::TcpMss<'a>;
        scalar ipv4_access_group_ingress_default("ipv4_access_group_ingress_default", 1) -> &'a str;
        model ipv4_access_groups("ipv4_access_groups", 2) -> control_plane::Ipv4AccessGroups<'a>;
        scalar ipv6_access_group_ingress_default("ipv6_access_group_ingress_default", 3) -> &'a str;
        model ipv6_access_groups("ipv6_access_groups", 4) -> control_plane::Ipv6AccessGroups<'a>;
    }
}

pub mod control_plane {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct TcpMss {
            scalar ipv4("ipv4", 0) -> i64;
            scalar ipv6("ipv6", 1) -> i64;
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Ipv4AccessGroups {
            model item (0) -> ipv4_access_groups::Item<'a>;
        }
    }

    pub mod ipv4_access_groups {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar acl_name("acl_name", 0) -> &'a str;
                scalar vrf("vrf", 1) -> &'a str;
            }
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Ipv6AccessGroups {
            model item (0) -> ipv6_access_groups::Item<'a>;
        }
    }

    pub mod ipv6_access_groups {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar acl_name("acl_name", 0) -> &'a str;
                scalar vrf("vrf", 1) -> &'a str;
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct L1 {
        scalar unsupported_speed_action("unsupported_speed_action", 0) -> &'a str;
        scalar unsupported_error_correction_action("unsupported_error_correction_action", 1) -> &'a str;
    }
}
