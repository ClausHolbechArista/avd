// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "AVDDesign.SshSettings", schema = Dict(1761)))]
pub struct SshSettings<'a, Mode> {
    #[data_view(schema = Bool(658))]
    pub enabled: ::data_views::Field<bool>,
    #[data_view(schema = List(778))]
    pub vrfs: ::data_views::Field<Vrfs<'a, Mode>>,
    #[data_view(schema = Int(235))]
    pub idle_timeout: ::data_views::Field<i64>,
    #[data_view(schema = List(779))]
    pub client_vrfs: ::data_views::Field<ClientVrfs<'a, Mode>>,
}

#[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.SshSettings.Vrfs", schema = List(778), item_name = "AVDDesign.SshSettings.VrfsItem"))]
pub struct Vrfs<'a, Mode> (::data_views::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::data_views::data_view(python(name = "AVDDesign.SshSettings.VrfsItemBase", schema = Dict(1759)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(1639))]
        pub name: ::data_views::Field<&'a str>,
        #[data_view(schema = Bool(659))]
        pub enabled: ::data_views::RequiredValue<bool, Mode>,
        #[data_view(schema = Str(1053))]
        pub ipv4_acl: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1054))]
        pub ipv6_acl: ::data_views::Field<&'a str>,
    }
}

#[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.SshSettings.ClientVrfs", schema = List(779), item_name = "AVDDesign.SshSettings.ClientVrfsItem"))]
pub struct ClientVrfs<'a, Mode> (::data_views::Field<client_vrfs::Item<'a, Mode>>);

pub mod client_vrfs {

    #[::data_views::data_view(python(name = "AVDDesign.SshSettings.ClientVrfsItemBase", schema = Dict(1760)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(1640))]
        pub name: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1641))]
        pub source_interface: ::data_views::Field<&'a str>,
    }
}
