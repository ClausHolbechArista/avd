// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct IpIgmpSnooping<'a, Mode> {
    pub globally_enabled: ::data_views::Field<bool>,
    pub robustness_variable: ::data_views::Field<i64>,
    pub restart_query_interval: ::data_views::Field<i64>,
    pub interface_restart_query: ::data_views::Field<i64>,
    pub fast_leave: ::data_views::Field<bool>,
    pub querier: ::data_views::Field<Querier<'a, Mode>>,
    pub proxy: ::data_views::Field<bool>,
    pub vlans: ::data_views::Field<Vlans<'a, Mode>>,
}

#[::data_views::data_view]
pub struct Querier<'a, Mode> {
    pub enabled: ::data_views::Field<bool>,
    pub address: ::data_views::Field<&'a str>,
    pub query_interval: ::data_views::Field<i64>,
    pub max_response_time: ::data_views::Field<i64>,
    pub last_member_query_interval: ::data_views::Field<i64>,
    pub last_member_query_count: ::data_views::Field<i64>,
    pub startup_query_interval: ::data_views::Field<i64>,
    pub startup_query_count: ::data_views::Field<i64>,
    pub version: ::data_views::Field<i64>,
}

#[::data_views::data_view(indexed_list, primary_key(id))]
pub struct Vlans<'a, Mode> (::data_views::Field<vlans::Item<'a, Mode>>);

pub mod vlans {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub id: ::data_views::Field<i64>,
        pub enabled: ::data_views::Field<bool>,
        pub querier: ::data_views::Field<item::Querier<'a, Mode>>,
        pub max_groups: ::data_views::Field<i64>,
        pub fast_leave: ::data_views::Field<bool>,
        pub proxy: ::data_views::Field<bool>,
    }

    pub mod item {

        #[::data_views::data_view]
        pub struct Querier<'a, Mode> {
            pub enabled: ::data_views::Field<bool>,
            pub address: ::data_views::Field<&'a str>,
            pub query_interval: ::data_views::Field<i64>,
            pub max_response_time: ::data_views::Field<i64>,
            pub last_member_query_interval: ::data_views::Field<i64>,
            pub last_member_query_count: ::data_views::Field<i64>,
            pub startup_query_interval: ::data_views::Field<i64>,
            pub startup_query_count: ::data_views::Field<i64>,
            pub version: ::data_views::Field<i64>,
        }
    }
}
