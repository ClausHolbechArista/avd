// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(name), python(name = "EOSConfig.EventHandlers", schema = List(140), item_name = "EOSConfig.EventHandlersItem"))]
pub struct EventHandlers<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view(python(name = "EOSConfig.EventHandlersItemBase", schema = Dict(372)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(323))]
    pub name: ::data_views::Field<&'a str>,
    #[data_view(schema = Dict(367))]
    pub actions: ::data_views::Field<item::Actions<'a, Mode>>,
    #[data_view(schema = Int(164))]
    pub delay: ::data_views::Field<i64>,
    #[data_view(schema = Str(326))]
    pub trigger: ::data_views::Field<&'a str>,
    #[data_view(schema = Dict(368))]
    pub trigger_on_counters: ::data_views::Field<item::TriggerOnCounters<'a, Mode>>,
    #[data_view(schema = Dict(369))]
    pub trigger_on_logging: ::data_views::Field<item::TriggerOnLogging<'a, Mode>>,
    #[data_view(schema = Dict(370))]
    pub trigger_on_intf: ::data_views::Field<item::TriggerOnIntf<'a, Mode>>,
    #[data_view(schema = Dict(371))]
    pub trigger_on_maintenance: ::data_views::Field<item::TriggerOnMaintenance<'a, Mode>>,
    #[data_view(schema = Bool(189))]
    pub asynchronous: ::data_views::Field<bool>,
}

pub mod item {

    #[::data_views::data_view(python(name = "EOSConfig.EventHandlersItemBase.Actions", schema = Dict(367)))]
    pub struct Actions<'a, Mode> {
        #[data_view(schema = Str(324))]
        pub bash_command: ::data_views::Field<&'a str>,
        #[data_view(schema = Bool(184))]
        pub log: ::data_views::Field<bool>,
        #[data_view(schema = Str(325))]
        pub increment_device_health_metric: ::data_views::Field<&'a str>,
    }

    #[::data_views::data_view(python(name = "EOSConfig.EventHandlersItemBase.TriggerOnCounters", schema = Dict(368)))]
    pub struct TriggerOnCounters<'a, Mode> {
        #[data_view(schema = Str(327))]
        pub condition: ::data_views::Field<&'a str>,
        #[data_view(schema = Bool(185))]
        pub granularity_per_source: ::data_views::Field<bool>,
        #[data_view(schema = Int(165))]
        pub poll_interval: ::data_views::Field<i64>,
    }

    #[::data_views::data_view(python(name = "EOSConfig.EventHandlersItemBase.TriggerOnLogging", schema = Dict(369)))]
    pub struct TriggerOnLogging<'a, Mode> {
        #[data_view(schema = Int(165))]
        pub poll_interval: ::data_views::Field<i64>,
        #[data_view(schema = Str(328))]
        pub regex: ::data_views::Field<&'a str>,
    }

    #[::data_views::data_view(python(name = "EOSConfig.EventHandlersItemBase.TriggerOnIntf", schema = Dict(370)))]
    pub struct TriggerOnIntf<'a, Mode> {
        #[data_view(schema = Str(329))]
        pub interface: ::data_views::RequiredValue<&'a str, Mode>,
        #[data_view(schema = Bool(186))]
        pub ip: ::data_views::Field<bool>,
        #[data_view(schema = Bool(187))]
        pub ipv6: ::data_views::Field<bool>,
        #[data_view(schema = Bool(188))]
        pub operstatus: ::data_views::Field<bool>,
    }

    #[::data_views::data_view(python(name = "EOSConfig.EventHandlersItemBase.TriggerOnMaintenance", schema = Dict(371)))]
    pub struct TriggerOnMaintenance<'a, Mode> {
        #[data_view(schema = Str(330))]
        pub operation: ::data_views::RequiredValue<&'a str, Mode>,
        #[data_view(schema = Str(331))]
        pub bgp_peer: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(332))]
        pub action: ::data_views::RequiredValue<&'a str, Mode>,
        #[data_view(schema = Str(333))]
        pub stage: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(334))]
        pub vrf: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(335))]
        pub interface: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(336))]
        pub unit: ::data_views::Field<&'a str>,
    }
}
