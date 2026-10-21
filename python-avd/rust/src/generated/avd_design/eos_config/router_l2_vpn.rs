// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view]
pub struct RouterL2Vpn<'a, Mode> {
    pub arp_learning_bridged: ::validated_data::Field<bool>,
    pub arp_proxy: ::validated_data::Field<ArpProxy<'a, Mode>>,
    pub arp_selective_install: ::validated_data::Field<bool>,
    pub nd_learning_bridged: ::validated_data::Field<bool>,
    pub nd_proxy: ::validated_data::Field<NdProxy<'a, Mode>>,
    pub nd_rs_flooding_disabled: ::validated_data::Field<bool>,
    pub virtual_router_nd_ra_flooding_disabled: ::validated_data::Field<bool>,
}

#[::validated_data::data_view]
pub struct ArpProxy<'a, Mode> {
    pub prefix_list: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view]
pub struct NdProxy<'a, Mode> {
    pub prefix_list: ::validated_data::Field<&'a str>,
}
