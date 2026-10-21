// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct AaaAuthentication<'a, Mode> {
    pub login: ::data_views::Field<Login<'a, Mode>>,
    pub enable: ::data_views::Field<Enable<'a, Mode>>,
    pub dot1x: ::data_views::Field<Dot1x<'a, Mode>>,
    pub policies: ::data_views::Field<Policies<'a, Mode>>,
}

#[::data_views::data_view(python(name = "EOSConfig.AaaAuthentication.Login", schema = Dict(41)))]
pub struct Login<'a, Mode> {
    #[data_view(schema = Str(13))]
    pub default: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(14))]
    pub command_api: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(15))]
    pub console: ::data_views::Field<&'a str>,
}

#[::data_views::data_view(python(name = "EOSConfig.AaaAuthentication.Enable", schema = Dict(42)))]
pub struct Enable<'a, Mode> {
    #[data_view(schema = Str(16))]
    pub default: ::data_views::Field<&'a str>,
}

#[::data_views::data_view]
pub struct Dot1x<'a, Mode> {
    pub default: ::data_views::Field<&'a str>,
}

#[::data_views::data_view(python(name = "EOSConfig.AaaAuthentication.Policies", schema = Dict(46)))]
pub struct Policies<'a, Mode> {
    #[data_view(schema = Bool(7))]
    pub on_failure_log: ::data_views::Field<bool>,
    #[data_view(schema = Bool(7))]
    pub on_success_log: ::data_views::Field<bool>,
    #[data_view(schema = Dict(44))]
    pub local: ::data_views::Field<policies::Local<'a, Mode>>,
    #[data_view(schema = Dict(45))]
    pub lockout: ::data_views::Field<policies::Lockout<'a, Mode>>,
}

pub mod policies {

    #[::data_views::data_view(python(name = "EOSConfig.AaaAuthentication.Policies.Local", schema = Dict(44)))]
    pub struct Local<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub allow_nopassword: ::data_views::Field<bool>,
    }

    #[::data_views::data_view(python(name = "EOSConfig.AaaAuthentication.Policies.Lockout", schema = Dict(45)))]
    pub struct Lockout<'a, Mode> {
        #[data_view(schema = Int(1))]
        pub failure: ::data_views::RequiredValue<i64, Mode>,
        #[data_view(schema = Int(2))]
        pub duration: ::data_views::RequiredValue<i64, Mode>,
        #[data_view(schema = Int(3))]
        pub window: ::data_views::Field<i64>,
    }
}
