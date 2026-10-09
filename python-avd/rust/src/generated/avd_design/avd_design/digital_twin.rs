// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Fabric<'a, Mode> {
    pub act_os_version: ::validated_data::Field<&'a str>,
    pub act_username: ::validated_data::Field<&'a str>,
    pub act_password: ::validated_data::Field<&'a str>,
    pub act_internet_access: ::validated_data::Field<bool>,
    pub act_ensure_eapi_access: ::validated_data::Field<bool>,
}
