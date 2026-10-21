// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct System<'a, Mode> {
    pub control_plane: ::data_views::Field<ControlPlane<'a, Mode>>,
    pub l1: ::data_views::Field<L1<'a, Mode>>,
    pub mac_address: ::data_views::Field<&'a str>,
}

#[::data_views::data_view]
pub struct ControlPlane<'a, Mode> {
    pub tcp_mss: ::data_views::Field<control_plane::TcpMss<'a, Mode>>,
    pub ipv4_access_group_ingress_default: ::data_views::Field<&'a str>,
    pub ipv4_access_groups: ::data_views::Field<control_plane::Ipv4AccessGroups<'a, Mode>>,
    pub ipv6_access_group_ingress_default: ::data_views::Field<&'a str>,
    pub ipv6_access_groups: ::data_views::Field<control_plane::Ipv6AccessGroups<'a, Mode>>,
}

pub mod control_plane {

    #[::data_views::data_view]
    pub struct TcpMss<'a, Mode> {
        pub ipv4: ::data_views::Field<i64>,
        pub ipv6: ::data_views::Field<i64>,
    }

    #[::data_views::data_view(list)]
    pub struct Ipv4AccessGroups<'a, Mode> (::data_views::Field<ipv4_access_groups::Item<'a, Mode>>);

    pub mod ipv4_access_groups {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub acl_name: ::data_views::RequiredValue<&'a str, Mode>,
            pub vrf: ::data_views::Field<&'a str>,
        }
    }

    #[::data_views::data_view(list)]
    pub struct Ipv6AccessGroups<'a, Mode> (::data_views::Field<ipv6_access_groups::Item<'a, Mode>>);

    pub mod ipv6_access_groups {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub acl_name: ::data_views::RequiredValue<&'a str, Mode>,
            pub vrf: ::data_views::Field<&'a str>,
        }
    }
}

#[::data_views::data_view]
pub struct L1<'a, Mode> {
    pub unsupported_speed_action: ::data_views::Field<&'a str>,
    pub unsupported_error_correction_action: ::data_views::Field<&'a str>,
}
