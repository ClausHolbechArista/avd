// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(indexed_list, primary_key(profile), python(name = "AVDDesign.PtpProfiles", schema = List(769), item_name = "AVDDesign.PtpProfilesItem"))]
pub struct PtpProfiles<'a, Mode> (::validated_data::Field<Item<'a, Mode>>);

#[::validated_data::data_view(python(name = "AVDDesign.PtpProfilesItemBase", schema = Dict(1734)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(1618))]
    pub profile: ::validated_data::Field<&'a str>,
    #[data_view(schema = Dict(1732))]
    pub announce: ::validated_data::Field<item::Announce<'a, Mode>>,
    #[data_view(schema = Int(627))]
    pub delay_req: ::validated_data::Field<i64>,
    #[data_view(schema = Dict(1733))]
    pub sync_message: ::validated_data::Field<item::SyncMessage<'a, Mode>>,
    #[data_view(schema = Str(1619))]
    pub transport: ::validated_data::Field<&'a str>,
    #[data_view(schema = Dict(270))]
    pub management: ::validated_data::Field<item::Management<'a, Mode>>,
}

pub mod item {

    #[::validated_data::data_view(python(name = "AVDDesign.PtpProfilesItemBase.Announce", schema = Dict(1732)))]
    pub struct Announce<'a, Mode> {
        #[data_view(schema = Int(626))]
        pub interval: ::validated_data::Field<i64>,
        #[data_view(schema = Int(328))]
        pub timeout: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.PtpProfilesItemBase.SyncMessage", schema = Dict(1733)))]
    pub struct SyncMessage<'a, Mode> {
        #[data_view(schema = Int(628))]
        pub interval: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(python(name = "AVDDesign.PtpProfilesItemBase.Management", schema = Dict(270)))]
    pub struct Management<'a, Mode> {
        #[data_view(schema = Bool(155))]
        pub drop: ::validated_data::Field<bool>,
    }
}
