// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(list, python(name = "AVDDesign.DnsSettings.DomainList"))]
pub struct DomainList<'a, Mode> (::validated_data::Field<&'a str>);

#[::validated_data::data_view(list, python(name = "AVDDesign.DnsSettings.Servers"))]
pub struct Servers<'a, Mode> (::validated_data::Field<servers::Item<'a, Mode>>);

pub mod servers {

    #[::validated_data::data_view(python(name = "AVDDesign.DnsSettings.ServersItem"))]
    pub struct Item<'a, Mode> {
        pub vrf: ::validated_data::Field<&'a str>,
        pub ip_address: ::validated_data::RequiredValue<&'a str, Mode>,
        pub priority: ::validated_data::Field<i64>,
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.DnsSettings.Vrfs", item_name = "AVDDesign.DnsSettings.VrfsItem"))]
pub struct Vrfs<'a, Mode> (::validated_data::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::validated_data::data_view(python(name = "AVDDesign.DnsSettings.VrfsItemBase"))]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub source_interface: ::validated_data::Field<&'a str>,
    }
}
