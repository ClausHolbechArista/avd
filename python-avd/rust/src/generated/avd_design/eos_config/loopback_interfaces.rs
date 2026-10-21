// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct LoopbackInterfaces<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view]
pub struct Item<'a, Mode> {
    pub name: ::data_views::Field<&'a str>,
    pub description: ::data_views::Field<&'a str>,
    pub shutdown: ::data_views::Field<bool>,
    pub vrf: ::data_views::Field<&'a str>,
    pub ip_address: ::data_views::Field<&'a str>,
    pub ip_address_secondaries: ::data_views::Field<item::IpAddressSecondaries<'a, Mode>>,
    pub ipv6_enable: ::data_views::Field<bool>,
    pub ipv6_address: ::data_views::Field<&'a str>,
    pub ipv6_addresses: ::data_views::Field<item::Ipv6Addresses<'a, Mode>>,
    pub ipv6_address_auto_config: ::data_views::Field<bool>,
    pub ip_proxy_arp: ::data_views::Field<bool>,
    pub ospf_area: ::data_views::Field<&'a str>,
    pub mpls: ::data_views::Field<item::Mpls<'a, Mode>>,
    pub isis_enable: ::data_views::Field<&'a str>,
    pub isis_bfd: ::data_views::Field<bool>,
    pub isis_passive: ::data_views::Field<bool>,
    pub isis_metric: ::data_views::Field<i64>,
    pub isis_network_point_to_point: ::data_views::Field<bool>,
    pub node_segment: ::data_views::Field<item::NodeSegment<'a, Mode>>,
    pub hardware_forwarding_id: ::data_views::Field<bool>,
    pub eos_cli: ::data_views::Field<&'a str>,
}

pub mod item {

    #[::data_views::data_view(list)]
    pub struct IpAddressSecondaries<'a, Mode> (::data_views::Field<&'a str>);

    #[::data_views::data_view(list)]
    pub struct Ipv6Addresses<'a, Mode> (::data_views::Field<&'a str>);

    #[::data_views::data_view]
    pub struct Mpls<'a, Mode> {
        pub ldp: ::data_views::Field<mpls::Ldp<'a, Mode>>,
    }

    pub mod mpls {

        #[::data_views::data_view]
        pub struct Ldp<'a, Mode> {
            pub interface: ::data_views::Field<bool>,
        }
    }

    #[::data_views::data_view]
    pub struct NodeSegment<'a, Mode> {
        pub ipv4_index: ::data_views::Field<i64>,
        pub ipv6_index: ::data_views::Field<i64>,
    }
}
