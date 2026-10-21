// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "AVDDesign.AaaSettings", schema = Dict(1385)))]
pub struct AaaSettings<'a, Mode> {
    #[data_view(schema = Dict(1366))]
    pub enable_password: ::data_views::Field<EnablePassword<'a, Mode>>,
    #[data_view(schema = Dict(1370))]
    pub tacacs: ::data_views::Field<Tacacs<'a, Mode>>,
    #[data_view(schema = Dict(1374))]
    pub radius: ::data_views::Field<Radius<'a, Mode>>,
    #[data_view(schema = Dict(1376))]
    pub authentication: ::data_views::Field<Authentication<'a, Mode>>,
    #[data_view(schema = Dict(1378))]
    pub authorization: ::data_views::Field<Authorization<'a, Mode>>,
    #[data_view(schema = Dict(1382))]
    pub accounting: ::data_views::Field<Accounting<'a, Mode>>,
    #[data_view(schema = Dict(1383))]
    pub root_login: ::data_views::Field<RootLogin<'a, Mode>>,
    #[data_view(schema = List(561))]
    pub local_users: ::data_views::Field<LocalUsers<'a, Mode>>,
}

#[::data_views::data_view(python(name = "AVDDesign.AaaSettings.EnablePassword", schema = Dict(1366)))]
pub struct EnablePassword<'a, Mode> {
    #[data_view(schema = Str(1082))]
    pub password: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1083))]
    pub cleartext_password: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1084))]
    pub password_type: ::data_views::Field<&'a str>,
}

#[::data_views::data_view(python(name = "AVDDesign.AaaSettings.Tacacs", schema = Dict(1370)))]
pub struct Tacacs<'a, Mode> {
    #[data_view(schema = List(554))]
    pub servers: ::data_views::Field<tacacs::Servers<'a, Mode>>,
    #[data_view(schema = List(555))]
    pub vrfs: ::data_views::Field<tacacs::Vrfs<'a, Mode>>,
    #[data_view(schema = Dict(1369))]
    pub policy: ::data_views::Field<tacacs::Policy<'a, Mode>>,
}

pub mod tacacs {

    #[::data_views::data_view(list, python(name = "AVDDesign.AaaSettings.Tacacs.Servers", schema = List(554)))]
    pub struct Servers<'a, Mode> (::data_views::Field<servers::Item<'a, Mode>>);

    pub mod servers {

        #[::data_views::data_view(python(name = "AVDDesign.AaaSettings.Tacacs.ServersItem", schema = Dict(1367)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(1085))]
            pub host: ::data_views::RequiredValue<&'a str, Mode>,
            #[data_view(schema = List(553))]
            pub groups: ::data_views::Field<item::Groups<'a, Mode>>,
            #[data_view(schema = Str(1087))]
            pub vrf: ::data_views::Field<&'a str>,
            #[data_view(schema = Int(479))]
            pub timeout: ::data_views::Field<i64>,
            #[data_view(schema = Str(1088))]
            pub key: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(1089))]
            pub cleartext_key: ::data_views::Field<&'a str>,
        }

        pub mod item {

            #[::data_views::data_view(list, python(name = "AVDDesign.AaaSettings.Tacacs.ServersItem.Groups", schema = List(553)))]
            pub struct Groups<'a, Mode> (::data_views::Field<&'a str>);
        }
    }

    #[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.AaaSettings.Tacacs.Vrfs", schema = List(555), item_name = "AVDDesign.AaaSettings.Tacacs.VrfsItem"))]
    pub struct Vrfs<'a, Mode> (::data_views::Field<vrfs::Item<'a, Mode>>);

    pub mod vrfs {

        #[::data_views::data_view(python(name = "AVDDesign.AaaSettings.Tacacs.VrfsItemBase", schema = Dict(1368)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(400))]
            pub name: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(1090))]
            pub source_interface: ::data_views::Field<&'a str>,
        }
    }

    #[::data_views::data_view(python(name = "AVDDesign.AaaSettings.Tacacs.Policy", schema = Dict(1369)))]
    pub struct Policy<'a, Mode> {
        #[data_view(schema = Bool(340))]
        pub ignore_unknown_mandatory_attribute: ::data_views::Field<bool>,
    }
}

#[::data_views::data_view(python(name = "AVDDesign.AaaSettings.Radius", schema = Dict(1374)))]
pub struct Radius<'a, Mode> {
    #[data_view(schema = List(557))]
    pub servers: ::data_views::Field<radius::Servers<'a, Mode>>,
    #[data_view(schema = List(558))]
    pub vrfs: ::data_views::Field<radius::Vrfs<'a, Mode>>,
}

pub mod radius {

    #[::data_views::data_view(list, python(name = "AVDDesign.AaaSettings.Radius.Servers", schema = List(557)))]
    pub struct Servers<'a, Mode> (::data_views::Field<servers::Item<'a, Mode>>);

    pub mod servers {

        #[::data_views::data_view(python(name = "AVDDesign.AaaSettings.Radius.ServersItem", schema = Dict(1372)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(1091))]
            pub host: ::data_views::RequiredValue<&'a str, Mode>,
            #[data_view(schema = List(556))]
            pub groups: ::data_views::Field<item::Groups<'a, Mode>>,
            #[data_view(schema = Str(1092))]
            pub vrf: ::data_views::Field<&'a str>,
            #[data_view(schema = Int(352))]
            pub timeout: ::data_views::Field<i64>,
            #[data_view(schema = Int(353))]
            pub retransmit: ::data_views::Field<i64>,
            #[data_view(schema = Str(1093))]
            pub key: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(1094))]
            pub cleartext_key: ::data_views::Field<&'a str>,
            #[data_view(schema = Dict(1371))]
            pub tls: ::data_views::Field<super::super::super::super::eos_config::radius_server::servers::item::Tls<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view(list, python(name = "AVDDesign.AaaSettings.Radius.ServersItem.Groups", schema = List(556)))]
            pub struct Groups<'a, Mode> (::data_views::Field<&'a str>);
        }
    }

    #[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.AaaSettings.Radius.Vrfs", schema = List(558), item_name = "AVDDesign.AaaSettings.Radius.VrfsItem"))]
    pub struct Vrfs<'a, Mode> (::data_views::Field<vrfs::Item<'a, Mode>>);

    pub mod vrfs {

        #[::data_views::data_view(python(name = "AVDDesign.AaaSettings.Radius.VrfsItemBase", schema = Dict(1373)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(400))]
            pub name: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(1095))]
            pub source_interface: ::data_views::Field<&'a str>,
        }
    }
}

#[::data_views::data_view(python(name = "AVDDesign.AaaSettings.Authentication", schema = Dict(1376)))]
pub struct Authentication<'a, Mode> {
    #[data_view(schema = Dict(41))]
    pub login: ::data_views::Field<super::super::eos_config::aaa_authentication::Login<'a, Mode>>,
    #[data_view(schema = Dict(42))]
    pub enable: ::data_views::Field<super::super::eos_config::aaa_authentication::Enable<'a, Mode>>,
    #[data_view(schema = Dict(46))]
    pub policies: ::data_views::Field<super::super::eos_config::aaa_authentication::Policies<'a, Mode>>,
}

pub mod authentication {
}

#[::data_views::data_view(python(name = "AVDDesign.AaaSettings.Authorization", schema = Dict(1378)))]
pub struct Authorization<'a, Mode> {
    #[data_view(schema = Dict(48))]
    pub policy: ::data_views::Field<super::super::eos_config::aaa_authorization::Policy<'a, Mode>>,
    #[data_view(schema = Dict(49))]
    pub exec: ::data_views::Field<super::super::eos_config::aaa_authorization::Exec<'a, Mode>>,
    #[data_view(schema = Bool(7))]
    pub config_commands: ::data_views::Field<bool>,
    #[data_view(schema = Bool(7))]
    pub serial_console: ::data_views::Field<bool>,
    #[data_view(schema = Dict(52))]
    pub commands: ::data_views::Field<super::super::eos_config::aaa_authorization::Commands<'a, Mode>>,
}

pub mod authorization {
}

#[::data_views::data_view(python(name = "AVDDesign.AaaSettings.Accounting", schema = Dict(1382)))]
pub struct Accounting<'a, Mode> {
    #[data_view(schema = Dict(32))]
    pub exec: ::data_views::Field<super::super::eos_config::aaa_accounting::Exec<'a, Mode>>,
    #[data_view(schema = Dict(34))]
    pub system: ::data_views::Field<super::super::eos_config::aaa_accounting::System<'a, Mode>>,
    #[data_view(schema = Dict(39))]
    pub commands: ::data_views::Field<super::super::eos_config::aaa_accounting::Commands<'a, Mode>>,
}

pub mod accounting {
}

#[::data_views::data_view(python(name = "AVDDesign.AaaSettings.RootLogin", schema = Dict(1383)))]
pub struct RootLogin<'a, Mode> {
    #[data_view(schema = Bool(340))]
    pub enabled: ::data_views::Field<bool>,
    #[data_view(schema = Str(3))]
    pub sha512_password: ::data_views::Field<&'a str>,
}

#[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.AaaSettings.LocalUsers", schema = List(561), item_name = "AVDDesign.AaaSettings.LocalUsersItem"))]
pub struct LocalUsers<'a, Mode> (::data_views::Field<local_users::Item<'a, Mode>>);

pub mod local_users {

    #[::data_views::data_view(python(name = "AVDDesign.AaaSettings.LocalUsersItemBase", schema = Dict(1384)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(1100))]
        pub sha512_password: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1101))]
        pub cleartext_password: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1084))]
        pub password_type: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(451))]
        pub name: ::data_views::Field<&'a str>,
        #[data_view(schema = Bool(228))]
        pub disabled: ::data_views::Field<bool>,
        #[data_view(schema = Int(218))]
        pub privilege: ::data_views::Field<i64>,
        #[data_view(schema = Str(452))]
        pub role: ::data_views::Field<&'a str>,
        #[data_view(schema = Bool(229))]
        pub no_password: ::data_views::Field<bool>,
        #[data_view(schema = Str(3))]
        pub ssh_key: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(3))]
        pub secondary_ssh_key: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(454))]
        pub shell: ::data_views::Field<&'a str>,
    }
}
