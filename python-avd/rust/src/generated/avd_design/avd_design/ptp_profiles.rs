// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(profile), python(name = "AVDDesign.PtpProfiles", schema = List(769), item_name = "AVDDesign.PtpProfilesItem"))]
pub struct PtpProfiles<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view(python(name = "AVDDesign.PtpProfilesItemBase", schema = Dict(1734)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(1618))]
    pub profile: ::data_views::Field<&'a str>,
    #[data_view(schema = Dict(1732))]
    pub announce: ::data_views::Field<item::Announce<'a, Mode>>,
    #[data_view(schema = Int(627))]
    pub delay_req: ::data_views::Field<i64>,
    #[data_view(schema = Dict(1733))]
    pub sync_message: ::data_views::Field<item::SyncMessage<'a, Mode>>,
    #[data_view(schema = Str(1619))]
    pub transport: ::data_views::Field<&'a str>,
    #[data_view(schema = Dict(270))]
    pub management: ::data_views::Field<item::Management<'a, Mode>>,
}

pub mod item {

    #[::data_views::data_view(python(name = "AVDDesign.PtpProfilesItemBase.Announce", schema = Dict(1732)))]
    pub struct Announce<'a, Mode> {
        #[data_view(schema = Int(626))]
        pub interval: ::data_views::Field<i64>,
        #[data_view(schema = Int(328))]
        pub timeout: ::data_views::Field<i64>,
    }

    #[::data_views::data_view(python(name = "AVDDesign.PtpProfilesItemBase.SyncMessage", schema = Dict(1733)))]
    pub struct SyncMessage<'a, Mode> {
        #[data_view(schema = Int(628))]
        pub interval: ::data_views::Field<i64>,
    }

    #[::data_views::data_view(python(name = "AVDDesign.PtpProfilesItemBase.Management", schema = Dict(270)))]
    pub struct Management<'a, Mode> {
        #[data_view(schema = Bool(155))]
        pub drop: ::data_views::Field<bool>,
    }
}
