// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
    pub id: ::validated_data::RequiredValue<i64, Mode>,
    pub description: ::validated_data::Field<&'a str>,
    pub ipsec: ::validated_data::Field<item::Ipsec<'a, Mode>>,
    pub import_path_groups: ::validated_data::Field<item::ImportPathGroups<'a, Mode>>,
    pub default_preference: ::validated_data::Field<&'a str>,
    pub excluded_from_default_policy: ::validated_data::Field<bool>,
    pub dps_keepalive: ::validated_data::Field<item::DpsKeepalive<'a, Mode>>,
}

pub mod item {

    #[::validated_data::data_view]
    pub struct Ipsec<'a, Mode> {
        pub dynamic_peers: ::validated_data::Field<bool>,
        pub static_peers: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view(list)]
    pub struct ImportPathGroups<'a, Mode> (::validated_data::Field<import_path_groups::Item<'a, Mode>>);

    pub mod import_path_groups {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub remote: ::validated_data::Field<&'a str>,
            pub local: ::validated_data::Field<&'a str>,
        }
    }

    #[::validated_data::data_view]
    pub struct DpsKeepalive<'a, Mode> {
        pub interval: ::validated_data::Field<&'a str>,
        pub failure_threshold: ::validated_data::Field<i64>,
    }
}
