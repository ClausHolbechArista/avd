// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct Logging<'a, Mode> {
    pub console: ::data_views::Field<&'a str>,
    pub monitor: ::data_views::Field<&'a str>,
    pub buffered: ::data_views::Field<Buffered<'a, Mode>>,
    pub repeat_messages: ::data_views::Field<bool>,
    pub trap: ::data_views::Field<&'a str>,
    pub synchronous: ::data_views::Field<Synchronous<'a, Mode>>,
    pub format: ::data_views::Field<Format<'a, Mode>>,
    pub facility: ::data_views::Field<&'a str>,
    pub source_interface: ::data_views::Field<&'a str>,
    pub local_interface: ::data_views::Field<&'a str>,
    pub vrfs: ::data_views::Field<Vrfs<'a, Mode>>,
    pub policy: ::data_views::Field<Policy<'a, Mode>>,
    pub event: ::data_views::Field<Event<'a, Mode>>,
    pub level: ::data_views::Field<Level<'a, Mode>>,
}

#[::data_views::data_view(python(name = "EOSConfig.Logging.Buffered", schema = Dict(513)))]
pub struct Buffered<'a, Mode> {
    #[data_view(schema = Int(219))]
    pub size: ::data_views::Field<i64>,
    #[data_view(schema = Str(457))]
    pub level: ::data_views::Field<&'a str>,
}

#[::data_views::data_view(python(name = "EOSConfig.Logging.Synchronous", schema = Dict(514)))]
pub struct Synchronous<'a, Mode> {
    #[data_view(schema = Str(459))]
    pub level: ::data_views::Field<&'a str>,
}

#[::data_views::data_view(python(name = "EOSConfig.Logging.Format", schema = Dict(515)))]
pub struct Format<'a, Mode> {
    #[data_view(schema = Str(460))]
    pub timestamp: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(461))]
    pub hostname: ::data_views::Field<&'a str>,
    #[data_view(schema = Bool(231))]
    pub sequence_numbers: ::data_views::Field<bool>,
    #[data_view(schema = Bool(232))]
    pub rfc5424: ::data_views::Field<bool>,
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Vrfs<'a, Mode> (::data_views::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub source_interface: ::data_views::Field<&'a str>,
        pub local_interface: ::data_views::Field<&'a str>,
        pub hosts: ::data_views::Field<item::Hosts<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view(indexed_list, primary_key(name))]
        pub struct Hosts<'a, Mode> (::data_views::Field<hosts::Item<'a, Mode>>);

        pub mod hosts {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::data_views::Field<&'a str>,
                pub protocol: ::data_views::Field<&'a str>,
                pub ports: ::data_views::Field<item::Ports<'a, Mode>>,
                pub ssl_profile: ::data_views::Field<&'a str>,
            }

            pub mod item {

                #[::data_views::data_view(list)]
                pub struct Ports<'a, Mode> (::data_views::Field<i64>);
            }
        }
    }
}

#[::data_views::data_view(python(name = "EOSConfig.Logging.Policy", schema = Dict(520)))]
pub struct Policy<'a, Mode> {
    #[data_view(schema = Dict(519), rename = "match")]
    pub field_match: ::data_views::Field<policy::FieldMatch<'a, Mode>>,
}

pub mod policy {

    #[::data_views::data_view(python(name = "EOSConfig.Logging.Policy.Match", schema = Dict(519)))]
    pub struct FieldMatch<'a, Mode> {
        #[data_view(schema = List(220))]
        pub match_lists: ::data_views::Field<field_match::MatchLists<'a, Mode>>,
    }

    pub mod field_match {

        #[::data_views::data_view(indexed_list, primary_key(name), python(name = "EOSConfig.Logging.Policy.Match.MatchLists", schema = List(220), item_name = "EOSConfig.Logging.Policy.Match.MatchListsItem"))]
        pub struct MatchLists<'a, Mode> (::data_views::Field<match_lists::Item<'a, Mode>>);

        pub mod match_lists {

            #[::data_views::data_view(python(name = "EOSConfig.Logging.Policy.Match.MatchListsItemBase", schema = Dict(518)))]
            pub struct Item<'a, Mode> {
                #[data_view(schema = Str(469))]
                pub name: ::data_views::Field<&'a str>,
                #[data_view(schema = Str(470))]
                pub action: ::data_views::RequiredValue<&'a str, Mode>,
            }
        }
    }
}

#[::data_views::data_view(python(name = "EOSConfig.Logging.Event", schema = Dict(523)))]
pub struct Event<'a, Mode> {
    #[data_view(schema = Int(220))]
    pub congestion_drops_interval: ::data_views::Field<i64>,
    #[data_view(schema = Bool(7))]
    pub global_link_status: ::data_views::Field<bool>,
    #[data_view(schema = Dict(522))]
    pub storm_control: ::data_views::Field<event::StormControl<'a, Mode>>,
}

pub mod event {

    #[::data_views::data_view(python(name = "EOSConfig.Logging.Event.StormControl", schema = Dict(522)))]
    pub struct StormControl<'a, Mode> {
        #[data_view(schema = Dict(521))]
        pub discards: ::data_views::Field<storm_control::Discards<'a, Mode>>,
    }

    pub mod storm_control {

        #[::data_views::data_view(python(name = "EOSConfig.Logging.Event.StormControl.Discards", schema = Dict(521)))]
        pub struct Discards<'a, Mode> {
            #[data_view(schema = Bool(7), rename = "global")]
            pub field_global: ::data_views::Field<bool>,
            #[data_view(schema = Int(221))]
            pub interval: ::data_views::Field<i64>,
        }
    }
}

#[::data_views::data_view(indexed_list, primary_key(facility), python(name = "EOSConfig.Logging.Level", schema = List(221), item_name = "EOSConfig.Logging.LevelItem"))]
pub struct Level<'a, Mode> (::data_views::Field<level::Item<'a, Mode>>);

pub mod level {

    #[::data_views::data_view(python(name = "EOSConfig.Logging.LevelItemBase", schema = Dict(524)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(3))]
        pub facility: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(471))]
        pub severity: ::data_views::RequiredValue<&'a str, Mode>,
    }
}
