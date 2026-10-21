// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view]
pub struct DaemonTerminattr<'a, Mode> {
    pub cvaddrs: ::validated_data::Field<Cvaddrs<'a, Mode>>,
    pub clusters: ::validated_data::Field<Clusters<'a, Mode>>,
    pub cvauth: ::validated_data::Field<Cvauth<'a, Mode>>,
    pub cvobscurekeyfile: ::validated_data::Field<bool>,
    pub cvproxy: ::validated_data::Field<&'a str>,
    pub cvsourceip: ::validated_data::Field<&'a str>,
    pub cvsourceintf: ::validated_data::Field<&'a str>,
    pub cvvrf: ::validated_data::Field<&'a str>,
    pub cvgnmi: ::validated_data::Field<bool>,
    pub disable_aaa: ::validated_data::Field<bool>,
    pub grpcaddr: ::validated_data::Field<&'a str>,
    pub grpcreadonly: ::validated_data::Field<bool>,
    pub ingestexclude: ::validated_data::Field<&'a str>,
    pub smashexcludes: ::validated_data::Field<&'a str>,
    pub sysdbexcludes: ::validated_data::Field<&'a str>,
    pub taillogs: ::validated_data::Field<&'a str>,
    pub ecodhcpaddr: ::validated_data::Field<&'a str>,
    pub ipfix: ::validated_data::Field<bool>,
    pub ipfixaddr: ::validated_data::Field<&'a str>,
    pub sflow: ::validated_data::Field<bool>,
    pub sflowaddr: ::validated_data::Field<&'a str>,
    pub cvconfig: ::validated_data::Field<bool>,
    pub cv_loss_timeout: ::validated_data::Field<i64>,
    pub cvtargetconfigs: ::validated_data::Field<Cvtargetconfigs<'a, Mode>>,
    pub flowdns: ::validated_data::Field<bool>,
    pub custom_cv_options: ::validated_data::Field<CustomCvOptions<'a, Mode>>,
}

#[::validated_data::data_view(list)]
pub struct Cvaddrs<'a, Mode> (::validated_data::Field<&'a str>);

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Clusters<'a, Mode> (::validated_data::Field<clusters::Item<'a, Mode>>);

pub mod clusters {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub cvaddrs: ::validated_data::RequiredValue<item::Cvaddrs<'a, Mode>, Mode>,
        pub cvauth: ::validated_data::Field<item::Cvauth<'a, Mode>>,
        pub cvobscurekeyfile: ::validated_data::Field<bool>,
        pub cvproxy: ::validated_data::Field<&'a str>,
        pub cvsourceip: ::validated_data::Field<&'a str>,
        pub cvsourceintf: ::validated_data::Field<&'a str>,
        pub cvvrf: ::validated_data::Field<&'a str>,
    }

    pub mod item {

        #[::validated_data::data_view(list)]
        pub struct Cvaddrs<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view]
        pub struct Cvauth<'a, Mode> {
            pub method: ::validated_data::Field<&'a str>,
            pub key: ::validated_data::Field<&'a str>,
            pub token_file: ::validated_data::Field<&'a str>,
            pub cert_file: ::validated_data::Field<&'a str>,
            pub ca_file: ::validated_data::Field<&'a str>,
            pub key_file: ::validated_data::Field<&'a str>,
        }
    }
}

#[::validated_data::data_view]
pub struct Cvauth<'a, Mode> {
    pub method: ::validated_data::Field<&'a str>,
    pub key: ::validated_data::Field<&'a str>,
    pub token_file: ::validated_data::Field<&'a str>,
    pub cert_file: ::validated_data::Field<&'a str>,
    pub ca_file: ::validated_data::Field<&'a str>,
    pub key_file: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(list)]
pub struct Cvtargetconfigs<'a, Mode> (::validated_data::Field<&'a str>);

#[::validated_data::data_view(list)]
pub struct CustomCvOptions<'a, Mode> (::validated_data::Field<custom_cv_options::Item<'a, Mode>>);

pub mod custom_cv_options {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub flag: ::validated_data::RequiredValue<&'a str, Mode>,
        pub value: ::validated_data::Field<&'a str>,
    }
}
