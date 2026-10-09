// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct EnablePassword<'a, Mode> {
    pub password: ::validated_data::Field<&'a str>,
    pub cleartext_password: ::validated_data::Field<&'a str>,
    pub password_type: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view]
pub struct Tacacs<'a, Mode> {
    pub servers: ::validated_data::Field<tacacs::Servers<'a, Mode>>,
    pub vrfs: ::validated_data::Field<tacacs::Vrfs<'a, Mode>>,
    pub policy: ::validated_data::Field<tacacs::Policy<'a, Mode>>,
}

pub mod tacacs {

    #[::validated_data::data_view(list)]
    pub struct Servers<'a, Mode> (::validated_data::Field<servers::Item<'a, Mode>>);

    pub mod servers {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub host: ::validated_data::RequiredValue<&'a str, Mode>,
            pub groups: ::validated_data::Field<item::Groups<'a, Mode>>,
            pub vrf: ::validated_data::Field<&'a str>,
            pub timeout: ::validated_data::Field<i64>,
            pub key: ::validated_data::Field<&'a str>,
            pub cleartext_key: ::validated_data::Field<&'a str>,
        }

        pub mod item {

            #[::validated_data::data_view(list)]
            pub struct Groups<'a, Mode> (::validated_data::Field<&'a str>);
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct Vrfs<'a, Mode> (::validated_data::Field<vrfs::Item<'a, Mode>>);

    pub mod vrfs {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub source_interface: ::validated_data::Field<&'a str>,
        }
    }

    #[::validated_data::data_view]
    pub struct Policy<'a, Mode> {
        pub ignore_unknown_mandatory_attribute: ::validated_data::Field<bool>,
    }
}

#[::validated_data::data_view]
pub struct Radius<'a, Mode> {
    pub servers: ::validated_data::Field<radius::Servers<'a, Mode>>,
    pub vrfs: ::validated_data::Field<radius::Vrfs<'a, Mode>>,
}

pub mod radius {

    #[::validated_data::data_view(list)]
    pub struct Servers<'a, Mode> (::validated_data::Field<servers::Item<'a, Mode>>);

    pub mod servers {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub host: ::validated_data::RequiredValue<&'a str, Mode>,
            pub groups: ::validated_data::Field<item::Groups<'a, Mode>>,
            pub vrf: ::validated_data::Field<&'a str>,
            pub timeout: ::validated_data::Field<i64>,
            pub retransmit: ::validated_data::Field<i64>,
            pub key: ::validated_data::Field<&'a str>,
            pub cleartext_key: ::validated_data::Field<&'a str>,
            pub tls: ::validated_data::Field<super::super::super::super::eos_cli_config_gen::radius_server::servers::item::Tls<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view(list)]
            pub struct Groups<'a, Mode> (::validated_data::Field<&'a str>);
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct Vrfs<'a, Mode> (::validated_data::Field<vrfs::Item<'a, Mode>>);

    pub mod vrfs {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub source_interface: ::validated_data::Field<&'a str>,
        }
    }
}

#[::validated_data::data_view]
pub struct Authentication<'a, Mode> {
    pub login: ::validated_data::Field<super::super::eos_cli_config_gen::aaa_authentication::Login<'a, Mode>>,
    pub enable: ::validated_data::Field<super::super::eos_cli_config_gen::aaa_authentication::Enable<'a, Mode>>,
    pub policies: ::validated_data::Field<super::super::eos_cli_config_gen::aaa_authentication::Policies<'a, Mode>>,
}

pub mod authentication {
}

#[::validated_data::data_view]
pub struct Authorization<'a, Mode> {
    pub policy: ::validated_data::Field<super::super::eos_cli_config_gen::aaa_authorization::Policy<'a, Mode>>,
    pub exec: ::validated_data::Field<super::super::eos_cli_config_gen::aaa_authorization::Exec<'a, Mode>>,
    pub config_commands: ::validated_data::Field<bool>,
    pub serial_console: ::validated_data::Field<bool>,
    pub commands: ::validated_data::Field<super::super::eos_cli_config_gen::aaa_authorization::Commands<'a, Mode>>,
}

pub mod authorization {
}

#[::validated_data::data_view]
pub struct Accounting<'a, Mode> {
    pub exec: ::validated_data::Field<super::super::eos_cli_config_gen::aaa_accounting::Exec<'a, Mode>>,
    pub system: ::validated_data::Field<super::super::eos_cli_config_gen::aaa_accounting::System<'a, Mode>>,
    pub commands: ::validated_data::Field<super::super::eos_cli_config_gen::aaa_accounting::Commands<'a, Mode>>,
}

pub mod accounting {
}

#[::validated_data::data_view]
pub struct RootLogin<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub sha512_password: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct LocalUsers<'a, Mode> (::validated_data::Field<local_users::Item<'a, Mode>>);

pub mod local_users {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub sha512_password: ::validated_data::Field<&'a str>,
        pub cleartext_password: ::validated_data::Field<&'a str>,
        pub password_type: ::validated_data::Field<&'a str>,
        pub name: ::validated_data::Field<&'a str>,
        pub disabled: ::validated_data::Field<bool>,
        pub privilege: ::validated_data::Field<i64>,
        pub role: ::validated_data::Field<&'a str>,
        pub no_password: ::validated_data::Field<bool>,
        pub ssh_key: ::validated_data::Field<&'a str>,
        pub secondary_ssh_key: ::validated_data::Field<&'a str>,
        pub shell: ::validated_data::Field<&'a str>,
    }
}
