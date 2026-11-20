// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct RouterMulticast<'a, Mode> {
    pub ipv4: ::data_views::Field<Ipv4<'a, Mode>>,
    pub ipv6: ::data_views::Field<Ipv6<'a, Mode>>,
    pub vrfs: ::data_views::Field<Vrfs<'a, Mode>>,
}

#[::data_views::data_view]
pub struct Ipv4<'a, Mode> {
    pub activity_polling_interval: ::data_views::Field<i64>,
    pub counters: ::data_views::Field<ipv4::Counters<'a, Mode>>,
    pub routing: ::data_views::Field<bool>,
    pub multipath: ::data_views::Field<&'a str>,
    pub software_forwarding: ::data_views::Field<&'a str>,
    pub rpf: ::data_views::Field<ipv4::Rpf<'a, Mode>>,
}

pub mod ipv4 {

    #[::data_views::data_view]
    pub struct Counters<'a, Mode> {
        pub rate_period_decay: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct Rpf<'a, Mode> {
        pub routes: ::data_views::Field<rpf::Routes<'a, Mode>>,
    }

    pub mod rpf {

        #[::data_views::data_view(list)]
        pub struct Routes<'a, Mode> (::data_views::Field<routes::Item<'a, Mode>>);

        pub mod routes {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub source_prefix: ::data_views::RequiredValue<&'a str, Mode>,
                pub destinations: ::data_views::RequiredValue<item::Destinations<'a, Mode>, Mode>,
            }

            pub mod item {

                #[::data_views::data_view(list)]
                pub struct Destinations<'a, Mode> (::data_views::Field<destinations::Item<'a, Mode>>);

                pub mod destinations {

                    #[::data_views::data_view]
                    pub struct Item<'a, Mode> {
                        pub nexthop: ::data_views::RequiredValue<&'a str, Mode>,
                        pub distance: ::data_views::Field<i64>,
                    }
                }
            }
        }
    }
}

#[::data_views::data_view]
pub struct Ipv6<'a, Mode> {
    pub activity_polling_interval: ::data_views::Field<i64>,
    pub routing: ::data_views::Field<bool>,
    pub software_forwarding: ::data_views::Field<&'a str>,
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Vrfs<'a, Mode> (::data_views::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub ipv4: ::data_views::Field<item::Ipv4<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view]
        pub struct Ipv4<'a, Mode> {
            pub routing: ::data_views::Field<bool>,
        }
    }
}
