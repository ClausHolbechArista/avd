// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct ClassMaps<'a, Mode> {
    pub pbr: ::data_views::Field<Pbr<'a, Mode>>,
    pub qos: ::data_views::Field<Qos<'a, Mode>>,
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Pbr<'a, Mode> (::data_views::Field<pbr::Item<'a, Mode>>);

pub mod pbr {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub ip: ::data_views::Field<item::Ip<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view]
        pub struct Ip<'a, Mode> {
            pub access_group: ::data_views::Field<&'a str>,
        }
    }
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Qos<'a, Mode> (::data_views::Field<qos::Item<'a, Mode>>);

pub mod qos {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub vlan: ::data_views::Field<&'a str>,
        pub cos: ::data_views::Field<&'a str>,
        pub ip: ::data_views::Field<item::Ip<'a, Mode>>,
        pub ipv6: ::data_views::Field<item::Ipv6<'a, Mode>>,
        pub dscp: ::data_views::Field<&'a str>,
        pub ecn: ::data_views::Field<&'a str>,
    }

    pub mod item {

        #[::data_views::data_view]
        pub struct Ip<'a, Mode> {
            pub access_group: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view]
        pub struct Ipv6<'a, Mode> {
            pub access_group: ::data_views::Field<&'a str>,
        }
    }
}
