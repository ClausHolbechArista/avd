// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct EnablePassword {
        scalar password("password", 0) -> &'a str;
        scalar cleartext_password("cleartext_password", 1) -> &'a str;
        scalar password_type("password_type", 2) -> &'a str;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Tacacs {
        model servers("servers", 0) -> tacacs::Servers<'a>;
        model vrfs("vrfs", 1) -> tacacs::Vrfs<'a>;
        model policy("policy", 2) -> tacacs::Policy<'a>;
    }
}

pub mod tacacs {

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Servers {
            model item (0) -> servers::Item<'a>;
        }
    }

    pub mod servers {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar host("host", 0) -> &'a str;
                model groups("groups", 1) -> item::Groups<'a>;
                scalar vrf("vrf", 2) -> &'a str;
                scalar timeout("timeout", 3) -> i64;
                scalar key("key", 4) -> &'a str;
                scalar cleartext_key("cleartext_key", 5) -> &'a str;
            }
        }

        pub mod item {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Groups {
                    scalar item (0) -> &'a str;
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
                scalar source_interface("source_interface", 1) -> &'a str;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Policy {
            scalar ignore_unknown_mandatory_attribute("ignore_unknown_mandatory_attribute", 0) -> bool;
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Radius {
        model servers("servers", 0) -> radius::Servers<'a>;
        model vrfs("vrfs", 1) -> radius::Vrfs<'a>;
    }
}

pub mod radius {

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Servers {
            model item (0) -> servers::Item<'a>;
        }
    }

    pub mod servers {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar host("host", 0) -> &'a str;
                model groups("groups", 1) -> item::Groups<'a>;
                scalar vrf("vrf", 2) -> &'a str;
                scalar timeout("timeout", 3) -> i64;
                scalar retransmit("retransmit", 4) -> i64;
                scalar key("key", 5) -> &'a str;
                scalar cleartext_key("cleartext_key", 6) -> &'a str;
                model tls("tls", 7) -> super::super::super::super::eos_cli_config_gen::radius_server::servers::item::Tls<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Groups {
                    scalar item (0) -> &'a str;
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
                scalar source_interface("source_interface", 1) -> &'a str;
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Authentication {
        model login("login", 0) -> super::super::eos_cli_config_gen::aaa_authentication::Login<'a>;
        model enable("enable", 1) -> super::super::eos_cli_config_gen::aaa_authentication::Enable<'a>;
        model policies("policies", 2) -> super::super::eos_cli_config_gen::aaa_authentication::Policies<'a>;
    }
}

pub mod authentication {
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Authorization {
        model policy("policy", 0) -> super::super::eos_cli_config_gen::aaa_authorization::Policy<'a>;
        model exec("exec", 1) -> super::super::eos_cli_config_gen::aaa_authorization::Exec<'a>;
        scalar config_commands("config_commands", 2) -> bool;
        scalar serial_console("serial_console", 3) -> bool;
        model commands("commands", 4) -> super::super::eos_cli_config_gen::aaa_authorization::Commands<'a>;
    }
}

pub mod authorization {
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Accounting {
        model exec("exec", 0) -> super::super::eos_cli_config_gen::aaa_accounting::Exec<'a>;
        model system("system", 1) -> super::super::eos_cli_config_gen::aaa_accounting::System<'a>;
        model commands("commands", 2) -> super::super::eos_cli_config_gen::aaa_accounting::Commands<'a>;
    }
}

pub mod accounting {
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct RootLogin {
        scalar enabled("enabled", 0) -> bool;
        scalar sha512_password("sha512_password", 1) -> &'a str;
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct LocalUsers {
        model item (0) -> local_users::Item<'a>;
        primary_key_fields: [3];
    }
}

pub mod local_users {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar sha512_password("sha512_password", 0) -> &'a str;
            scalar cleartext_password("cleartext_password", 1) -> &'a str;
            scalar password_type("password_type", 2) -> &'a str;
            scalar name("name", 3) -> &'a str;
            scalar disabled("disabled", 4) -> bool;
            scalar privilege("privilege", 5) -> i64;
            scalar role("role", 6) -> &'a str;
            scalar no_password("no_password", 7) -> bool;
            scalar ssh_key("ssh_key", 8) -> &'a str;
            scalar secondary_ssh_key("secondary_ssh_key", 9) -> &'a str;
            scalar shell("shell", 10) -> &'a str;
        }
    }
}
