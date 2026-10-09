// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub profile: ::validated_data::Field<&'a str>,
    pub announce: ::validated_data::Field<item::Announce<'a, Mode>>,
    pub delay_req: ::validated_data::Field<i64>,
    pub sync_message: ::validated_data::Field<item::SyncMessage<'a, Mode>>,
    pub transport: ::validated_data::Field<&'a str>,
    pub management: ::validated_data::Field<item::Management<'a, Mode>>,
}

pub mod item {

    #[::validated_data::data_view]
    pub struct Announce<'a, Mode> {
        pub interval: ::validated_data::Field<i64>,
        pub timeout: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct SyncMessage<'a, Mode> {
        pub interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct Management<'a, Mode> {
        pub drop: ::validated_data::Field<bool>,
    }
}
