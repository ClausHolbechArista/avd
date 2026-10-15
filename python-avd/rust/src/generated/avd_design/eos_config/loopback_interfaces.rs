// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
    pub description: ::validated_data::Field<&'a str>,
    pub shutdown: ::validated_data::Field<bool>,
    pub vrf: ::validated_data::Field<&'a str>,
    pub ip_address: ::validated_data::Field<&'a str>,
    pub ip_address_secondaries: ::validated_data::Field<item::IpAddressSecondaries<'a, Mode>>,
    pub ipv6_enable: ::validated_data::Field<bool>,
    pub ipv6_address: ::validated_data::Field<&'a str>,
    pub ipv6_addresses: ::validated_data::Field<item::Ipv6Addresses<'a, Mode>>,
    pub ipv6_address_auto_config: ::validated_data::Field<bool>,
    pub ip_proxy_arp: ::validated_data::Field<bool>,
    pub ospf_area: ::validated_data::Field<&'a str>,
    pub mpls: ::validated_data::Field<item::Mpls<'a, Mode>>,
    pub isis_enable: ::validated_data::Field<&'a str>,
    pub isis_bfd: ::validated_data::Field<bool>,
    pub isis_passive: ::validated_data::Field<bool>,
    pub isis_metric: ::validated_data::Field<i64>,
    pub isis_network_point_to_point: ::validated_data::Field<bool>,
    pub node_segment: ::validated_data::Field<item::NodeSegment<'a, Mode>>,
    pub hardware_forwarding_id: ::validated_data::Field<bool>,
    pub eos_cli: ::validated_data::Field<&'a str>,
}

pub mod item {

    #[::validated_data::data_view(list)]
    pub struct IpAddressSecondaries<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(list)]
    pub struct Ipv6Addresses<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view]
    pub struct Mpls<'a, Mode> {
        pub ldp: ::validated_data::Field<mpls::Ldp<'a, Mode>>,
    }

    pub mod mpls {

        #[::validated_data::data_view]
        pub struct Ldp<'a, Mode> {
            pub interface: ::validated_data::Field<bool>,
        }
    }

    #[::validated_data::data_view]
    pub struct NodeSegment<'a, Mode> {
        pub ipv4_index: ::validated_data::Field<i64>,
        pub ipv6_index: ::validated_data::Field<i64>,
    }
}
