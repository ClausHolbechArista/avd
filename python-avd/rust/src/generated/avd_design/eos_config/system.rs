// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct ControlPlane<'a, Mode> {
    pub tcp_mss: ::validated_data::Field<control_plane::TcpMss<'a, Mode>>,
    pub ipv4_access_group_ingress_default: ::validated_data::Field<&'a str>,
    pub ipv4_access_groups: ::validated_data::Field<control_plane::Ipv4AccessGroups<'a, Mode>>,
    pub ipv6_access_group_ingress_default: ::validated_data::Field<&'a str>,
    pub ipv6_access_groups: ::validated_data::Field<control_plane::Ipv6AccessGroups<'a, Mode>>,
}

pub mod control_plane {

    #[::validated_data::data_view]
    pub struct TcpMss<'a, Mode> {
        pub ipv4: ::validated_data::Field<i64>,
        pub ipv6: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(list)]
    pub struct Ipv4AccessGroups<'a, Mode> (::validated_data::Field<ipv4_access_groups::Item<'a, Mode>>);

    pub mod ipv4_access_groups {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub acl_name: ::validated_data::RequiredValue<&'a str, Mode>,
            pub vrf: ::validated_data::Field<&'a str>,
        }
    }

    #[::validated_data::data_view(list)]
    pub struct Ipv6AccessGroups<'a, Mode> (::validated_data::Field<ipv6_access_groups::Item<'a, Mode>>);

    pub mod ipv6_access_groups {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub acl_name: ::validated_data::RequiredValue<&'a str, Mode>,
            pub vrf: ::validated_data::Field<&'a str>,
        }
    }
}

#[::validated_data::data_view]
pub struct L1<'a, Mode> {
    pub unsupported_speed_action: ::validated_data::Field<&'a str>,
    pub unsupported_error_correction_action: ::validated_data::Field<&'a str>,
}
