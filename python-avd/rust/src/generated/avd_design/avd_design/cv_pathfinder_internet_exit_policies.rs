// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar name("name", 0) -> &'a str;
        scalar field_type("type", 1) -> &'a str;
        scalar fallback_to_system_default("fallback_to_system_default", 2) -> bool;
        model zscaler("zscaler", 3) -> item::Zscaler<'a>;
    }
}

pub mod item {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Zscaler {
            scalar ipsec_key_salt("ipsec_key_salt", 0) -> &'a str;
            scalar domain_name("domain_name", 1) -> &'a str;
            scalar encrypt_traffic("encrypt_traffic", 2) -> bool;
            scalar download_bandwidth("download_bandwidth", 3) -> i64;
            scalar upload_bandwidth("upload_bandwidth", 4) -> i64;
            model firewall("firewall", 5) -> zscaler::Firewall<'a>;
            scalar acceptable_use_policy("acceptable_use_policy", 6) -> bool;
        }
    }

    pub mod zscaler {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Firewall {
                scalar enabled("enabled", 0) -> bool;
                scalar ips("ips", 1) -> bool;
            }
        }
    }
}
