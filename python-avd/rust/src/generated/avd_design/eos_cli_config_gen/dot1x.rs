// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct MacBasedAuthentication {
        scalar delay("delay", 0) -> i64;
        scalar hold_period("hold_period", 1) -> i64;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct RadiusAvPairUsernameFormat {
        scalar delimiter("delimiter", 0) -> &'a str;
        scalar mac_string_case("mac_string_case", 1) -> &'a str;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct RadiusAvPair {
        scalar service_type("service_type", 0) -> bool;
        scalar framed_mtu("framed_mtu", 1) -> i64;
        model lldp("lldp", 2) -> radius_av_pair::Lldp<'a>;
        model dhcp("dhcp", 3) -> radius_av_pair::Dhcp<'a>;
        model filter_id("filter_id", 4) -> radius_av_pair::FilterId<'a>;
    }
}

pub mod radius_av_pair {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Lldp {
            model system_name("system_name", 0) -> lldp::SystemName<'a>;
            model system_description("system_description", 1) -> lldp::SystemDescription<'a>;
        }
    }

    pub mod lldp {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct SystemName {
                scalar enabled("enabled", 0) -> bool;
                scalar auth_only("auth_only", 1) -> bool;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct SystemDescription {
                scalar enabled("enabled", 0) -> bool;
                scalar auth_only("auth_only", 1) -> bool;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Dhcp {
            model hostname("hostname", 0) -> dhcp::Hostname<'a>;
            model parameter_request_list("parameter_request_list", 1) -> dhcp::ParameterRequestList<'a>;
            model vendor_class_id("vendor_class_id", 2) -> dhcp::VendorClassId<'a>;
        }
    }

    pub mod dhcp {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Hostname {
                scalar enabled("enabled", 0) -> bool;
                scalar auth_only("auth_only", 1) -> bool;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct ParameterRequestList {
                scalar enabled("enabled", 0) -> bool;
                scalar auth_only("auth_only", 1) -> bool;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct VendorClassId {
                scalar enabled("enabled", 0) -> bool;
                scalar auth_only("auth_only", 1) -> bool;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct FilterId {
            scalar delimiter_period("delimiter_period", 0) -> bool;
            scalar ipv4_ipv6_required("ipv4_ipv6_required", 1) -> bool;
            scalar multiple("multiple", 2) -> bool;
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Aaa {
        model unresponsive("unresponsive", 0) -> aaa::Unresponsive<'a>;
        scalar accounting_update_interval("accounting_update_interval", 1) -> i64;
    }
}

pub mod aaa {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Unresponsive {
            scalar eap_response("eap_response", 0) -> &'a str;
            model action("action", 1) -> unresponsive::Action<'a>;
            model phone_action("phone_action", 2) -> unresponsive::PhoneAction<'a>;
            scalar recovery_action_reauthenticate("recovery_action_reauthenticate", 3) -> bool;
        }
    }

    pub mod unresponsive {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Action {
                scalar apply_alternate("apply_alternate", 0) -> bool;
                scalar traffic_allow_vlan("traffic_allow_vlan", 1) -> i64;
                scalar apply_cached_results("apply_cached_results", 2) -> bool;
                model cached_results_timeout("cached_results_timeout", 3) -> action::CachedResultsTimeout<'a>;
                scalar traffic_allow("traffic_allow", 4) -> bool;
            }
        }

        pub mod action {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct CachedResultsTimeout {
                    scalar time_duration("time_duration", 0) -> i64;
                    scalar time_duration_unit("time_duration_unit", 1) -> &'a str;
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct PhoneAction {
                scalar apply_cached_results("apply_cached_results", 0) -> bool;
                model cached_results_timeout("cached_results_timeout", 1) -> phone_action::CachedResultsTimeout<'a>;
                scalar apply_alternate("apply_alternate", 2) -> bool;
                scalar traffic_allow("traffic_allow", 3) -> bool;
            }
        }

        pub mod phone_action {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct CachedResultsTimeout {
                    scalar time_duration("time_duration", 0) -> i64;
                    scalar time_duration_unit("time_duration_unit", 1) -> &'a str;
                }
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct CaptivePortal {
        scalar enabled("enabled", 0) -> bool;
        scalar url("url", 1) -> &'a str;
        scalar ssl_profile("ssl_profile", 2) -> &'a str;
        scalar start_limit_infinite("start_limit_infinite", 3) -> bool;
        scalar access_list_ipv4("access_list_ipv4", 4) -> &'a str;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Supplicant {
        model profiles("profiles", 0) -> supplicant::Profiles<'a>;
        scalar logging("logging", 1) -> bool;
        scalar disconnect_cached_results_timeout("disconnect_cached_results_timeout", 2) -> i64;
    }
}

pub mod supplicant {

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Profiles {
            model item (0) -> profiles::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod profiles {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                scalar eap_method("eap_method", 1) -> &'a str;
                scalar identity("identity", 2) -> &'a str;
                scalar passphrase_type("passphrase_type", 3) -> &'a str;
                scalar passphrase("passphrase", 4) -> &'a str;
                scalar ssl_profile("ssl_profile", 5) -> &'a str;
            }
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct VlanAssignmentGroups {
        model item (0) -> vlan_assignment_groups::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod vlan_assignment_groups {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar members("members", 1) -> &'a str;
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Eapol {
        scalar vlan_change_logoff_disabled("vlan_change_logoff_disabled", 0) -> bool;
        scalar unresponsive_action_traffic_allow_vlan("unresponsive_action_traffic_allow_vlan", 1) -> i64;
    }
}
