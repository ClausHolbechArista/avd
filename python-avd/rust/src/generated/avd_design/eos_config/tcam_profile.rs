// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "EOSConfig.TcamProfile.Profiles", schema = List(509), item_name = "EOSConfig.TcamProfile.ProfilesItem"))]
pub struct Profiles<'a, Mode> (::validated_data::Field<profiles::Item<'a, Mode>>);

pub mod profiles {

    #[::validated_data::data_view(python(name = "EOSConfig.TcamProfile.ProfilesItemBase", schema = Dict(1281)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(996))]
        pub name: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(997))]
        pub config: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(998))]
        pub source: ::validated_data::Field<&'a str>,
    }
}
