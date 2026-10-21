// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "AVDDesign.SnmpSettings", schema = Dict(1749)))]
pub struct SnmpSettings<'a, Mode> {
    #[data_view(schema = Str(948))]
    pub contact: ::data_views::Field<&'a str>,
    #[data_view(schema = Bool(644))]
    pub location: ::data_views::Field<bool>,
    #[data_view(schema = Str(1627))]
    pub location_template: ::data_views::Field<&'a str>,
    #[data_view(schema = List(772))]
    pub vrfs: ::data_views::Field<Vrfs<'a, Mode>>,
    #[data_view(schema = Bool(646))]
    pub compute_local_engineid: ::data_views::Field<bool>,
    #[data_view(schema = Str(1630))]
    pub compute_local_engineid_source: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1631))]
    pub local_engineid_ip: ::data_views::Field<&'a str>,
    #[data_view(schema = Bool(647))]
    pub compute_v3_user_localized_key: ::data_views::Field<bool>,
    #[data_view(schema = List(773))]
    pub users: ::data_views::Field<Users<'a, Mode>>,
    #[data_view(schema = List(774))]
    pub hosts: ::data_views::Field<Hosts<'a, Mode>>,
    #[data_view(schema = List(775))]
    pub communities: ::data_views::Field<Communities<'a, Mode>>,
    #[data_view(schema = List(491))]
    pub views: ::data_views::Field<Views<'a, Mode>>,
    #[data_view(schema = List(492))]
    pub groups: ::data_views::Field<Groups<'a, Mode>>,
    #[data_view(schema = Dict(1243))]
    pub traps: ::data_views::Field<super::super::eos_config::snmp_server::Traps<'a, Mode>>,
}

#[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.SnmpSettings.Vrfs", schema = List(772), item_name = "AVDDesign.SnmpSettings.VrfsItem"))]
pub struct Vrfs<'a, Mode> (::data_views::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::data_views::data_view(python(name = "AVDDesign.SnmpSettings.VrfsItemBase", schema = Dict(1744)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(1628))]
        pub name: ::data_views::Field<&'a str>,
        #[data_view(schema = Bool(645))]
        pub enable: ::data_views::Field<bool>,
        #[data_view(schema = Str(1629))]
        pub source_interface: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1053))]
        pub ipv4_acl: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1054))]
        pub ipv6_acl: ::data_views::Field<&'a str>,
    }
}

#[::data_views::data_view(list, python(name = "AVDDesign.SnmpSettings.Users", schema = List(773)))]
pub struct Users<'a, Mode> (::data_views::Field<users::Item<'a, Mode>>);

pub mod users {

    #[::data_views::data_view(python(name = "AVDDesign.SnmpSettings.UsersItem", schema = Dict(1745)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(451))]
        pub name: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(183))]
        pub group: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(954))]
        pub version: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1632))]
        pub auth: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1633))]
        pub auth_passphrase: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1634), rename = "priv")]
        pub field_priv: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1635))]
        pub priv_passphrase: ::data_views::Field<&'a str>,
    }
}

#[::data_views::data_view(list, python(name = "AVDDesign.SnmpSettings.Hosts", schema = List(774)))]
pub struct Hosts<'a, Mode> (::data_views::Field<hosts::Item<'a, Mode>>);

pub mod hosts {

    #[::data_views::data_view(python(name = "AVDDesign.SnmpSettings.HostsItem", schema = Dict(1746)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(970))]
        pub host: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1636))]
        pub vrf: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(971))]
        pub version: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(972))]
        pub community: ::data_views::Field<&'a str>,
        #[data_view(schema = List(494))]
        pub users: ::data_views::Field<item::Users<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view(list, python(name = "AVDDesign.SnmpSettings.HostsItem.Users", schema = List(494)))]
        pub struct Users<'a, Mode> (::data_views::Field<users::Item<'a, Mode>>);

        pub mod users {

            #[::data_views::data_view(python(name = "AVDDesign.SnmpSettings.HostsItem.UsersItem", schema = Dict(1240)))]
            pub struct Item<'a, Mode> {
                #[data_view(schema = Str(3))]
                pub username: ::data_views::Field<&'a str>,
                #[data_view(schema = Str(955))]
                pub authentication_level: ::data_views::Field<&'a str>,
            }
        }
    }
}

#[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.SnmpSettings.Communities", schema = List(775), item_name = "AVDDesign.SnmpSettings.CommunitiesItem"))]
pub struct Communities<'a, Mode> (::data_views::Field<communities::Item<'a, Mode>>);

pub mod communities {

    #[::data_views::data_view(python(name = "AVDDesign.SnmpSettings.CommunitiesItemBase", schema = Dict(1748)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(950))]
        pub name: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(951))]
        pub access: ::data_views::Field<&'a str>,
        #[data_view(schema = Dict(1747))]
        pub access_list_ipv4: ::data_views::Field<item::AccessListIpv4<'a, Mode>>,
        #[data_view(schema = Str(1637))]
        pub ipv4_standard_acl: ::data_views::Field<&'a str>,
        #[data_view(schema = Dict(1232))]
        pub access_list_ipv6: ::data_views::Field<item::AccessListIpv6<'a, Mode>>,
        #[data_view(schema = Str(3))]
        pub view: ::data_views::Field<&'a str>,
    }

    pub mod item {

        #[::data_views::data_view(python(name = "AVDDesign.SnmpSettings.CommunitiesItemBase.AccessListIpv4", schema = Dict(1747)))]
        pub struct AccessListIpv4<'a, Mode> {
            #[data_view(schema = Str(952))]
            pub name: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view(python(name = "AVDDesign.SnmpSettings.CommunitiesItemBase.AccessListIpv6", schema = Dict(1232)))]
        pub struct AccessListIpv6<'a, Mode> {
            #[data_view(schema = Str(217))]
            pub name: ::data_views::Field<&'a str>,
        }
    }
}

#[::data_views::data_view(list, python(name = "AVDDesign.SnmpSettings.Views", schema = List(491)))]
pub struct Views<'a, Mode> (::data_views::Field<views::Item<'a, Mode>>);

pub mod views {

    #[::data_views::data_view(python(name = "AVDDesign.SnmpSettings.ViewsItem", schema = Dict(1237)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(953))]
        pub name: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(3))]
        pub mib_family_name: ::data_views::Field<&'a str>,
        #[data_view(schema = Bool(7))]
        pub included: ::data_views::Field<bool>,
    }
}

#[::data_views::data_view(list, python(name = "AVDDesign.SnmpSettings.Groups", schema = List(492)))]
pub struct Groups<'a, Mode> (::data_views::Field<groups::Item<'a, Mode>>);

pub mod groups {

    #[::data_views::data_view(python(name = "AVDDesign.SnmpSettings.GroupsItem", schema = Dict(1238)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(183))]
        pub name: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(954))]
        pub version: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(955))]
        pub authentication: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(956))]
        pub read: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(957))]
        pub write: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(958))]
        pub notify: ::data_views::Field<&'a str>,
    }
}
