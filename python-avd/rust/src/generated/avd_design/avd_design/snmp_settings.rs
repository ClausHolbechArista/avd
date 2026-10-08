// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


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
            scalar source_interface("source_interface", 2) -> &'a str;
            scalar ipv4_acl("ipv4_acl", 3) -> &'a str;
            scalar ipv6_acl("ipv6_acl", 4) -> &'a str;
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
            scalar version("version", 2) -> &'a str;
            scalar auth("auth", 3) -> &'a str;
            scalar auth_passphrase("auth_passphrase", 4) -> &'a str;
            scalar field_priv("priv", 5) -> &'a str;
            scalar priv_passphrase("priv_passphrase", 6) -> &'a str;
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
            scalar ipv4_standard_acl("ipv4_standard_acl", 3) -> &'a str;
            model access_list_ipv6("access_list_ipv6", 4) -> item::AccessListIpv6<'a>;
            scalar view("view", 5) -> &'a str;
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
