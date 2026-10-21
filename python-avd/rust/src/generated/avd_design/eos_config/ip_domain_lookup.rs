// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view]
pub struct IpDomainLookup<'a, Mode> {
    pub source_interfaces: ::validated_data::Field<SourceInterfaces<'a, Mode>>,
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct SourceInterfaces<'a, Mode> (::validated_data::Field<source_interfaces::Item<'a, Mode>>);

pub mod source_interfaces {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub vrf: ::validated_data::Field<&'a str>,
    }
}
