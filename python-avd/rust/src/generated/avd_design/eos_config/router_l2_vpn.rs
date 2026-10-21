// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct RouterL2Vpn<'a, Mode> {
    pub arp_learning_bridged: ::data_views::Field<bool>,
    pub arp_proxy: ::data_views::Field<ArpProxy<'a, Mode>>,
    pub arp_selective_install: ::data_views::Field<bool>,
    pub nd_learning_bridged: ::data_views::Field<bool>,
    pub nd_proxy: ::data_views::Field<NdProxy<'a, Mode>>,
    pub nd_rs_flooding_disabled: ::data_views::Field<bool>,
    pub virtual_router_nd_ra_flooding_disabled: ::data_views::Field<bool>,
}

#[::data_views::data_view]
pub struct ArpProxy<'a, Mode> {
    pub prefix_list: ::data_views::Field<&'a str>,
}

#[::data_views::data_view]
pub struct NdProxy<'a, Mode> {
    pub prefix_list: ::data_views::Field<&'a str>,
}
