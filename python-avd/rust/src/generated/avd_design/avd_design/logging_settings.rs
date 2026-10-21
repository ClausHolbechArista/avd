// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(python(name = "AVDDesign.LoggingSettings", schema = Dict(1617)))]
pub struct LoggingSettings<'a, Mode> {
    #[data_view(schema = Bool(551))]
    pub use_local_interface_cli: ::validated_data::Field<bool>,
    #[data_view(schema = List(671))]
    pub hosts: ::validated_data::Field<Hosts<'a, Mode>>,
    #[data_view(schema = List(672))]
    pub vrfs: ::validated_data::Field<Vrfs<'a, Mode>>,
    #[data_view(schema = Str(455))]
    pub console: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(456))]
    pub monitor: ::validated_data::Field<&'a str>,
    #[data_view(schema = Dict(513))]
    pub buffered: ::validated_data::Field<super::super::eos_config::logging::Buffered<'a, Mode>>,
    #[data_view(schema = Bool(230))]
    pub repeat_messages: ::validated_data::Field<bool>,
    #[data_view(schema = Str(458))]
    pub trap: ::validated_data::Field<&'a str>,
    #[data_view(schema = Dict(514))]
    pub synchronous: ::validated_data::Field<super::super::eos_config::logging::Synchronous<'a, Mode>>,
    #[data_view(schema = Dict(515))]
    pub format: ::validated_data::Field<super::super::eos_config::logging::Format<'a, Mode>>,
    #[data_view(schema = Str(462))]
    pub facility: ::validated_data::Field<&'a str>,
    #[data_view(schema = Dict(520))]
    pub policy: ::validated_data::Field<super::super::eos_config::logging::Policy<'a, Mode>>,
    #[data_view(schema = Dict(523))]
    pub event: ::validated_data::Field<super::super::eos_config::logging::Event<'a, Mode>>,
    #[data_view(schema = List(221))]
    pub level: ::validated_data::Field<super::super::eos_config::logging::Level<'a, Mode>>,
    #[data_view(schema = Dict(652))]
    pub monitor_layer1: ::validated_data::Field<super::super::eos_config::MonitorLayer1<'a, Mode>>,
}

#[::validated_data::data_view(list, python(name = "AVDDesign.LoggingSettings.Hosts", schema = List(671)))]
pub struct Hosts<'a, Mode> (::validated_data::Field<hosts::Item<'a, Mode>>);

pub mod hosts {

    #[::validated_data::data_view(python(name = "AVDDesign.LoggingSettings.HostsItem", schema = Dict(1615)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(1442))]
        pub name: ::validated_data::RequiredValue<&'a str, Mode>,
        #[data_view(schema = Str(1443))]
        pub vrf: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(467))]
        pub protocol: ::validated_data::Field<&'a str>,
        #[data_view(schema = List(217))]
        pub ports: ::validated_data::Field<item::Ports<'a, Mode>>,
        #[data_view(schema = Str(468))]
        pub ssl_profile: ::validated_data::Field<&'a str>,
    }

    pub mod item {

        #[::validated_data::data_view(list, python(name = "AVDDesign.LoggingSettings.HostsItem.Ports", schema = List(217)))]
        pub struct Ports<'a, Mode> (::validated_data::Field<i64>);
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.LoggingSettings.Vrfs", schema = List(672), item_name = "AVDDesign.LoggingSettings.VrfsItem"))]
pub struct Vrfs<'a, Mode> (::validated_data::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::validated_data::data_view(python(name = "AVDDesign.LoggingSettings.VrfsItemBase", schema = Dict(1616)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(25))]
        pub name: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1444))]
        pub source_interface: ::validated_data::Field<&'a str>,
    }
}
