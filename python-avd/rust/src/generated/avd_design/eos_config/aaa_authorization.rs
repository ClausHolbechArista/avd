// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct AaaAuthorization<'a, Mode> {
    pub policy: ::data_views::Field<Policy<'a, Mode>>,
    pub exec: ::data_views::Field<Exec<'a, Mode>>,
    pub config_commands: ::data_views::Field<bool>,
    pub serial_console: ::data_views::Field<bool>,
    pub dynamic: ::data_views::Field<Dynamic<'a, Mode>>,
    pub commands: ::data_views::Field<Commands<'a, Mode>>,
}

#[::data_views::data_view(python(name = "EOSConfig.AaaAuthorization.Policy", schema = Dict(48)))]
pub struct Policy<'a, Mode> {
    #[data_view(schema = Str(3))]
    pub local_default_role: ::data_views::Field<&'a str>,
}

#[::data_views::data_view(python(name = "EOSConfig.AaaAuthorization.Exec", schema = Dict(49)))]
pub struct Exec<'a, Mode> {
    #[data_view(schema = Str(18))]
    pub default: ::data_views::Field<&'a str>,
}

#[::data_views::data_view]
pub struct Dynamic<'a, Mode> {
    pub dot1x_additional_groups: ::data_views::Field<dynamic::Dot1xAdditionalGroups<'a, Mode>>,
}

pub mod dynamic {

    #[::data_views::data_view(list)]
    pub struct Dot1xAdditionalGroups<'a, Mode> (::data_views::Field<&'a str>);
}

#[::data_views::data_view(python(name = "EOSConfig.AaaAuthorization.Commands", schema = Dict(52)))]
pub struct Commands<'a, Mode> {
    #[data_view(schema = Str(19))]
    pub all_default: ::data_views::Field<&'a str>,
    #[data_view(schema = List(27))]
    pub privilege: ::data_views::Field<commands::Privilege<'a, Mode>>,
}

pub mod commands {

    #[::data_views::data_view(list, python(name = "EOSConfig.AaaAuthorization.Commands.Privilege", schema = List(27)))]
    pub struct Privilege<'a, Mode> (::data_views::Field<privilege::Item<'a, Mode>>);

    pub mod privilege {

        #[::data_views::data_view(python(name = "EOSConfig.AaaAuthorization.Commands.PrivilegeItem", schema = Dict(51)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(20))]
            pub level: ::data_views::RequiredValue<&'a str, Mode>,
            #[data_view(schema = Str(21))]
            pub default: ::data_views::RequiredValue<&'a str, Mode>,
        }
    }
}
