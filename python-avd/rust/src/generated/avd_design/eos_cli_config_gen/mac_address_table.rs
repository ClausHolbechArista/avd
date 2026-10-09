// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct NotificationHostFlap<'a, Mode> {
    pub logging: ::validated_data::Field<bool>,
    pub detection: ::validated_data::Field<notification_host_flap::Detection<'a, Mode>>,
}

pub mod notification_host_flap {

    #[::validated_data::data_view]
    pub struct Detection<'a, Mode> {
        pub window: ::validated_data::Field<i64>,
        pub moves: ::validated_data::Field<i64>,
    }
}

#[::validated_data::data_view(list)]
pub struct StaticEntries<'a, Mode> (::validated_data::Field<static_entries::Item<'a, Mode>>);

pub mod static_entries {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub mac_address: ::validated_data::RequiredValue<&'a str, Mode>,
        pub vlan: ::validated_data::RequiredValue<i64, Mode>,
        pub drop: ::validated_data::Field<bool>,
        pub interface: ::validated_data::Field<&'a str>,
        pub eligibility_forwarding: ::validated_data::Field<bool>,
    }
}
