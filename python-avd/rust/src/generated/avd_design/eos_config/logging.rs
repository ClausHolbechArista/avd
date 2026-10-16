// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(python(name = "EOSConfig.Logging.Buffered", schema = Dict(513)))]
pub struct Buffered<'a, Mode> {
    #[data_view(schema = Int(219))]
    pub size: ::validated_data::Field<i64>,
    #[data_view(schema = Str(457))]
    pub level: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(python(name = "EOSConfig.Logging.Synchronous", schema = Dict(514)))]
pub struct Synchronous<'a, Mode> {
    #[data_view(schema = Str(459))]
    pub level: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(python(name = "EOSConfig.Logging.Format", schema = Dict(515)))]
pub struct Format<'a, Mode> {
    #[data_view(schema = Str(460))]
    pub timestamp: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(461))]
    pub hostname: ::validated_data::Field<&'a str>,
    #[data_view(schema = Bool(231))]
    pub sequence_numbers: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(232))]
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

#[::validated_data::data_view(python(name = "EOSConfig.Logging.Policy", schema = Dict(520)))]
pub struct Policy<'a, Mode> {
    #[data_view(schema = Dict(519), rename = "match")]
    pub field_match: ::validated_data::Field<policy::FieldMatch<'a, Mode>>,
}

pub mod policy {

    #[::validated_data::data_view(python(name = "EOSConfig.Logging.Policy.Match", schema = Dict(519)))]
    pub struct FieldMatch<'a, Mode> {
        #[data_view(schema = List(220))]
        pub match_lists: ::validated_data::Field<field_match::MatchLists<'a, Mode>>,
    }

    pub mod field_match {

        #[::validated_data::data_view(indexed_list, primary_key(name), python(name = "EOSConfig.Logging.Policy.Match.MatchLists", schema = List(220), item_name = "EOSConfig.Logging.Policy.Match.MatchListsItem"))]
        pub struct MatchLists<'a, Mode> (::validated_data::Field<match_lists::Item<'a, Mode>>);

        pub mod match_lists {

            #[::validated_data::data_view(python(name = "EOSConfig.Logging.Policy.Match.MatchListsItemBase", schema = Dict(518)))]
            pub struct Item<'a, Mode> {
                #[data_view(schema = Str(469))]
                pub name: ::validated_data::Field<&'a str>,
                #[data_view(schema = Str(470))]
                pub action: ::validated_data::RequiredValue<&'a str, Mode>,
            }
        }
    }
}

#[::validated_data::data_view(python(name = "EOSConfig.Logging.Event", schema = Dict(523)))]
pub struct Event<'a, Mode> {
    #[data_view(schema = Int(220))]
    pub congestion_drops_interval: ::validated_data::Field<i64>,
    #[data_view(schema = Bool(7))]
    pub global_link_status: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(522))]
    pub storm_control: ::validated_data::Field<event::StormControl<'a, Mode>>,
}

pub mod event {

    #[::validated_data::data_view(python(name = "EOSConfig.Logging.Event.StormControl", schema = Dict(522)))]
    pub struct StormControl<'a, Mode> {
        #[data_view(schema = Dict(521))]
        pub discards: ::validated_data::Field<storm_control::Discards<'a, Mode>>,
    }

    pub mod storm_control {

        #[::validated_data::data_view(python(name = "EOSConfig.Logging.Event.StormControl.Discards", schema = Dict(521)))]
        pub struct Discards<'a, Mode> {
            #[data_view(schema = Bool(7), rename = "global")]
            pub field_global: ::validated_data::Field<bool>,
            #[data_view(schema = Int(221))]
            pub interval: ::validated_data::Field<i64>,
        }
    }
}

#[::validated_data::data_view(indexed_list, primary_key(facility), python(name = "EOSConfig.Logging.Level", schema = List(221), item_name = "EOSConfig.Logging.LevelItem"))]
pub struct Level<'a, Mode> (::validated_data::Field<level::Item<'a, Mode>>);

pub mod level {

    #[::validated_data::data_view(python(name = "EOSConfig.Logging.LevelItemBase", schema = Dict(524)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(3))]
        pub facility: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(471))]
        pub severity: ::validated_data::RequiredValue<&'a str, Mode>,
    }
}
