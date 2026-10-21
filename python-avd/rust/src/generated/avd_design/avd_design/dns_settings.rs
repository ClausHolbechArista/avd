// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "AVDDesign.DnsSettings", schema = Dict(1526)))]
pub struct DnsSettings<'a, Mode> {
    #[data_view(schema = Str(1381))]
    pub domain: ::data_views::Field<&'a str>,
    #[data_view(schema = List(90))]
    pub domain_list: ::data_views::Field<DomainList<'a, Mode>>,
    #[data_view(schema = List(645))]
    pub servers: ::data_views::RequiredValue<Servers<'a, Mode>, Mode>,
    #[data_view(schema = List(646))]
    pub vrfs: ::data_views::Field<Vrfs<'a, Mode>>,
    #[data_view(schema = Bool(492))]
    pub set_source_interfaces: ::data_views::Field<bool>,
    #[data_view(schema = List(173))]
    pub ip_hosts: ::data_views::Field<super::super::eos_config::IpHosts<'a, Mode>>,
}

#[::data_views::data_view(list, python(name = "AVDDesign.DnsSettings.DomainList", schema = List(90)))]
pub struct DomainList<'a, Mode> (::data_views::Field<&'a str>);

#[::data_views::data_view(list, python(name = "AVDDesign.DnsSettings.Servers", schema = List(645)))]
pub struct Servers<'a, Mode> (::data_views::Field<servers::Item<'a, Mode>>);

pub mod servers {

    #[::data_views::data_view(python(name = "AVDDesign.DnsSettings.ServersItem", schema = Dict(1524)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(1382))]
        pub vrf: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(387))]
        pub ip_address: ::data_views::RequiredValue<&'a str, Mode>,
        #[data_view(schema = Int(179))]
        pub priority: ::data_views::Field<i64>,
    }
}

#[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.DnsSettings.Vrfs", schema = List(646), item_name = "AVDDesign.DnsSettings.VrfsItem"))]
pub struct Vrfs<'a, Mode> (::data_views::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::data_views::data_view(python(name = "AVDDesign.DnsSettings.VrfsItemBase", schema = Dict(1525)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(25))]
        pub name: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1383))]
        pub source_interface: ::data_views::Field<&'a str>,
    }
}
