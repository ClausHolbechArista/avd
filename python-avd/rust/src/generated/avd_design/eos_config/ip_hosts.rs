// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(indexed_list, primary_key(hostname), python(name = "EOSConfig.IpHosts", schema = List(173), item_name = "EOSConfig.IpHostsItem"))]
pub struct IpHosts<'a, Mode> (::validated_data::Field<Item<'a, Mode>>);

#[::validated_data::data_view(python(name = "EOSConfig.IpHostsItemBase", schema = Dict(424)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(3))]
    pub hostname: ::validated_data::Field<&'a str>,
    #[data_view(schema = List(172))]
    pub ipv4_addresses: ::validated_data::RequiredValue<item::Ipv4Addresses<'a, Mode>, Mode>,
}

pub mod item {

    #[::validated_data::data_view(list, python(name = "EOSConfig.IpHostsItemBase.Ipv4Addresses", schema = List(172)))]
    pub struct Ipv4Addresses<'a, Mode> (::validated_data::Field<&'a str>);
}
