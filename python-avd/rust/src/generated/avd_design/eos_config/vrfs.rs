// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Vrfs<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view]
pub struct Item<'a, Mode> {
    pub name: ::data_views::Field<&'a str>,
    pub description: ::data_views::Field<&'a str>,
    pub rd: ::data_views::Field<&'a str>,
    pub ip_routing: ::data_views::Field<bool>,
    pub ipv6_routing: ::data_views::Field<bool>,
    pub ip_routing_ipv6_interfaces: ::data_views::Field<bool>,
    pub metadata: ::data_views::Field<item::Metadata<'a, Mode>>,
}

pub mod item {

    #[::data_views::data_view]
    pub struct Metadata<'a, Mode> {
        pub tenants: ::data_views::Field<metadata::Tenants<'a, Mode>>,
    }

    pub mod metadata {

        #[::data_views::data_view(list)]
        pub struct Tenants<'a, Mode> (::data_views::Field<&'a str>);
    }
}
