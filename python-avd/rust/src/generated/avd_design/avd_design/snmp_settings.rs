// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.SnmpSettings.Vrfs", schema = List(772), item_name = "AVDDesign.SnmpSettings.VrfsItem"))]
pub struct Vrfs<'a, Mode> (::validated_data::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::validated_data::data_view(python(name = "AVDDesign.SnmpSettings.VrfsItemBase", schema = Dict(1744)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(1628))]
        pub name: ::validated_data::Field<&'a str>,
        #[data_view(schema = Bool(645))]
        pub enable: ::validated_data::Field<bool>,
        #[data_view(schema = Str(1629))]
        pub source_interface: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1053))]
        pub ipv4_acl: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1054))]
        pub ipv6_acl: ::validated_data::Field<&'a str>,
    }
}

#[::validated_data::data_view(list, python(name = "AVDDesign.SnmpSettings.Users", schema = List(773)))]
pub struct Users<'a, Mode> (::validated_data::Field<users::Item<'a, Mode>>);

pub mod users {

    #[::validated_data::data_view(python(name = "AVDDesign.SnmpSettings.UsersItem", schema = Dict(1745)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(451))]
        pub name: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(183))]
        pub group: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(954))]
        pub version: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1632))]
        pub auth: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1633))]
        pub auth_passphrase: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1634), rename = "priv")]
        pub field_priv: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1635))]
        pub priv_passphrase: ::validated_data::Field<&'a str>,
    }
}

#[::validated_data::data_view(list, python(name = "AVDDesign.SnmpSettings.Hosts", schema = List(774)))]
pub struct Hosts<'a, Mode> (::validated_data::Field<hosts::Item<'a, Mode>>);

pub mod hosts {

    #[::validated_data::data_view(python(name = "AVDDesign.SnmpSettings.HostsItem", schema = Dict(1746)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(970))]
        pub host: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1636))]
        pub vrf: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(971))]
        pub version: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(972))]
        pub community: ::validated_data::Field<&'a str>,
        #[data_view(schema = List(494))]
        pub users: ::validated_data::Field<item::Users<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view(list, python(name = "AVDDesign.SnmpSettings.HostsItem.Users", schema = List(494)))]
        pub struct Users<'a, Mode> (::validated_data::Field<users::Item<'a, Mode>>);

        pub mod users {

            #[::validated_data::data_view(python(name = "AVDDesign.SnmpSettings.HostsItem.UsersItem", schema = Dict(1240)))]
            pub struct Item<'a, Mode> {
                #[data_view(schema = Str(3))]
                pub username: ::validated_data::Field<&'a str>,
                #[data_view(schema = Str(955))]
                pub authentication_level: ::validated_data::Field<&'a str>,
            }
        }
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.SnmpSettings.Communities", schema = List(775), item_name = "AVDDesign.SnmpSettings.CommunitiesItem"))]
pub struct Communities<'a, Mode> (::validated_data::Field<communities::Item<'a, Mode>>);

pub mod communities {

    #[::validated_data::data_view(python(name = "AVDDesign.SnmpSettings.CommunitiesItemBase", schema = Dict(1748)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(950))]
        pub name: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(951))]
        pub access: ::validated_data::Field<&'a str>,
        #[data_view(schema = Dict(1747))]
        pub access_list_ipv4: ::validated_data::Field<item::AccessListIpv4<'a, Mode>>,
        #[data_view(schema = Str(1637))]
        pub ipv4_standard_acl: ::validated_data::Field<&'a str>,
        #[data_view(schema = Dict(1232))]
        pub access_list_ipv6: ::validated_data::Field<item::AccessListIpv6<'a, Mode>>,
        #[data_view(schema = Str(3))]
        pub view: ::validated_data::Field<&'a str>,
    }

    pub mod item {

        #[::validated_data::data_view(python(name = "AVDDesign.SnmpSettings.CommunitiesItemBase.AccessListIpv4", schema = Dict(1747)))]
        pub struct AccessListIpv4<'a, Mode> {
            #[data_view(schema = Str(952))]
            pub name: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view(python(name = "AVDDesign.SnmpSettings.CommunitiesItemBase.AccessListIpv6", schema = Dict(1232)))]
        pub struct AccessListIpv6<'a, Mode> {
            #[data_view(schema = Str(217))]
            pub name: ::validated_data::Field<&'a str>,
        }
    }
}

#[::validated_data::data_view(list, python(name = "AVDDesign.SnmpSettings.Views", schema = List(491)))]
pub struct Views<'a, Mode> (::validated_data::Field<views::Item<'a, Mode>>);

pub mod views {

    #[::validated_data::data_view(python(name = "AVDDesign.SnmpSettings.ViewsItem", schema = Dict(1237)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(953))]
        pub name: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(3))]
        pub mib_family_name: ::validated_data::Field<&'a str>,
        #[data_view(schema = Bool(7))]
        pub included: ::validated_data::Field<bool>,
    }
}

#[::validated_data::data_view(list, python(name = "AVDDesign.SnmpSettings.Groups", schema = List(492)))]
pub struct Groups<'a, Mode> (::validated_data::Field<groups::Item<'a, Mode>>);

pub mod groups {

    #[::validated_data::data_view(python(name = "AVDDesign.SnmpSettings.GroupsItem", schema = Dict(1238)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(183))]
        pub name: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(954))]
        pub version: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(955))]
        pub authentication: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(956))]
        pub read: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(957))]
        pub write: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(958))]
        pub notify: ::validated_data::Field<&'a str>,
    }
}
