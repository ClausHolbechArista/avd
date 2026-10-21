// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(python(name = "AVDDesign.ManagementEapi", schema = Dict(1622)))]
pub struct ManagementEapi<'a, Mode> {
    #[data_view(schema = Bool(552))]
    pub enabled: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(7))]
    pub enable_http: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(37))]
    pub enable_https: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(7))]
    pub default_services: ::validated_data::Field<bool>,
    #[data_view(schema = List(675))]
    pub vrfs: ::validated_data::Field<Vrfs<'a, Mode>>,
}

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
