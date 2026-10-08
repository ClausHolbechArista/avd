// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct EngineIds {
        scalar local("local", 0) -> &'a str;
        model remotes("remotes", 1) -> engine_ids::Remotes<'a>;
    }
}

pub mod engine_ids {

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Remotes {
            model item (0) -> remotes::Item<'a>;
        }
    }

    pub mod remotes {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar id("id", 0) -> &'a str;
                scalar address("address", 1) -> &'a str;
                scalar udp_port("udp_port", 2) -> i64;
            }
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Extensions {
        model item (0) -> extensions::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod extensions {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar oid("oid", 0) -> &'a str;
            scalar path("path", 1) -> &'a str;
            scalar one_shot("one_shot", 2) -> bool;
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Communities {
        model item (0) -> communities::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod communities {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar access("access", 1) -> &'a str;
            model access_list_ipv4("access_list_ipv4", 2) -> item::AccessListIpv4<'a>;
            model access_list_ipv6("access_list_ipv6", 3) -> item::AccessListIpv6<'a>;
            scalar view("view", 4) -> &'a str;
        }
    }

    pub mod item {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct AccessListIpv4 {
                scalar name("name", 0) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct AccessListIpv6 {
                scalar name("name", 0) -> &'a str;
            }
        }
    }
}

::validation::define_archive_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Ipv4Acls {
        model item (0) -> ipv4_acls::Item<'a>;
    }
}

pub mod ipv4_acls {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar vrf("vrf", 1) -> &'a str;
        }
    }
}

::validation::define_archive_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Ipv6Acls {
        model item (0) -> ipv6_acls::Item<'a>;
    }
}

pub mod ipv6_acls {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar vrf("vrf", 1) -> &'a str;
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct LocalInterfaces {
        model item (0) -> local_interfaces::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod local_interfaces {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar vrf("vrf", 1) -> &'a str;
        }
    }
}

::validation::define_archive_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Views {
        model item (0) -> views::Item<'a>;
    }
}

pub mod views {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar mib_family_name("mib_family_name", 1) -> &'a str;
            scalar included("included", 2) -> bool;
        }
    }
}

::validation::define_archive_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Groups {
        model item (0) -> groups::Item<'a>;
    }
}

pub mod groups {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar version("version", 1) -> &'a str;
            scalar authentication("authentication", 2) -> &'a str;
            scalar read("read", 3) -> &'a str;
            scalar write("write", 4) -> &'a str;
            scalar notify("notify", 5) -> &'a str;
        }
    }
}

::validation::define_archive_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Users {
        model item (0) -> users::Item<'a>;
    }
}

pub mod users {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar group("group", 1) -> &'a str;
            scalar remote_address("remote_address", 2) -> &'a str;
            scalar udp_port("udp_port", 3) -> i64;
            scalar version("version", 4) -> &'a str;
            scalar localized("localized", 5) -> &'a str;
            scalar auth("auth", 6) -> &'a str;
            scalar auth_key_type("auth_key_type", 7) -> &'a str;
            scalar auth_key("auth_key", 8) -> &'a str;
            scalar auth_passphrase("auth_passphrase", 9) -> &'a str;
            scalar field_priv("priv", 10) -> &'a str;
            scalar priv_key_type("priv_key_type", 11) -> &'a str;
            scalar priv_key("priv_key", 12) -> &'a str;
            scalar priv_passphrase("priv_passphrase", 13) -> &'a str;
        }
    }
}

::validation::define_archive_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Hosts {
        model item (0) -> hosts::Item<'a>;
    }
}

pub mod hosts {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar host("host", 0) -> &'a str;
            scalar vrf("vrf", 1) -> &'a str;
            scalar version("version", 2) -> &'a str;
            scalar community("community", 3) -> &'a str;
            model users("users", 4) -> item::Users<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Users {
                model item (0) -> users::Item<'a>;
            }
        }

        pub mod users {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar username("username", 0) -> &'a str;
                    scalar authentication_level("authentication_level", 1) -> &'a str;
                }
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Traps {
        scalar enable("enable", 0) -> bool;
        model snmp_traps("snmp_traps", 1) -> traps::SnmpTraps<'a>;
    }
}

pub mod traps {

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct SnmpTraps {
            model item (0) -> snmp_traps::Item<'a>;
        }
    }

    pub mod snmp_traps {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                scalar enabled("enabled", 1) -> bool;
            }
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Vrfs {
        model item (0) -> vrfs::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod vrfs {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar enable("enable", 1) -> bool;
        }
    }
}
