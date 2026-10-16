// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(python(name = "EOSConfig.AaaAuthentication.Login", schema = Dict(41)))]
pub struct Login<'a, Mode> {
    #[data_view(schema = Str(13))]
    pub default: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(14))]
    pub command_api: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(15))]
    pub console: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(python(name = "EOSConfig.AaaAuthentication.Enable", schema = Dict(42)))]
pub struct Enable<'a, Mode> {
    #[data_view(schema = Str(16))]
    pub default: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view]
pub struct Dot1x<'a, Mode> {
    pub default: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(python(name = "EOSConfig.AaaAuthentication.Policies", schema = Dict(46)))]
pub struct Policies<'a, Mode> {
    #[data_view(schema = Bool(7))]
    pub on_failure_log: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(7))]
    pub on_success_log: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(44))]
    pub local: ::validated_data::Field<policies::Local<'a, Mode>>,
    #[data_view(schema = Dict(45))]
    pub lockout: ::validated_data::Field<policies::Lockout<'a, Mode>>,
}

pub mod policies {

    #[::validated_data::data_view(python(name = "EOSConfig.AaaAuthentication.Policies.Local", schema = Dict(44)))]
    pub struct Local<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub allow_nopassword: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view(python(name = "EOSConfig.AaaAuthentication.Policies.Lockout", schema = Dict(45)))]
    pub struct Lockout<'a, Mode> {
        #[data_view(schema = Int(1))]
        pub failure: ::validated_data::RequiredValue<i64, Mode>,
        #[data_view(schema = Int(2))]
        pub duration: ::validated_data::RequiredValue<i64, Mode>,
        #[data_view(schema = Int(3))]
        pub window: ::validated_data::Field<i64>,
    }
}
