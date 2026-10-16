// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.ManagementEapi.Vrfs", schema = List(675), item_name = "AVDDesign.ManagementEapi.VrfsItem"))]
pub struct Vrfs<'a, Mode> (::validated_data::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::validated_data::data_view(python(name = "AVDDesign.ManagementEapi.VrfsItemBase", schema = Dict(1621)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(1452))]
        pub name: ::validated_data::Field<&'a str>,
        #[data_view(schema = Bool(553))]
        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
        #[data_view(schema = Str(1053))]
        pub ipv4_acl: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1054))]
        pub ipv6_acl: ::validated_data::Field<&'a str>,
    }
}
