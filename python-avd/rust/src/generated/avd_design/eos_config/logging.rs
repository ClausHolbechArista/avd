// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(python(name = "EOSConfig.Logging.Buffered"))]
pub struct Buffered<'a, Mode> {
    pub size: ::validated_data::Field<i64>,
    pub level: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(python(name = "EOSConfig.Logging.Synchronous"))]
pub struct Synchronous<'a, Mode> {
    pub level: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(python(name = "EOSConfig.Logging.Format"))]
pub struct Format<'a, Mode> {
    pub timestamp: ::validated_data::Field<&'a str>,
    pub hostname: ::validated_data::Field<&'a str>,
    pub sequence_numbers: ::validated_data::Field<bool>,
    pub rfc5424: ::validated_data::Field<bool>,
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Vrfs<'a, Mode> (::validated_data::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub source_interface: ::validated_data::Field<&'a str>,
        pub local_interface: ::validated_data::Field<&'a str>,
        pub hosts: ::validated_data::Field<item::Hosts<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view(indexed_list, primary_key(name))]
        pub struct Hosts<'a, Mode> (::validated_data::Field<hosts::Item<'a, Mode>>);

        pub mod hosts {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::validated_data::Field<&'a str>,
                pub protocol: ::validated_data::Field<&'a str>,
                pub ports: ::validated_data::Field<item::Ports<'a, Mode>>,
                pub ssl_profile: ::validated_data::Field<&'a str>,
            }

            pub mod item {

                #[::validated_data::data_view(list)]
                pub struct Ports<'a, Mode> (::validated_data::Field<i64>);
            }
        }
    }
}

#[::validated_data::data_view(python(name = "EOSConfig.Logging.Policy"))]
pub struct Policy<'a, Mode> {
    #[data_view(rename = "match")]
    pub field_match: ::validated_data::Field<policy::FieldMatch<'a, Mode>>,
}

pub mod policy {

    #[::validated_data::data_view(python(name = "EOSConfig.Logging.Policy.Match"))]
    pub struct FieldMatch<'a, Mode> {
        pub match_lists: ::validated_data::Field<field_match::MatchLists<'a, Mode>>,
    }

    pub mod field_match {

        #[::validated_data::data_view(indexed_list, primary_key(name), python(name = "EOSConfig.Logging.Policy.Match.MatchLists", item_name = "EOSConfig.Logging.Policy.Match.MatchListsItem"))]
        pub struct MatchLists<'a, Mode> (::validated_data::Field<match_lists::Item<'a, Mode>>);

        pub mod match_lists {

            #[::validated_data::data_view(python(name = "EOSConfig.Logging.Policy.Match.MatchListsItemBase"))]
            pub struct Item<'a, Mode> {
                pub name: ::validated_data::Field<&'a str>,
                pub action: ::validated_data::RequiredValue<&'a str, Mode>,
            }
        }
    }
}

#[::validated_data::data_view(python(name = "EOSConfig.Logging.Event"))]
pub struct Event<'a, Mode> {
    pub congestion_drops_interval: ::validated_data::Field<i64>,
    pub global_link_status: ::validated_data::Field<bool>,
    pub storm_control: ::validated_data::Field<event::StormControl<'a, Mode>>,
}

pub mod event {

    #[::validated_data::data_view(python(name = "EOSConfig.Logging.Event.StormControl"))]
    pub struct StormControl<'a, Mode> {
        pub discards: ::validated_data::Field<storm_control::Discards<'a, Mode>>,
    }

    pub mod storm_control {

        #[::validated_data::data_view(python(name = "EOSConfig.Logging.Event.StormControl.Discards"))]
        pub struct Discards<'a, Mode> {
            #[data_view(rename = "global")]
            pub field_global: ::validated_data::Field<bool>,
            pub interval: ::validated_data::Field<i64>,
        }
    }
}

#[::validated_data::data_view(indexed_list, primary_key(facility), python(name = "EOSConfig.Logging.Level", item_name = "EOSConfig.Logging.LevelItem"))]
pub struct Level<'a, Mode> (::validated_data::Field<level::Item<'a, Mode>>);

pub mod level {

    #[::validated_data::data_view(python(name = "EOSConfig.Logging.LevelItemBase"))]
    pub struct Item<'a, Mode> {
        pub facility: ::validated_data::Field<&'a str>,
        pub severity: ::validated_data::RequiredValue<&'a str, Mode>,
    }
}
