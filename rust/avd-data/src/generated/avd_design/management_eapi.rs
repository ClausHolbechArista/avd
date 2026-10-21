// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "AVDDesign.ManagementEapi", schema = Dict(1622), feature = "python"))]
pub struct ManagementEapi<'a, Mode> {
    #[data_view(schema = Bool(552))]
    pub enabled: ::data_views::Field<bool>,
    #[data_view(schema = Bool(7))]
    pub enable_http: ::data_views::Field<bool>,
    #[data_view(schema = Bool(37))]
    pub enable_https: ::data_views::Field<bool>,
    #[data_view(schema = Bool(7))]
    pub default_services: ::data_views::Field<bool>,
    #[data_view(schema = List(675))]
    pub vrfs: ::data_views::Field<Vrfs<'a, Mode>>,
}

#[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.ManagementEapi.Vrfs", schema = List(675), feature = "python", item_name = "AVDDesign.ManagementEapi.VrfsItem"))]
pub struct Vrfs<'a, Mode> (::data_views::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::data_views::data_view(python(name = "AVDDesign.ManagementEapi.VrfsItemBase", schema = Dict(1621), feature = "python"))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(1452))]
        pub name: ::data_views::Field<&'a str>,
        #[data_view(schema = Bool(553))]
        pub enabled: ::data_views::RequiredValue<bool, Mode>,
        #[data_view(schema = Str(1053))]
        pub ipv4_acl: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1054))]
        pub ipv6_acl: ::data_views::Field<&'a str>,
    }
}
