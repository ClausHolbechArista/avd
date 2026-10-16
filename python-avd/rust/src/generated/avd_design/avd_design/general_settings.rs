// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(python(name = "AVDDesign.GeneralSettings.InterfaceDefaults", schema = Dict(1582)))]
pub struct InterfaceDefaults<'a, Mode> {
    #[data_view(schema = Bool(534))]
    pub ethernet_shutdown: ::validated_data::Field<bool>,
}

#[::validated_data::data_view(python(name = "AVDDesign.GeneralSettings.Arp", schema = Dict(1583)))]
pub struct Arp<'a, Mode> {
    #[data_view(schema = Dict(78))]
    pub persistent: ::validated_data::Field<super::super::eos_config::arp::Persistent<'a, Mode>>,
    #[data_view(schema = Dict(79))]
    pub aging: ::validated_data::Field<super::super::eos_config::arp::Aging<'a, Mode>>,
}

pub mod arp {
}

#[::validated_data::data_view(python(name = "AVDDesign.GeneralSettings.DhcpRelay", schema = Dict(1584)))]
pub struct DhcpRelay<'a, Mode> {
    #[data_view(schema = Bool(535))]
    pub information_option: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(536))]
    pub tunnel_requests_disabled: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(537))]
    pub mlag_peerlink_requests_disabled: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(38))]
    pub reply_source_address_validation: ::validated_data::Field<bool>,
}

#[::validated_data::data_view(indexed_list, primary_key(id), python(name = "AVDDesign.GeneralSettings.SuspendedVlans", schema = List(654), item_name = "AVDDesign.GeneralSettings.SuspendedVlansItem"))]
pub struct SuspendedVlans<'a, Mode> (::validated_data::Field<suspended_vlans::Item<'a, Mode>>);

pub mod suspended_vlans {

    #[::validated_data::data_view(python(name = "AVDDesign.GeneralSettings.SuspendedVlansItemBase", schema = Dict(1585)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Int(587))]
        pub id: ::validated_data::Field<i64>,
        #[data_view(schema = Str(3))]
        pub name: ::validated_data::Field<&'a str>,
    }
}

#[::validated_data::data_view(python(name = "AVDDesign.GeneralSettings.IpSoftwareForwardingExceedActionDrop", schema = Dict(1586)))]
pub struct IpSoftwareForwardingExceedActionDrop<'a, Mode> {
    #[data_view(schema = Bool(1))]
    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
    #[data_view(schema = Int(588))]
    pub mtu: ::validated_data::RequiredValue<i64, Mode>,
}
