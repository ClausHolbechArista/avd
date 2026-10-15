// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.SshSettings.Vrfs", item_name = "AVDDesign.SshSettings.VrfsItem"))]
pub struct Vrfs<'a, Mode> (::validated_data::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::validated_data::data_view(python(name = "AVDDesign.SshSettings.VrfsItemBase"))]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
        pub ipv4_acl: ::validated_data::Field<&'a str>,
        pub ipv6_acl: ::validated_data::Field<&'a str>,
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.SshSettings.ClientVrfs", item_name = "AVDDesign.SshSettings.ClientVrfsItem"))]
pub struct ClientVrfs<'a, Mode> (::validated_data::Field<client_vrfs::Item<'a, Mode>>);

pub mod client_vrfs {

    #[::validated_data::data_view(python(name = "AVDDesign.SshSettings.ClientVrfsItemBase"))]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub source_interface: ::validated_data::Field<&'a str>,
    }
}
