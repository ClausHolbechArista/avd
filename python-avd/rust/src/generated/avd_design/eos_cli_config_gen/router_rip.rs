// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(indexed_list, primary_key(vrf))]
pub struct Vrfs<'a, Mode> (::validated_data::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub vrf: ::validated_data::Field<&'a str>,
        pub metric_default: ::validated_data::Field<i64>,
        pub networks: ::validated_data::Field<item::Networks<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view(list)]
        pub struct Networks<'a, Mode> (::validated_data::Field<&'a str>);
    }
}
