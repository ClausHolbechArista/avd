// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(python(name = "AVDDesign.SshSettings", schema = Dict(1761)))]
pub struct SshSettings<'a, Mode> {
    #[data_view(schema = Bool(658))]
    pub enabled: ::validated_data::Field<bool>,
    #[data_view(schema = List(778))]
    pub vrfs: ::validated_data::Field<Vrfs<'a, Mode>>,
    #[data_view(schema = Int(235))]
    pub idle_timeout: ::validated_data::Field<i64>,
    #[data_view(schema = List(779))]
    pub client_vrfs: ::validated_data::Field<ClientVrfs<'a, Mode>>,
}

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.SshSettings.Vrfs", schema = List(778), item_name = "AVDDesign.SshSettings.VrfsItem"))]
pub struct Vrfs<'a, Mode> (::validated_data::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::validated_data::data_view(python(name = "AVDDesign.SshSettings.VrfsItemBase", schema = Dict(1759)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(1639))]
        pub name: ::validated_data::Field<&'a str>,
        #[data_view(schema = Bool(659))]
        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
        #[data_view(schema = Str(1053))]
        pub ipv4_acl: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1054))]
        pub ipv6_acl: ::validated_data::Field<&'a str>,
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.SshSettings.ClientVrfs", schema = List(779), item_name = "AVDDesign.SshSettings.ClientVrfsItem"))]
pub struct ClientVrfs<'a, Mode> (::validated_data::Field<client_vrfs::Item<'a, Mode>>);

pub mod client_vrfs {

    #[::validated_data::data_view(python(name = "AVDDesign.SshSettings.ClientVrfsItemBase", schema = Dict(1760)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(1640))]
        pub name: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1641))]
        pub source_interface: ::validated_data::Field<&'a str>,
    }
}
