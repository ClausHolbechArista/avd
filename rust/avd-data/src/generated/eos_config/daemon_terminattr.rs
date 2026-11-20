// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct DaemonTerminattr<'a, Mode> {
    pub cvaddrs: ::data_views::Field<Cvaddrs<'a, Mode>>,
    pub clusters: ::data_views::Field<Clusters<'a, Mode>>,
    pub cvauth: ::data_views::Field<Cvauth<'a, Mode>>,
    pub cvobscurekeyfile: ::data_views::Field<bool>,
    pub cvproxy: ::data_views::Field<&'a str>,
    pub cvsourceip: ::data_views::Field<&'a str>,
    pub cvsourceintf: ::data_views::Field<&'a str>,
    pub cvvrf: ::data_views::Field<&'a str>,
    pub cvgnmi: ::data_views::Field<bool>,
    pub disable_aaa: ::data_views::Field<bool>,
    pub grpcaddr: ::data_views::Field<&'a str>,
    pub grpcreadonly: ::data_views::Field<bool>,
    pub ingestexclude: ::data_views::Field<&'a str>,
    pub smashexcludes: ::data_views::Field<&'a str>,
    pub sysdbexcludes: ::data_views::Field<&'a str>,
    pub taillogs: ::data_views::Field<&'a str>,
    pub ecodhcpaddr: ::data_views::Field<&'a str>,
    pub ipfix: ::data_views::Field<bool>,
    pub ipfixaddr: ::data_views::Field<&'a str>,
    pub sflow: ::data_views::Field<bool>,
    pub sflowaddr: ::data_views::Field<&'a str>,
    pub cvconfig: ::data_views::Field<bool>,
    pub cv_loss_timeout: ::data_views::Field<i64>,
    pub cvtargetconfigs: ::data_views::Field<Cvtargetconfigs<'a, Mode>>,
    pub flowdns: ::data_views::Field<bool>,
    pub custom_cv_options: ::data_views::Field<CustomCvOptions<'a, Mode>>,
}

#[::data_views::data_view(list)]
pub struct Cvaddrs<'a, Mode> (::data_views::Field<&'a str>);

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Clusters<'a, Mode> (::data_views::Field<clusters::Item<'a, Mode>>);

pub mod clusters {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub cvaddrs: ::data_views::RequiredValue<item::Cvaddrs<'a, Mode>, Mode>,
        pub cvauth: ::data_views::Field<item::Cvauth<'a, Mode>>,
        pub cvobscurekeyfile: ::data_views::Field<bool>,
        pub cvproxy: ::data_views::Field<&'a str>,
        pub cvsourceip: ::data_views::Field<&'a str>,
        pub cvsourceintf: ::data_views::Field<&'a str>,
        pub cvvrf: ::data_views::Field<&'a str>,
    }

    pub mod item {

        #[::data_views::data_view(list)]
        pub struct Cvaddrs<'a, Mode> (::data_views::Field<&'a str>);

        #[::data_views::data_view]
        pub struct Cvauth<'a, Mode> {
            pub method: ::data_views::Field<&'a str>,
            pub key: ::data_views::Field<&'a str>,
            pub token_file: ::data_views::Field<&'a str>,
            pub cert_file: ::data_views::Field<&'a str>,
            pub ca_file: ::data_views::Field<&'a str>,
            pub key_file: ::data_views::Field<&'a str>,
        }
    }
}

#[::data_views::data_view]
pub struct Cvauth<'a, Mode> {
    pub method: ::data_views::Field<&'a str>,
    pub key: ::data_views::Field<&'a str>,
    pub token_file: ::data_views::Field<&'a str>,
    pub cert_file: ::data_views::Field<&'a str>,
    pub ca_file: ::data_views::Field<&'a str>,
    pub key_file: ::data_views::Field<&'a str>,
}

#[::data_views::data_view(list)]
pub struct Cvtargetconfigs<'a, Mode> (::data_views::Field<&'a str>);

#[::data_views::data_view(list)]
pub struct CustomCvOptions<'a, Mode> (::data_views::Field<custom_cv_options::Item<'a, Mode>>);

pub mod custom_cv_options {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub flag: ::data_views::RequiredValue<&'a str, Mode>,
        pub value: ::data_views::Field<&'a str>,
    }
}
