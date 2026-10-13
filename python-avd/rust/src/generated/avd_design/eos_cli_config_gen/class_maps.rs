// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Pbr<'a, Mode> (::validated_data::Field<pbr::Item<'a, Mode>>);

pub mod pbr {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub ip: ::validated_data::Field<item::Ip<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view]
        pub struct Ip<'a, Mode> {
            pub access_group: ::validated_data::Field<&'a str>,
        }
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Qos<'a, Mode> (::validated_data::Field<qos::Item<'a, Mode>>);

pub mod qos {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub vlan: ::validated_data::Field<&'a str>,
        pub cos: ::validated_data::Field<&'a str>,
        pub ip: ::validated_data::Field<item::Ip<'a, Mode>>,
        pub ipv6: ::validated_data::Field<item::Ipv6<'a, Mode>>,
        pub dscp: ::validated_data::Field<&'a str>,
        pub ecn: ::validated_data::Field<&'a str>,
    }

    pub mod item {

        #[::validated_data::data_view]
        pub struct Ip<'a, Mode> {
            pub access_group: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view]
        pub struct Ipv6<'a, Mode> {
            pub access_group: ::validated_data::Field<&'a str>,
        }
    }
}
