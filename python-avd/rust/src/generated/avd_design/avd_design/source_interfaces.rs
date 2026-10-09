// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct HttpClient<'a, Mode> {
    pub mgmt_interface: ::validated_data::Field<bool>,
    pub inband_mgmt_interface: ::validated_data::Field<bool>,
}

#[::validated_data::data_view]
pub struct SshClient<'a, Mode> {
    pub mgmt_interface: ::validated_data::Field<bool>,
    pub inband_mgmt_interface: ::validated_data::Field<bool>,
}
