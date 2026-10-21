// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "EOSConfig.EventHandlers", schema = List(140), item_name = "EOSConfig.EventHandlersItem"))]
pub struct EventHandlers<'a, Mode> (::validated_data::Field<Item<'a, Mode>>);

#[::validated_data::data_view(python(name = "EOSConfig.EventHandlersItemBase", schema = Dict(372)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(323))]
    pub name: ::validated_data::Field<&'a str>,
    #[data_view(schema = Dict(367))]
    pub actions: ::validated_data::Field<item::Actions<'a, Mode>>,
    #[data_view(schema = Int(164))]
    pub delay: ::validated_data::Field<i64>,
    #[data_view(schema = Str(326))]
    pub trigger: ::validated_data::Field<&'a str>,
    #[data_view(schema = Dict(368))]
    pub trigger_on_counters: ::validated_data::Field<item::TriggerOnCounters<'a, Mode>>,
    #[data_view(schema = Dict(369))]
    pub trigger_on_logging: ::validated_data::Field<item::TriggerOnLogging<'a, Mode>>,
    #[data_view(schema = Dict(370))]
    pub trigger_on_intf: ::validated_data::Field<item::TriggerOnIntf<'a, Mode>>,
    #[data_view(schema = Dict(371))]
    pub trigger_on_maintenance: ::validated_data::Field<item::TriggerOnMaintenance<'a, Mode>>,
    #[data_view(schema = Bool(189))]
    pub asynchronous: ::validated_data::Field<bool>,
}

pub mod item {

    #[::validated_data::data_view(python(name = "EOSConfig.EventHandlersItemBase.Actions", schema = Dict(367)))]
    pub struct Actions<'a, Mode> {
        #[data_view(schema = Str(324))]
        pub bash_command: ::validated_data::Field<&'a str>,
        #[data_view(schema = Bool(184))]
        pub log: ::validated_data::Field<bool>,
        #[data_view(schema = Str(325))]
        pub increment_device_health_metric: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view(python(name = "EOSConfig.EventHandlersItemBase.TriggerOnCounters", schema = Dict(368)))]
    pub struct TriggerOnCounters<'a, Mode> {
        #[data_view(schema = Str(327))]
        pub condition: ::validated_data::Field<&'a str>,
        #[data_view(schema = Bool(185))]
        pub granularity_per_source: ::validated_data::Field<bool>,
        #[data_view(schema = Int(165))]
        pub poll_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(python(name = "EOSConfig.EventHandlersItemBase.TriggerOnLogging", schema = Dict(369)))]
    pub struct TriggerOnLogging<'a, Mode> {
        #[data_view(schema = Int(165))]
        pub poll_interval: ::validated_data::Field<i64>,
        #[data_view(schema = Str(328))]
        pub regex: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view(python(name = "EOSConfig.EventHandlersItemBase.TriggerOnIntf", schema = Dict(370)))]
    pub struct TriggerOnIntf<'a, Mode> {
        #[data_view(schema = Str(329))]
        pub interface: ::validated_data::RequiredValue<&'a str, Mode>,
        #[data_view(schema = Bool(186))]
        pub ip: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(187))]
        pub ipv6: ::validated_data::Field<bool>,
        #[data_view(schema = Bool(188))]
        pub operstatus: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view(python(name = "EOSConfig.EventHandlersItemBase.TriggerOnMaintenance", schema = Dict(371)))]
    pub struct TriggerOnMaintenance<'a, Mode> {
        #[data_view(schema = Str(330))]
        pub operation: ::validated_data::RequiredValue<&'a str, Mode>,
        #[data_view(schema = Str(331))]
        pub bgp_peer: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(332))]
        pub action: ::validated_data::RequiredValue<&'a str, Mode>,
        #[data_view(schema = Str(333))]
        pub stage: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(334))]
        pub vrf: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(335))]
        pub interface: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(336))]
        pub unit: ::validated_data::Field<&'a str>,
    }
}
