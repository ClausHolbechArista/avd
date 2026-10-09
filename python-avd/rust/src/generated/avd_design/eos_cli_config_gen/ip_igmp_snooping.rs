// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Querier<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub address: ::validated_data::Field<&'a str>,
    pub query_interval: ::validated_data::Field<i64>,
    pub max_response_time: ::validated_data::Field<i64>,
    pub last_member_query_interval: ::validated_data::Field<i64>,
    pub last_member_query_count: ::validated_data::Field<i64>,
    pub startup_query_interval: ::validated_data::Field<i64>,
    pub startup_query_count: ::validated_data::Field<i64>,
    pub version: ::validated_data::Field<i64>,
}

#[::validated_data::data_view(indexed_list, primary_key(id))]
pub struct Vlans<'a, Mode> (::validated_data::Field<vlans::Item<'a, Mode>>);

pub mod vlans {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub id: ::validated_data::Field<i64>,
        pub enabled: ::validated_data::Field<bool>,
        pub querier: ::validated_data::Field<item::Querier<'a, Mode>>,
        pub max_groups: ::validated_data::Field<i64>,
        pub fast_leave: ::validated_data::Field<bool>,
        pub proxy: ::validated_data::Field<bool>,
    }

    pub mod item {

        #[::validated_data::data_view]
        pub struct Querier<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub address: ::validated_data::Field<&'a str>,
            pub query_interval: ::validated_data::Field<i64>,
            pub max_response_time: ::validated_data::Field<i64>,
            pub last_member_query_interval: ::validated_data::Field<i64>,
            pub last_member_query_count: ::validated_data::Field<i64>,
            pub startup_query_interval: ::validated_data::Field<i64>,
            pub startup_query_count: ::validated_data::Field<i64>,
            pub version: ::validated_data::Field<i64>,
        }
    }
}
