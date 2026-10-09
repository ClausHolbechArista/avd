// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Policy<'a, Mode> {
    pub local_default_role: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view]
pub struct Exec<'a, Mode> {
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

#[::validated_data::data_view]
pub struct Commands<'a, Mode> {
    pub all_default: ::validated_data::Field<&'a str>,
    pub privilege: ::validated_data::Field<commands::Privilege<'a, Mode>>,
}

pub mod commands {

    #[::validated_data::data_view(list)]
    pub struct Privilege<'a, Mode> (::validated_data::Field<privilege::Item<'a, Mode>>);

    pub mod privilege {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub level: ::validated_data::RequiredValue<&'a str, Mode>,
            pub default: ::validated_data::RequiredValue<&'a str, Mode>,
        }
    }
}
