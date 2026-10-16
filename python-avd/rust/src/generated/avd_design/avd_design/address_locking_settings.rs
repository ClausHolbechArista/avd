// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(list, python(name = "AVDDesign.AddressLockingSettings.DhcpServerInterfaces", schema = List(562)))]
pub struct DhcpServerInterfaces<'a, Mode> (::validated_data::Field<&'a str>);

#[::validated_data::data_view(list, python(name = "AVDDesign.AddressLockingSettings.DhcpServersIpv4", schema = List(32)))]
pub struct DhcpServersIpv4<'a, Mode> (::validated_data::Field<&'a str>);

#[::validated_data::data_view(list, python(name = "AVDDesign.AddressLockingSettings.Leases", schema = List(34)))]
pub struct Leases<'a, Mode> (::validated_data::Field<leases::Item<'a, Mode>>);

pub mod leases {

    #[::validated_data::data_view(python(name = "AVDDesign.AddressLockingSettings.LeasesItem", schema = Dict(61)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(31))]
        pub ip: ::validated_data::RequiredValue<&'a str, Mode>,
        #[data_view(schema = Str(32))]
        pub mac: ::validated_data::RequiredValue<&'a str, Mode>,
    }
}

#[::validated_data::data_view(python(name = "AVDDesign.AddressLockingSettings.LockedAddress", schema = Dict(62)))]
pub struct LockedAddress<'a, Mode> {
    #[data_view(schema = Bool(11))]
    pub expiration_mac_disabled: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(12))]
    pub ipv4_enforcement_disabled: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(13))]
    pub ipv6_enforcement_disabled: ::validated_data::Field<bool>,
}
