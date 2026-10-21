// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "AVDDesign.GeneralSettings", schema = Dict(1587)))]
pub struct GeneralSettings<'a, Mode> {
    #[data_view(schema = Dict(1582))]
    pub interface_defaults: ::data_views::Field<InterfaceDefaults<'a, Mode>>,
    #[data_view(schema = Dict(1583))]
    pub arp: ::data_views::Field<Arp<'a, Mode>>,
    #[data_view(schema = Bool(7))]
    pub ip_icmp_redirect: ::data_views::Field<bool>,
    #[data_view(schema = Dict(1584))]
    pub dhcp_relay: ::data_views::Field<DhcpRelay<'a, Mode>>,
    #[data_view(schema = List(654))]
    pub suspended_vlans: ::data_views::Field<SuspendedVlans<'a, Mode>>,
    #[data_view(schema = Dict(1586))]
    pub ip_software_forwarding_exceed_action_drop: ::data_views::Field<IpSoftwareForwardingExceedActionDrop<'a, Mode>>,
}

#[::data_views::data_view(python(name = "AVDDesign.GeneralSettings.InterfaceDefaults", schema = Dict(1582)))]
pub struct InterfaceDefaults<'a, Mode> {
    #[data_view(schema = Bool(534))]
    pub ethernet_shutdown: ::data_views::Field<bool>,
}

#[::data_views::data_view(python(name = "AVDDesign.GeneralSettings.Arp", schema = Dict(1583)))]
pub struct Arp<'a, Mode> {
    #[data_view(schema = Dict(78))]
    pub persistent: ::data_views::Field<super::super::eos_config::arp::Persistent<'a, Mode>>,
    #[data_view(schema = Dict(79))]
    pub aging: ::data_views::Field<super::super::eos_config::arp::Aging<'a, Mode>>,
}

pub mod arp {
}

#[::data_views::data_view(python(name = "AVDDesign.GeneralSettings.DhcpRelay", schema = Dict(1584)))]
pub struct DhcpRelay<'a, Mode> {
    #[data_view(schema = Bool(535))]
    pub information_option: ::data_views::Field<bool>,
    #[data_view(schema = Bool(536))]
    pub tunnel_requests_disabled: ::data_views::Field<bool>,
    #[data_view(schema = Bool(537))]
    pub mlag_peerlink_requests_disabled: ::data_views::Field<bool>,
    #[data_view(schema = Bool(38))]
    pub reply_source_address_validation: ::data_views::Field<bool>,
}

#[::data_views::data_view(indexed_list, primary_key(id), python(name = "AVDDesign.GeneralSettings.SuspendedVlans", schema = List(654), item_name = "AVDDesign.GeneralSettings.SuspendedVlansItem"))]
pub struct SuspendedVlans<'a, Mode> (::data_views::Field<suspended_vlans::Item<'a, Mode>>);

pub mod suspended_vlans {

    #[::data_views::data_view(python(name = "AVDDesign.GeneralSettings.SuspendedVlansItemBase", schema = Dict(1585)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Int(587))]
        pub id: ::data_views::Field<i64>,
        #[data_view(schema = Str(3))]
        pub name: ::data_views::Field<&'a str>,
    }
}

#[::data_views::data_view(python(name = "AVDDesign.GeneralSettings.IpSoftwareForwardingExceedActionDrop", schema = Dict(1586)))]
pub struct IpSoftwareForwardingExceedActionDrop<'a, Mode> {
    #[data_view(schema = Bool(1))]
    pub enabled: ::data_views::RequiredValue<bool, Mode>,
    #[data_view(schema = Int(588))]
    pub mtu: ::data_views::RequiredValue<i64, Mode>,
}
