// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Authentication<'a, Mode> {
    pub radius_groups: ::validated_data::Field<authentication::RadiusGroups<'a, Mode>>,
}

pub mod authentication {

    #[::validated_data::data_view(list)]
    pub struct RadiusGroups<'a, Mode> (::validated_data::Field<&'a str>);
}

#[::validated_data::data_view]
pub struct Accounting<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub mode: ::validated_data::Field<&'a str>,
    pub radius_groups: ::validated_data::Field<accounting::RadiusGroups<'a, Mode>>,
    pub multicast: ::validated_data::Field<bool>,
    pub syslog: ::validated_data::Field<bool>,
}

pub mod accounting {

    #[::validated_data::data_view(list)]
    pub struct RadiusGroups<'a, Mode> (::validated_data::Field<&'a str>);
}

#[::validated_data::data_view]
pub struct DynamicAuthorization<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub additional_groups: ::validated_data::Field<dynamic_authorization::AdditionalGroups<'a, Mode>>,
}

pub mod dynamic_authorization {

    #[::validated_data::data_view(list)]
    pub struct AdditionalGroups<'a, Mode> (::validated_data::Field<&'a str>);
}

#[::validated_data::data_view]
pub struct MacBasedAuthentication<'a, Mode> {
    pub username_format: ::validated_data::Field<mac_based_authentication::UsernameFormat<'a, Mode>>,
    pub delay: ::validated_data::Field<i64>,
    pub hold_period: ::validated_data::Field<i64>,
}

pub mod mac_based_authentication {

    #[::validated_data::data_view]
    pub struct UsernameFormat<'a, Mode> {
        pub delimiter: ::validated_data::RequiredValue<&'a str, Mode>,
        pub letter_case: ::validated_data::RequiredValue<&'a str, Mode>,
    }
}

#[::validated_data::data_view]
pub struct RadiusAvPairs<'a, Mode> {
    pub service_type: ::validated_data::Field<bool>,
    pub framed_mtu: ::validated_data::Field<i64>,
}

#[::validated_data::data_view]
pub struct DeviceProfiling<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub dhcp: ::validated_data::Field<device_profiling::Dhcp<'a, Mode>>,
    pub lldp: ::validated_data::Field<device_profiling::Lldp<'a, Mode>>,
}

pub mod device_profiling {

    #[::validated_data::data_view]
    pub struct Dhcp<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub hostname: ::validated_data::Field<dhcp::Hostname<'a, Mode>>,
        pub parameter_request_list: ::validated_data::Field<dhcp::ParameterRequestList<'a, Mode>>,
        pub vendor_class_id: ::validated_data::Field<dhcp::VendorClassId<'a, Mode>>,
    }

    pub mod dhcp {

        #[::validated_data::data_view]
        pub struct Hostname<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub auth_only: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct ParameterRequestList<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub auth_only: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct VendorClassId<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub auth_only: ::validated_data::Field<bool>,
        }
    }

    #[::validated_data::data_view]
    pub struct Lldp<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub system_name: ::validated_data::Field<lldp::SystemName<'a, Mode>>,
        pub system_description: ::validated_data::Field<lldp::SystemDescription<'a, Mode>>,
    }

    pub mod lldp {

        #[::validated_data::data_view]
        pub struct SystemName<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub auth_only: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct SystemDescription<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub auth_only: ::validated_data::Field<bool>,
        }
    }
}

#[::validated_data::data_view]
pub struct WebAuthentication<'a, Mode> {
    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
    pub ipv4_acl: ::validated_data::Field<&'a str>,
    pub url: ::validated_data::Field<&'a str>,
    pub ssl_profile: ::validated_data::Field<&'a str>,
    pub start_limit_infinite: ::validated_data::Field<bool>,
}
