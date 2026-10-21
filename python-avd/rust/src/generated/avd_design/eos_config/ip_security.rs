// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct IpSecurity<'a, Mode> {
    pub ike_policies: ::data_views::Field<IkePolicies<'a, Mode>>,
    pub sa_policies: ::data_views::Field<SaPolicies<'a, Mode>>,
    pub profiles: ::data_views::Field<Profiles<'a, Mode>>,
    pub key_controller: ::data_views::Field<KeyController<'a, Mode>>,
    pub hardware_encryption_disabled: ::data_views::Field<bool>,
    pub connection_tx_interface_match_source_ip: ::data_views::Field<bool>,
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct IkePolicies<'a, Mode> (::data_views::Field<ike_policies::Item<'a, Mode>>);

pub mod ike_policies {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub local_id: ::data_views::Field<&'a str>,
        pub local_id_fqdn: ::data_views::Field<&'a str>,
        pub ike_lifetime: ::data_views::Field<i64>,
        pub encryption: ::data_views::Field<&'a str>,
        pub dh_group: ::data_views::Field<i64>,
        pub integrity: ::data_views::Field<&'a str>,
    }
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct SaPolicies<'a, Mode> (::data_views::Field<sa_policies::Item<'a, Mode>>);

pub mod sa_policies {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub sa_lifetime: ::data_views::Field<item::SaLifetime<'a, Mode>>,
        pub esp: ::data_views::Field<item::Esp<'a, Mode>>,
        pub pfs_dh_group: ::data_views::Field<i64>,
    }

    pub mod item {

        #[::data_views::data_view]
        pub struct SaLifetime<'a, Mode> {
            pub value: ::data_views::Field<i64>,
            pub unit: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view]
        pub struct Esp<'a, Mode> {
            pub integrity: ::data_views::Field<&'a str>,
            pub encryption: ::data_views::Field<&'a str>,
        }
    }
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Profiles<'a, Mode> (::data_views::Field<profiles::Item<'a, Mode>>);

pub mod profiles {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub ike_policy: ::data_views::Field<&'a str>,
        pub sa_policy: ::data_views::Field<&'a str>,
        pub connection: ::data_views::Field<&'a str>,
        pub shared_key: ::data_views::Field<&'a str>,
        pub dpd: ::data_views::Field<item::Dpd<'a, Mode>>,
        pub mode: ::data_views::Field<&'a str>,
        pub flow_parallelization_encapsulation_udp: ::data_views::Field<bool>,
    }

    pub mod item {

        #[::data_views::data_view]
        pub struct Dpd<'a, Mode> {
            pub interval: ::data_views::RequiredValue<i64, Mode>,
            pub time: ::data_views::RequiredValue<i64, Mode>,
            pub action: ::data_views::RequiredValue<&'a str, Mode>,
        }
    }
}

#[::data_views::data_view]
pub struct KeyController<'a, Mode> {
    pub profile: ::data_views::Field<&'a str>,
}
