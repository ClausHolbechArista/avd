// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct ControlPlane<'a, Mode> {
    pub ike_policy_name: ::validated_data::Field<&'a str>,
    pub sa_policy_name: ::validated_data::Field<&'a str>,
    pub profile_name: ::validated_data::Field<&'a str>,
    pub shared_key: ::validated_data::Field<&'a str>,
    pub cleartext_shared_key: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view]
pub struct DataPlane<'a, Mode> {
    pub ike_policy_name: ::validated_data::Field<&'a str>,
    pub sa_policy_name: ::validated_data::Field<&'a str>,
    pub profile_name: ::validated_data::Field<&'a str>,
    pub shared_key: ::validated_data::Field<&'a str>,
    pub cleartext_shared_key: ::validated_data::Field<&'a str>,
}
