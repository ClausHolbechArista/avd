// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "AVDDesign.LoggingSettings", schema = Dict(1617)))]
pub struct LoggingSettings<'a, Mode> {
    #[data_view(schema = Bool(551))]
    pub use_local_interface_cli: ::data_views::Field<bool>,
    #[data_view(schema = List(671))]
    pub hosts: ::data_views::Field<Hosts<'a, Mode>>,
    #[data_view(schema = List(672))]
    pub vrfs: ::data_views::Field<Vrfs<'a, Mode>>,
    #[data_view(schema = Str(455))]
    pub console: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(456))]
    pub monitor: ::data_views::Field<&'a str>,
    #[data_view(schema = Dict(513))]
    pub buffered: ::data_views::Field<super::super::eos_config::logging::Buffered<'a, Mode>>,
    #[data_view(schema = Bool(230))]
    pub repeat_messages: ::data_views::Field<bool>,
    #[data_view(schema = Str(458))]
    pub trap: ::data_views::Field<&'a str>,
    #[data_view(schema = Dict(514))]
    pub synchronous: ::data_views::Field<super::super::eos_config::logging::Synchronous<'a, Mode>>,
    #[data_view(schema = Dict(515))]
    pub format: ::data_views::Field<super::super::eos_config::logging::Format<'a, Mode>>,
    #[data_view(schema = Str(462))]
    pub facility: ::data_views::Field<&'a str>,
    #[data_view(schema = Dict(520))]
    pub policy: ::data_views::Field<super::super::eos_config::logging::Policy<'a, Mode>>,
    #[data_view(schema = Dict(523))]
    pub event: ::data_views::Field<super::super::eos_config::logging::Event<'a, Mode>>,
    #[data_view(schema = List(221))]
    pub level: ::data_views::Field<super::super::eos_config::logging::Level<'a, Mode>>,
    #[data_view(schema = Dict(652))]
    pub monitor_layer1: ::data_views::Field<super::super::eos_config::MonitorLayer1<'a, Mode>>,
}

#[::data_views::data_view(list, python(name = "AVDDesign.LoggingSettings.Hosts", schema = List(671)))]
pub struct Hosts<'a, Mode> (::data_views::Field<hosts::Item<'a, Mode>>);

pub mod hosts {

    #[::data_views::data_view(python(name = "AVDDesign.LoggingSettings.HostsItem", schema = Dict(1615)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(1442))]
        pub name: ::data_views::RequiredValue<&'a str, Mode>,
        #[data_view(schema = Str(1443))]
        pub vrf: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(467))]
        pub protocol: ::data_views::Field<&'a str>,
        #[data_view(schema = List(217))]
        pub ports: ::data_views::Field<item::Ports<'a, Mode>>,
        #[data_view(schema = Str(468))]
        pub ssl_profile: ::data_views::Field<&'a str>,
    }

    pub mod item {

        #[::data_views::data_view(list, python(name = "AVDDesign.LoggingSettings.HostsItem.Ports", schema = List(217)))]
        pub struct Ports<'a, Mode> (::data_views::Field<i64>);
    }
}

#[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.LoggingSettings.Vrfs", schema = List(672), item_name = "AVDDesign.LoggingSettings.VrfsItem"))]
pub struct Vrfs<'a, Mode> (::data_views::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::data_views::data_view(python(name = "AVDDesign.LoggingSettings.VrfsItemBase", schema = Dict(1616)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(25))]
        pub name: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1444))]
        pub source_interface: ::data_views::Field<&'a str>,
    }
}
