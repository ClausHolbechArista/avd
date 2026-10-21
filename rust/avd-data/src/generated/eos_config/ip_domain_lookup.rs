// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct IpDomainLookup<'a, Mode> {
    pub source_interfaces: ::data_views::Field<SourceInterfaces<'a, Mode>>,
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct SourceInterfaces<'a, Mode> (::data_views::Field<source_interfaces::Item<'a, Mode>>);

pub mod source_interfaces {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub vrf: ::data_views::Field<&'a str>,
    }
}
