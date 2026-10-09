// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
    #[data_view(rename = "type")]
    pub field_type: ::validated_data::RequiredValue<&'a str, Mode>,
    pub fallback_to_system_default: ::validated_data::Field<bool>,
    pub zscaler: ::validated_data::Field<item::Zscaler<'a, Mode>>,
}

pub mod item {

    #[::validated_data::data_view]
    pub struct Zscaler<'a, Mode> {
        pub ipsec_key_salt: ::validated_data::RequiredValue<&'a str, Mode>,
        pub domain_name: ::validated_data::RequiredValue<&'a str, Mode>,
        pub encrypt_traffic: ::validated_data::Field<bool>,
        pub download_bandwidth: ::validated_data::Field<i64>,
        pub upload_bandwidth: ::validated_data::Field<i64>,
        pub firewall: ::validated_data::Field<zscaler::Firewall<'a, Mode>>,
        pub acceptable_use_policy: ::validated_data::Field<bool>,
    }

    pub mod zscaler {

        #[::validated_data::data_view]
        pub struct Firewall<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub ips: ::validated_data::Field<bool>,
        }
    }
}
