// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Authentication {
        model radius_groups("radius_groups", 0) -> authentication::RadiusGroups<'a>;
    }
}

pub mod authentication {

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct RadiusGroups {
            scalar item (0) -> &'a str;
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Accounting {
        scalar enabled("enabled", 0) -> bool;
        scalar mode("mode", 1) -> &'a str;
        model radius_groups("radius_groups", 2) -> accounting::RadiusGroups<'a>;
        scalar multicast("multicast", 3) -> bool;
        scalar syslog("syslog", 4) -> bool;
    }
}

pub mod accounting {

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct RadiusGroups {
            scalar item (0) -> &'a str;
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct DynamicAuthorization {
        scalar enabled("enabled", 0) -> bool;
        model additional_groups("additional_groups", 1) -> dynamic_authorization::AdditionalGroups<'a>;
    }
}

pub mod dynamic_authorization {

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct AdditionalGroups {
            scalar item (0) -> &'a str;
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct MacBasedAuthentication {
        model username_format("username_format", 0) -> mac_based_authentication::UsernameFormat<'a>;
        scalar delay("delay", 1) -> i64;
        scalar hold_period("hold_period", 2) -> i64;
    }
}

pub mod mac_based_authentication {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct UsernameFormat {
            scalar delimiter("delimiter", 0) -> &'a str;
            scalar letter_case("letter_case", 1) -> &'a str;
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct RadiusAvPairs {
        scalar service_type("service_type", 0) -> bool;
        scalar framed_mtu("framed_mtu", 1) -> i64;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct DeviceProfiling {
        scalar enabled("enabled", 0) -> bool;
        model dhcp("dhcp", 1) -> device_profiling::Dhcp<'a>;
        model lldp("lldp", 2) -> device_profiling::Lldp<'a>;
    }
}

pub mod device_profiling {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Dhcp {
            scalar enabled("enabled", 0) -> bool;
            model hostname("hostname", 1) -> dhcp::Hostname<'a>;
            model parameter_request_list("parameter_request_list", 2) -> dhcp::ParameterRequestList<'a>;
            model vendor_class_id("vendor_class_id", 3) -> dhcp::VendorClassId<'a>;
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
        pub struct Lldp {
            scalar enabled("enabled", 0) -> bool;
            model system_name("system_name", 1) -> lldp::SystemName<'a>;
            model system_description("system_description", 2) -> lldp::SystemDescription<'a>;
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
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct WebAuthentication {
        scalar enabled("enabled", 0) -> bool;
        scalar ipv4_acl("ipv4_acl", 1) -> &'a str;
        scalar url("url", 2) -> &'a str;
        scalar ssl_profile("ssl_profile", 3) -> &'a str;
        scalar start_limit_infinite("start_limit_infinite", 4) -> bool;
    }
}
