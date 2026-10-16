// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(python(name = "EOSConfig.MacAddressTable.NotificationHostFlap", schema = Dict(533)))]
pub struct NotificationHostFlap<'a, Mode> {
    #[data_view(schema = Bool(7))]
    pub logging: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(532))]
    pub detection: ::validated_data::Field<notification_host_flap::Detection<'a, Mode>>,
}

pub mod notification_host_flap {

    #[::validated_data::data_view(python(name = "EOSConfig.MacAddressTable.NotificationHostFlap.Detection", schema = Dict(532)))]
    pub struct Detection<'a, Mode> {
        #[data_view(schema = Int(223))]
        pub window: ::validated_data::Field<i64>,
        #[data_view(schema = Int(224))]
        pub moves: ::validated_data::Field<i64>,
    }
}

#[::validated_data::data_view(list, python(name = "EOSConfig.MacAddressTable.StaticEntries", schema = List(226)))]
pub struct StaticEntries<'a, Mode> (::validated_data::Field<static_entries::Item<'a, Mode>>);

pub mod static_entries {

    #[::validated_data::data_view(python(name = "EOSConfig.MacAddressTable.StaticEntriesItem", schema = Dict(534)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(477))]
        pub mac_address: ::validated_data::RequiredValue<&'a str, Mode>,
        #[data_view(schema = Int(225))]
        pub vlan: ::validated_data::RequiredValue<i64, Mode>,
        #[data_view(schema = Bool(234))]
        pub drop: ::validated_data::Field<bool>,
        #[data_view(schema = Str(478))]
        pub interface: ::validated_data::Field<&'a str>,
        #[data_view(schema = Bool(235))]
        pub eligibility_forwarding: ::validated_data::Field<bool>,
    }
}
