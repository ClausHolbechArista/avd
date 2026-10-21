// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "AVDDesign.AddressLockingSettings", schema = Dict(1386)))]
pub struct AddressLockingSettings<'a, Mode> {
    #[data_view(schema = Str(1102))]
    pub local_interface: ::data_views::Field<&'a str>,
    #[data_view(schema = List(562))]
    pub dhcp_server_interfaces: ::data_views::Field<DhcpServerInterfaces<'a, Mode>>,
    #[data_view(schema = List(32))]
    pub dhcp_servers_ipv4: ::data_views::Field<DhcpServersIpv4<'a, Mode>>,
    #[data_view(schema = Bool(10))]
    pub disabled: ::data_views::Field<bool>,
    #[data_view(schema = List(34))]
    pub leases: ::data_views::Field<Leases<'a, Mode>>,
    #[data_view(schema = Dict(62))]
    pub locked_address: ::data_views::Field<LockedAddress<'a, Mode>>,
}

#[::data_views::data_view(list, python(name = "AVDDesign.AddressLockingSettings.DhcpServerInterfaces", schema = List(562)))]
pub struct DhcpServerInterfaces<'a, Mode> (::data_views::Field<&'a str>);

#[::data_views::data_view(list, python(name = "AVDDesign.AddressLockingSettings.DhcpServersIpv4", schema = List(32)))]
pub struct DhcpServersIpv4<'a, Mode> (::data_views::Field<&'a str>);

#[::data_views::data_view(list, python(name = "AVDDesign.AddressLockingSettings.Leases", schema = List(34)))]
pub struct Leases<'a, Mode> (::data_views::Field<leases::Item<'a, Mode>>);

pub mod leases {

    #[::data_views::data_view(python(name = "AVDDesign.AddressLockingSettings.LeasesItem", schema = Dict(61)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(31))]
        pub ip: ::data_views::RequiredValue<&'a str, Mode>,
        #[data_view(schema = Str(32))]
        pub mac: ::data_views::RequiredValue<&'a str, Mode>,
    }
}

#[::data_views::data_view(python(name = "AVDDesign.AddressLockingSettings.LockedAddress", schema = Dict(62)))]
pub struct LockedAddress<'a, Mode> {
    #[data_view(schema = Bool(11))]
    pub expiration_mac_disabled: ::data_views::Field<bool>,
    #[data_view(schema = Bool(12))]
    pub ipv4_enforcement_disabled: ::data_views::Field<bool>,
    #[data_view(schema = Bool(13))]
    pub ipv6_enforcement_disabled: ::data_views::Field<bool>,
}
