// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
    pub actions: ::validated_data::Field<item::Actions<'a, Mode>>,
    pub delay: ::validated_data::Field<i64>,
    pub trigger: ::validated_data::Field<&'a str>,
    pub trigger_on_counters: ::validated_data::Field<item::TriggerOnCounters<'a, Mode>>,
    pub trigger_on_logging: ::validated_data::Field<item::TriggerOnLogging<'a, Mode>>,
    pub trigger_on_intf: ::validated_data::Field<item::TriggerOnIntf<'a, Mode>>,
    pub trigger_on_maintenance: ::validated_data::Field<item::TriggerOnMaintenance<'a, Mode>>,
    pub asynchronous: ::validated_data::Field<bool>,
}

pub mod item {

    #[::validated_data::data_view]
    pub struct Actions<'a, Mode> {
        pub bash_command: ::validated_data::Field<&'a str>,
        pub log: ::validated_data::Field<bool>,
        pub increment_device_health_metric: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view]
    pub struct TriggerOnCounters<'a, Mode> {
        pub condition: ::validated_data::Field<&'a str>,
        pub granularity_per_source: ::validated_data::Field<bool>,
        pub poll_interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct TriggerOnLogging<'a, Mode> {
        pub poll_interval: ::validated_data::Field<i64>,
        pub regex: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view]
    pub struct TriggerOnIntf<'a, Mode> {
        pub interface: ::validated_data::RequiredValue<&'a str, Mode>,
        pub ip: ::validated_data::Field<bool>,
        pub ipv6: ::validated_data::Field<bool>,
        pub operstatus: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view]
    pub struct TriggerOnMaintenance<'a, Mode> {
        pub operation: ::validated_data::RequiredValue<&'a str, Mode>,
        pub bgp_peer: ::validated_data::Field<&'a str>,
        pub action: ::validated_data::RequiredValue<&'a str, Mode>,
        pub stage: ::validated_data::Field<&'a str>,
        pub vrf: ::validated_data::Field<&'a str>,
        pub interface: ::validated_data::Field<&'a str>,
        pub unit: ::validated_data::Field<&'a str>,
    }
}
