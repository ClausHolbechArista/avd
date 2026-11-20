// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct AddressLocking<'a, Mode> {
    pub dhcp_servers_ipv4: ::data_views::Field<DhcpServersIpv4<'a, Mode>>,
    pub dhcp_server_interfaces: ::data_views::Field<DhcpServerInterfaces<'a, Mode>>,
    pub disabled: ::data_views::Field<bool>,
    pub leases: ::data_views::Field<Leases<'a, Mode>>,
    pub local_interface: ::data_views::Field<&'a str>,
    pub locked_address: ::data_views::Field<LockedAddress<'a, Mode>>,
}

#[::data_views::data_view(list)]
pub struct DhcpServersIpv4<'a, Mode> (::data_views::Field<&'a str>);

#[::data_views::data_view(list)]
pub struct DhcpServerInterfaces<'a, Mode> (::data_views::Field<&'a str>);

#[::data_views::data_view(list)]
pub struct Leases<'a, Mode> (::data_views::Field<leases::Item<'a, Mode>>);

pub mod leases {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub ip: ::data_views::RequiredValue<&'a str, Mode>,
        pub mac: ::data_views::RequiredValue<&'a str, Mode>,
    }
}

#[::data_views::data_view]
pub struct LockedAddress<'a, Mode> {
    pub expiration_mac_disabled: ::data_views::Field<bool>,
    pub ipv4_enforcement_disabled: ::data_views::Field<bool>,
    pub ipv6_enforcement_disabled: ::data_views::Field<bool>,
}
