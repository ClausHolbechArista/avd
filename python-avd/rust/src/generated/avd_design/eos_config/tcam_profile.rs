// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct TcamProfile<'a, Mode> {
    pub system: ::data_views::Field<&'a str>,
    pub profiles: ::data_views::Field<Profiles<'a, Mode>>,
}

#[::data_views::data_view(indexed_list, primary_key(name), python(name = "EOSConfig.TcamProfile.Profiles", schema = List(509), item_name = "EOSConfig.TcamProfile.ProfilesItem"))]
pub struct Profiles<'a, Mode> (::data_views::Field<profiles::Item<'a, Mode>>);

pub mod profiles {

    #[::data_views::data_view(python(name = "EOSConfig.TcamProfile.ProfilesItemBase", schema = Dict(1281)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(996))]
        pub name: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(997))]
        pub config: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(998))]
        pub source: ::data_views::Field<&'a str>,
    }
}
