// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "EOSConfig.MacAddressTable", schema = Dict(535)))]
pub struct MacAddressTable<'a, Mode> {
    #[data_view(schema = Int(222))]
    pub aging_time: ::data_views::Field<i64>,
    #[data_view(schema = Dict(533))]
    pub notification_host_flap: ::data_views::Field<NotificationHostFlap<'a, Mode>>,
    #[data_view(schema = List(226))]
    pub static_entries: ::data_views::Field<StaticEntries<'a, Mode>>,
}

#[::data_views::data_view(python(name = "EOSConfig.MacAddressTable.NotificationHostFlap", schema = Dict(533)))]
pub struct NotificationHostFlap<'a, Mode> {
    #[data_view(schema = Bool(7))]
    pub logging: ::data_views::Field<bool>,
    #[data_view(schema = Dict(532))]
    pub detection: ::data_views::Field<notification_host_flap::Detection<'a, Mode>>,
}

pub mod notification_host_flap {

    #[::data_views::data_view(python(name = "EOSConfig.MacAddressTable.NotificationHostFlap.Detection", schema = Dict(532)))]
    pub struct Detection<'a, Mode> {
        #[data_view(schema = Int(223))]
        pub window: ::data_views::Field<i64>,
        #[data_view(schema = Int(224))]
        pub moves: ::data_views::Field<i64>,
    }
}

#[::data_views::data_view(list, python(name = "EOSConfig.MacAddressTable.StaticEntries", schema = List(226)))]
pub struct StaticEntries<'a, Mode> (::data_views::Field<static_entries::Item<'a, Mode>>);

pub mod static_entries {

    #[::data_views::data_view(python(name = "EOSConfig.MacAddressTable.StaticEntriesItem", schema = Dict(534)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(477))]
        pub mac_address: ::data_views::RequiredValue<&'a str, Mode>,
        #[data_view(schema = Int(225))]
        pub vlan: ::data_views::RequiredValue<i64, Mode>,
        #[data_view(schema = Bool(234))]
        pub drop: ::data_views::Field<bool>,
        #[data_view(schema = Str(478))]
        pub interface: ::data_views::Field<&'a str>,
        #[data_view(schema = Bool(235))]
        pub eligibility_forwarding: ::data_views::Field<bool>,
    }
}
