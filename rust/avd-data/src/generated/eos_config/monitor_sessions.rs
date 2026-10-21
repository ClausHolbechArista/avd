// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct MonitorSessions<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view]
pub struct Item<'a, Mode> {
    pub name: ::data_views::Field<&'a str>,
    pub sources: ::data_views::Field<item::Sources<'a, Mode>>,
    pub destinations: ::data_views::Field<item::Destinations<'a, Mode>>,
    pub encapsulation_gre_metadata_tx: ::data_views::Field<bool>,
    pub header_remove_size: ::data_views::Field<i64>,
    pub access_group: ::data_views::Field<item::AccessGroup<'a, Mode>>,
    pub rate_limit_per_ingress_chip: ::data_views::Field<&'a str>,
    pub rate_limit_per_egress_chip: ::data_views::Field<&'a str>,
    pub sample: ::data_views::Field<i64>,
    pub truncate: ::data_views::Field<item::Truncate<'a, Mode>>,
}

pub mod item {

    #[::data_views::data_view(indexed_list, primary_key(name))]
    pub struct Sources<'a, Mode> (::data_views::Field<sources::Item<'a, Mode>>);

    pub mod sources {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::data_views::Field<&'a str>,
            pub direction: ::data_views::Field<&'a str>,
            pub access_group: ::data_views::Field<item::AccessGroup<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view]
            pub struct AccessGroup<'a, Mode> {
                #[data_view(rename = "type")]
                pub field_type: ::data_views::Field<&'a str>,
                pub name: ::data_views::Field<&'a str>,
                pub priority: ::data_views::Field<i64>,
            }
        }
    }

    #[::data_views::data_view(list)]
    pub struct Destinations<'a, Mode> (::data_views::Field<&'a str>);

    #[::data_views::data_view]
    pub struct AccessGroup<'a, Mode> {
        #[data_view(rename = "type")]
        pub field_type: ::data_views::Field<&'a str>,
        pub name: ::data_views::Field<&'a str>,
    }

    #[::data_views::data_view]
    pub struct Truncate<'a, Mode> {
        pub enabled: ::data_views::Field<bool>,
        pub size: ::data_views::Field<i64>,
    }
}
