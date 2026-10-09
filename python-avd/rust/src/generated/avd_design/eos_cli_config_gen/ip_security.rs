// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct IkePolicies<'a, Mode> (::validated_data::Field<ike_policies::Item<'a, Mode>>);

pub mod ike_policies {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub local_id: ::validated_data::Field<&'a str>,
        pub local_id_fqdn: ::validated_data::Field<&'a str>,
        pub ike_lifetime: ::validated_data::Field<i64>,
        pub encryption: ::validated_data::Field<&'a str>,
        pub dh_group: ::validated_data::Field<i64>,
        pub integrity: ::validated_data::Field<&'a str>,
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct SaPolicies<'a, Mode> (::validated_data::Field<sa_policies::Item<'a, Mode>>);

pub mod sa_policies {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub sa_lifetime: ::validated_data::Field<item::SaLifetime<'a, Mode>>,
        pub esp: ::validated_data::Field<item::Esp<'a, Mode>>,
        pub pfs_dh_group: ::validated_data::Field<i64>,
    }

    pub mod item {

        #[::validated_data::data_view]
        pub struct SaLifetime<'a, Mode> {
            pub value: ::validated_data::Field<i64>,
            pub unit: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view]
        pub struct Esp<'a, Mode> {
            pub integrity: ::validated_data::Field<&'a str>,
            pub encryption: ::validated_data::Field<&'a str>,
        }
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Profiles<'a, Mode> (::validated_data::Field<profiles::Item<'a, Mode>>);

pub mod profiles {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub ike_policy: ::validated_data::Field<&'a str>,
        pub sa_policy: ::validated_data::Field<&'a str>,
        pub connection: ::validated_data::Field<&'a str>,
        pub shared_key: ::validated_data::Field<&'a str>,
        pub dpd: ::validated_data::Field<item::Dpd<'a, Mode>>,
        pub mode: ::validated_data::Field<&'a str>,
        pub flow_parallelization_encapsulation_udp: ::validated_data::Field<bool>,
    }

    pub mod item {

        #[::validated_data::data_view]
        pub struct Dpd<'a, Mode> {
            pub interval: ::validated_data::RequiredValue<i64, Mode>,
            pub time: ::validated_data::RequiredValue<i64, Mode>,
            pub action: ::validated_data::RequiredValue<&'a str, Mode>,
        }
    }
}

#[::validated_data::data_view]
pub struct KeyController<'a, Mode> {
    pub profile: ::validated_data::Field<&'a str>,
}
