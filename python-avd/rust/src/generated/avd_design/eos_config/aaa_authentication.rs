// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(python(name = "EOSConfig.AaaAuthentication.Login"))]
pub struct Login<'a, Mode> {
    pub default: ::validated_data::Field<&'a str>,
    pub command_api: ::validated_data::Field<&'a str>,
    pub console: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(python(name = "EOSConfig.AaaAuthentication.Enable"))]
pub struct Enable<'a, Mode> {
    pub default: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view]
pub struct Dot1x<'a, Mode> {
    pub default: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(python(name = "EOSConfig.AaaAuthentication.Policies"))]
pub struct Policies<'a, Mode> {
    pub on_failure_log: ::validated_data::Field<bool>,
    pub on_success_log: ::validated_data::Field<bool>,
    pub local: ::validated_data::Field<policies::Local<'a, Mode>>,
    pub lockout: ::validated_data::Field<policies::Lockout<'a, Mode>>,
}

pub mod policies {

    #[::validated_data::data_view(python(name = "EOSConfig.AaaAuthentication.Policies.Local"))]
    pub struct Local<'a, Mode> {
        pub allow_nopassword: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view(python(name = "EOSConfig.AaaAuthentication.Policies.Lockout"))]
    pub struct Lockout<'a, Mode> {
        pub failure: ::validated_data::RequiredValue<i64, Mode>,
        pub duration: ::validated_data::RequiredValue<i64, Mode>,
        pub window: ::validated_data::Field<i64>,
    }
}
