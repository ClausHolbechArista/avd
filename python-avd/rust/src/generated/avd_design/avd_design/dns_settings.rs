// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(list, python(name = "AVDDesign.DnsSettings.DomainList", schema = List(90)))]
pub struct DomainList<'a, Mode> (::validated_data::Field<&'a str>);

#[::validated_data::data_view(list, python(name = "AVDDesign.DnsSettings.Servers", schema = List(645)))]
pub struct Servers<'a, Mode> (::validated_data::Field<servers::Item<'a, Mode>>);

pub mod servers {

    #[::validated_data::data_view(python(name = "AVDDesign.DnsSettings.ServersItem", schema = Dict(1524)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(1382))]
        pub vrf: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(387))]
        pub ip_address: ::validated_data::RequiredValue<&'a str, Mode>,
        #[data_view(schema = Int(179))]
        pub priority: ::validated_data::Field<i64>,
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.DnsSettings.Vrfs", schema = List(646), item_name = "AVDDesign.DnsSettings.VrfsItem"))]
pub struct Vrfs<'a, Mode> (::validated_data::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::validated_data::data_view(python(name = "AVDDesign.DnsSettings.VrfsItemBase", schema = Dict(1525)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(25))]
        pub name: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1383))]
        pub source_interface: ::validated_data::Field<&'a str>,
    }
}
