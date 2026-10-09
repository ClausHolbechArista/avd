// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Ipv4<'a, Mode> {
    pub activity_polling_interval: ::validated_data::Field<i64>,
    pub counters: ::validated_data::Field<ipv4::Counters<'a, Mode>>,
    pub routing: ::validated_data::Field<bool>,
    pub multipath: ::validated_data::Field<&'a str>,
    pub software_forwarding: ::validated_data::Field<&'a str>,
    pub rpf: ::validated_data::Field<ipv4::Rpf<'a, Mode>>,
}

pub mod ipv4 {

    #[::validated_data::data_view]
    pub struct Counters<'a, Mode> {
        pub rate_period_decay: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct Rpf<'a, Mode> {
        pub routes: ::validated_data::Field<rpf::Routes<'a, Mode>>,
    }

    pub mod rpf {

        #[::validated_data::data_view(list)]
        pub struct Routes<'a, Mode> (::validated_data::Field<routes::Item<'a, Mode>>);

        pub mod routes {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub source_prefix: ::validated_data::RequiredValue<&'a str, Mode>,
                pub destinations: ::validated_data::RequiredValue<item::Destinations<'a, Mode>, Mode>,
            }

            pub mod item {

                #[::validated_data::data_view(list)]
                pub struct Destinations<'a, Mode> (::validated_data::Field<destinations::Item<'a, Mode>>);

                pub mod destinations {

                    #[::validated_data::data_view]
                    pub struct Item<'a, Mode> {
                        pub nexthop: ::validated_data::RequiredValue<&'a str, Mode>,
                        pub distance: ::validated_data::Field<i64>,
                    }
                }
            }
        }
    }
}

#[::validated_data::data_view]
pub struct Ipv6<'a, Mode> {
    pub activity_polling_interval: ::validated_data::Field<i64>,
    pub routing: ::validated_data::Field<bool>,
    pub software_forwarding: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Vrfs<'a, Mode> (::validated_data::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub ipv4: ::validated_data::Field<item::Ipv4<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view]
        pub struct Ipv4<'a, Mode> {
            pub routing: ::validated_data::Field<bool>,
        }
    }
}
