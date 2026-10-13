// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct ArpProxy<'a, Mode> {
    pub prefix_list: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view]
pub struct NdProxy<'a, Mode> {
    pub prefix_list: ::validated_data::Field<&'a str>,
}
