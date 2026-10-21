// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view]
pub struct AaaAuthorization<'a, Mode> {
    pub policy: ::validated_data::Field<Policy<'a, Mode>>,
    pub exec: ::validated_data::Field<Exec<'a, Mode>>,
    pub config_commands: ::validated_data::Field<bool>,
    pub serial_console: ::validated_data::Field<bool>,
    pub dynamic: ::validated_data::Field<Dynamic<'a, Mode>>,
    pub commands: ::validated_data::Field<Commands<'a, Mode>>,
}

#[::validated_data::data_view(python(name = "EOSConfig.AaaAuthorization.Policy", schema = Dict(48)))]
pub struct Policy<'a, Mode> {
    #[data_view(schema = Str(3))]
    pub local_default_role: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(python(name = "EOSConfig.AaaAuthorization.Exec", schema = Dict(49)))]
pub struct Exec<'a, Mode> {
    #[data_view(schema = Str(18))]
    pub default: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view]
pub struct Dynamic<'a, Mode> {
    pub dot1x_additional_groups: ::validated_data::Field<dynamic::Dot1xAdditionalGroups<'a, Mode>>,
}

pub mod dynamic {

    #[::validated_data::data_view(list)]
    pub struct Dot1xAdditionalGroups<'a, Mode> (::validated_data::Field<&'a str>);
}

#[::validated_data::data_view(python(name = "EOSConfig.AaaAuthorization.Commands", schema = Dict(52)))]
pub struct Commands<'a, Mode> {
    #[data_view(schema = Str(19))]
    pub all_default: ::validated_data::Field<&'a str>,
    #[data_view(schema = List(27))]
    pub privilege: ::validated_data::Field<commands::Privilege<'a, Mode>>,
}

pub mod commands {

    #[::validated_data::data_view(list, python(name = "EOSConfig.AaaAuthorization.Commands.Privilege", schema = List(27)))]
    pub struct Privilege<'a, Mode> (::validated_data::Field<privilege::Item<'a, Mode>>);

    pub mod privilege {

        #[::validated_data::data_view(python(name = "EOSConfig.AaaAuthorization.Commands.PrivilegeItem", schema = Dict(51)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(20))]
            pub level: ::validated_data::RequiredValue<&'a str, Mode>,
            #[data_view(schema = Str(21))]
            pub default: ::validated_data::RequiredValue<&'a str, Mode>,
        }
    }
}
