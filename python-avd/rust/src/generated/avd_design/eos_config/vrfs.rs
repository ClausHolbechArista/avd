// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
    pub description: ::validated_data::Field<&'a str>,
    pub rd: ::validated_data::Field<&'a str>,
    pub ip_routing: ::validated_data::Field<bool>,
    pub ipv6_routing: ::validated_data::Field<bool>,
    pub ip_routing_ipv6_interfaces: ::validated_data::Field<bool>,
    pub metadata: ::validated_data::Field<item::Metadata<'a, Mode>>,
}

pub mod item {

    #[::validated_data::data_view]
    pub struct Metadata<'a, Mode> {
        pub tenants: ::validated_data::Field<metadata::Tenants<'a, Mode>>,
    }

    pub mod metadata {

        #[::validated_data::data_view(list)]
        pub struct Tenants<'a, Mode> (::validated_data::Field<&'a str>);
    }
}
