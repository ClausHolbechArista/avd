// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct Dot1x<'a, Mode> {
    pub system_auth_control: ::data_views::Field<bool>,
    pub protocol_lldp_bypass: ::data_views::Field<bool>,
    pub protocol_bpdu_bypass: ::data_views::Field<bool>,
    pub dynamic_authorization: ::data_views::Field<bool>,
    pub statistics_packets_dropped: ::data_views::Field<bool>,
    pub mac_based_authentication: ::data_views::Field<MacBasedAuthentication<'a, Mode>>,
    pub radius_av_pair_username_format: ::data_views::Field<RadiusAvPairUsernameFormat<'a, Mode>>,
    pub radius_av_pair: ::data_views::Field<RadiusAvPair<'a, Mode>>,
    pub aaa: ::data_views::Field<Aaa<'a, Mode>>,
    pub captive_portal: ::data_views::Field<CaptivePortal<'a, Mode>>,
    pub supplicant: ::data_views::Field<Supplicant<'a, Mode>>,
    pub vlan_assignment_groups: ::data_views::Field<VlanAssignmentGroups<'a, Mode>>,
    pub eapol: ::data_views::Field<Eapol<'a, Mode>>,
}

#[::data_views::data_view]
pub struct MacBasedAuthentication<'a, Mode> {
    pub delay: ::data_views::Field<i64>,
    pub hold_period: ::data_views::Field<i64>,
}

#[::data_views::data_view]
pub struct RadiusAvPairUsernameFormat<'a, Mode> {
    pub delimiter: ::data_views::RequiredValue<&'a str, Mode>,
    pub mac_string_case: ::data_views::RequiredValue<&'a str, Mode>,
}

#[::data_views::data_view]
pub struct RadiusAvPair<'a, Mode> {
    pub service_type: ::data_views::Field<bool>,
    pub framed_mtu: ::data_views::Field<i64>,
    pub lldp: ::data_views::Field<radius_av_pair::Lldp<'a, Mode>>,
    pub dhcp: ::data_views::Field<radius_av_pair::Dhcp<'a, Mode>>,
    pub filter_id: ::data_views::Field<radius_av_pair::FilterId<'a, Mode>>,
}

pub mod radius_av_pair {

    #[::data_views::data_view]
    pub struct Lldp<'a, Mode> {
        pub system_name: ::data_views::Field<lldp::SystemName<'a, Mode>>,
        pub system_description: ::data_views::Field<lldp::SystemDescription<'a, Mode>>,
    }

    pub mod lldp {

        #[::data_views::data_view]
        pub struct SystemName<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub auth_only: ::data_views::Field<bool>,
        }

        #[::data_views::data_view]
        pub struct SystemDescription<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub auth_only: ::data_views::Field<bool>,
        }
    }

    #[::data_views::data_view]
    pub struct Dhcp<'a, Mode> {
        pub hostname: ::data_views::Field<dhcp::Hostname<'a, Mode>>,
        pub parameter_request_list: ::data_views::Field<dhcp::ParameterRequestList<'a, Mode>>,
        pub vendor_class_id: ::data_views::Field<dhcp::VendorClassId<'a, Mode>>,
    }

    pub mod dhcp {

        #[::data_views::data_view]
        pub struct Hostname<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub auth_only: ::data_views::Field<bool>,
        }

        #[::data_views::data_view]
        pub struct ParameterRequestList<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub auth_only: ::data_views::Field<bool>,
        }

        #[::data_views::data_view]
        pub struct VendorClassId<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub auth_only: ::data_views::Field<bool>,
        }
    }

    #[::data_views::data_view]
    pub struct FilterId<'a, Mode> {
        pub delimiter_period: ::data_views::Field<bool>,
        pub ipv4_ipv6_required: ::data_views::Field<bool>,
        pub multiple: ::data_views::Field<bool>,
    }
}

#[::data_views::data_view]
pub struct Aaa<'a, Mode> {
    pub unresponsive: ::data_views::Field<aaa::Unresponsive<'a, Mode>>,
    pub accounting_update_interval: ::data_views::Field<i64>,
}

pub mod aaa {

    #[::data_views::data_view]
    pub struct Unresponsive<'a, Mode> {
        pub eap_response: ::data_views::Field<&'a str>,
        pub action: ::data_views::Field<unresponsive::Action<'a, Mode>>,
        pub phone_action: ::data_views::Field<unresponsive::PhoneAction<'a, Mode>>,
        pub recovery_action_reauthenticate: ::data_views::Field<bool>,
    }

    pub mod unresponsive {

        #[::data_views::data_view]
        pub struct Action<'a, Mode> {
            pub apply_alternate: ::data_views::Field<bool>,
            pub traffic_allow_vlan: ::data_views::Field<i64>,
            pub apply_cached_results: ::data_views::Field<bool>,
            pub cached_results_timeout: ::data_views::Field<action::CachedResultsTimeout<'a, Mode>>,
            pub traffic_allow: ::data_views::Field<bool>,
        }

        pub mod action {

            #[::data_views::data_view]
            pub struct CachedResultsTimeout<'a, Mode> {
                pub time_duration: ::data_views::RequiredValue<i64, Mode>,
                pub time_duration_unit: ::data_views::RequiredValue<&'a str, Mode>,
            }
        }

        #[::data_views::data_view(python(name = "EOSConfig.Dot1x.Aaa.Unresponsive.PhoneAction", schema = Dict(153)))]
        pub struct PhoneAction<'a, Mode> {
            #[data_view(schema = Bool(48))]
            pub apply_cached_results: ::data_views::Field<bool>,
            #[data_view(schema = Dict(151))]
            pub cached_results_timeout: ::data_views::Field<phone_action::CachedResultsTimeout<'a, Mode>>,
            #[data_view(schema = Bool(50))]
            pub apply_alternate: ::data_views::Field<bool>,
            #[data_view(schema = Bool(49))]
            pub traffic_allow: ::data_views::Field<bool>,
        }

        pub mod phone_action {

            #[::data_views::data_view(python(name = "EOSConfig.Dot1x.Aaa.Unresponsive.PhoneAction.CachedResultsTimeout", schema = Dict(151)))]
            pub struct CachedResultsTimeout<'a, Mode> {
                #[data_view(schema = Int(27))]
                pub time_duration: ::data_views::RequiredValue<i64, Mode>,
                #[data_view(schema = Str(155))]
                pub time_duration_unit: ::data_views::RequiredValue<&'a str, Mode>,
            }
        }
    }
}

#[::data_views::data_view]
pub struct CaptivePortal<'a, Mode> {
    pub enabled: ::data_views::RequiredValue<bool, Mode>,
    pub url: ::data_views::Field<&'a str>,
    pub ssl_profile: ::data_views::Field<&'a str>,
    pub start_limit_infinite: ::data_views::Field<bool>,
    pub access_list_ipv4: ::data_views::Field<&'a str>,
}

#[::data_views::data_view]
pub struct Supplicant<'a, Mode> {
    pub profiles: ::data_views::Field<supplicant::Profiles<'a, Mode>>,
    pub logging: ::data_views::Field<bool>,
    pub disconnect_cached_results_timeout: ::data_views::Field<i64>,
}

pub mod supplicant {

    #[::data_views::data_view(indexed_list, primary_key(name))]
    pub struct Profiles<'a, Mode> (::data_views::Field<profiles::Item<'a, Mode>>);

    pub mod profiles {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::data_views::Field<&'a str>,
            pub eap_method: ::data_views::Field<&'a str>,
            pub identity: ::data_views::Field<&'a str>,
            pub passphrase_type: ::data_views::Field<&'a str>,
            pub passphrase: ::data_views::Field<&'a str>,
            pub ssl_profile: ::data_views::Field<&'a str>,
        }
    }
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct VlanAssignmentGroups<'a, Mode> (::data_views::Field<vlan_assignment_groups::Item<'a, Mode>>);

pub mod vlan_assignment_groups {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub members: ::data_views::RequiredValue<&'a str, Mode>,
    }
}

#[::data_views::data_view]
pub struct Eapol<'a, Mode> {
    pub vlan_change_logoff_disabled: ::data_views::Field<bool>,
    pub unresponsive_action_traffic_allow_vlan: ::data_views::Field<i64>,
}
