// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct MacBasedAuthentication<'a, Mode> {
    pub delay: ::validated_data::Field<i64>,
    pub hold_period: ::validated_data::Field<i64>,
}

#[::validated_data::data_view]
pub struct RadiusAvPairUsernameFormat<'a, Mode> {
    pub delimiter: ::validated_data::RequiredValue<&'a str, Mode>,
    pub mac_string_case: ::validated_data::RequiredValue<&'a str, Mode>,
}

#[::validated_data::data_view]
pub struct RadiusAvPair<'a, Mode> {
    pub service_type: ::validated_data::Field<bool>,
    pub framed_mtu: ::validated_data::Field<i64>,
    pub lldp: ::validated_data::Field<radius_av_pair::Lldp<'a, Mode>>,
    pub dhcp: ::validated_data::Field<radius_av_pair::Dhcp<'a, Mode>>,
    pub filter_id: ::validated_data::Field<radius_av_pair::FilterId<'a, Mode>>,
}

pub mod radius_av_pair {

    #[::validated_data::data_view]
    pub struct Lldp<'a, Mode> {
        pub system_name: ::validated_data::Field<lldp::SystemName<'a, Mode>>,
        pub system_description: ::validated_data::Field<lldp::SystemDescription<'a, Mode>>,
    }

    pub mod lldp {

        #[::validated_data::data_view]
        pub struct SystemName<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub auth_only: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct SystemDescription<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub auth_only: ::validated_data::Field<bool>,
        }
    }

    #[::validated_data::data_view]
    pub struct Dhcp<'a, Mode> {
        pub hostname: ::validated_data::Field<dhcp::Hostname<'a, Mode>>,
        pub parameter_request_list: ::validated_data::Field<dhcp::ParameterRequestList<'a, Mode>>,
        pub vendor_class_id: ::validated_data::Field<dhcp::VendorClassId<'a, Mode>>,
    }

    pub mod dhcp {

        #[::validated_data::data_view]
        pub struct Hostname<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub auth_only: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct ParameterRequestList<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub auth_only: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct VendorClassId<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub auth_only: ::validated_data::Field<bool>,
        }
    }

    #[::validated_data::data_view]
    pub struct FilterId<'a, Mode> {
        pub delimiter_period: ::validated_data::Field<bool>,
        pub ipv4_ipv6_required: ::validated_data::Field<bool>,
        pub multiple: ::validated_data::Field<bool>,
    }
}

#[::validated_data::data_view]
pub struct Aaa<'a, Mode> {
    pub unresponsive: ::validated_data::Field<aaa::Unresponsive<'a, Mode>>,
    pub accounting_update_interval: ::validated_data::Field<i64>,
}

pub mod aaa {

    #[::validated_data::data_view]
    pub struct Unresponsive<'a, Mode> {
        pub eap_response: ::validated_data::Field<&'a str>,
        pub action: ::validated_data::Field<unresponsive::Action<'a, Mode>>,
        pub phone_action: ::validated_data::Field<unresponsive::PhoneAction<'a, Mode>>,
        pub recovery_action_reauthenticate: ::validated_data::Field<bool>,
    }

    pub mod unresponsive {

        #[::validated_data::data_view]
        pub struct Action<'a, Mode> {
            pub apply_alternate: ::validated_data::Field<bool>,
            pub traffic_allow_vlan: ::validated_data::Field<i64>,
            pub apply_cached_results: ::validated_data::Field<bool>,
            pub cached_results_timeout: ::validated_data::Field<action::CachedResultsTimeout<'a, Mode>>,
            pub traffic_allow: ::validated_data::Field<bool>,
        }

        pub mod action {

            #[::validated_data::data_view]
            pub struct CachedResultsTimeout<'a, Mode> {
                pub time_duration: ::validated_data::RequiredValue<i64, Mode>,
                pub time_duration_unit: ::validated_data::RequiredValue<&'a str, Mode>,
            }
        }

        #[::validated_data::data_view]
        pub struct PhoneAction<'a, Mode> {
            pub apply_cached_results: ::validated_data::Field<bool>,
            pub cached_results_timeout: ::validated_data::Field<phone_action::CachedResultsTimeout<'a, Mode>>,
            pub apply_alternate: ::validated_data::Field<bool>,
            pub traffic_allow: ::validated_data::Field<bool>,
        }

        pub mod phone_action {

            #[::validated_data::data_view]
            pub struct CachedResultsTimeout<'a, Mode> {
                pub time_duration: ::validated_data::RequiredValue<i64, Mode>,
                pub time_duration_unit: ::validated_data::RequiredValue<&'a str, Mode>,
            }
        }
    }
}

#[::validated_data::data_view]
pub struct CaptivePortal<'a, Mode> {
    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
    pub url: ::validated_data::Field<&'a str>,
    pub ssl_profile: ::validated_data::Field<&'a str>,
    pub start_limit_infinite: ::validated_data::Field<bool>,
    pub access_list_ipv4: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view]
pub struct Supplicant<'a, Mode> {
    pub profiles: ::validated_data::Field<supplicant::Profiles<'a, Mode>>,
    pub logging: ::validated_data::Field<bool>,
    pub disconnect_cached_results_timeout: ::validated_data::Field<i64>,
}

pub mod supplicant {

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct Profiles<'a, Mode> (::validated_data::Field<profiles::Item<'a, Mode>>);

    pub mod profiles {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub eap_method: ::validated_data::Field<&'a str>,
            pub identity: ::validated_data::Field<&'a str>,
            pub passphrase_type: ::validated_data::Field<&'a str>,
            pub passphrase: ::validated_data::Field<&'a str>,
            pub ssl_profile: ::validated_data::Field<&'a str>,
        }
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct VlanAssignmentGroups<'a, Mode> (::validated_data::Field<vlan_assignment_groups::Item<'a, Mode>>);

pub mod vlan_assignment_groups {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub members: ::validated_data::RequiredValue<&'a str, Mode>,
    }
}

#[::validated_data::data_view]
pub struct Eapol<'a, Mode> {
    pub vlan_change_logoff_disabled: ::validated_data::Field<bool>,
    pub unresponsive_action_traffic_allow_vlan: ::validated_data::Field<i64>,
}
