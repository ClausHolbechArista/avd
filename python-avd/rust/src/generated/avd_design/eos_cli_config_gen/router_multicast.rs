// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Ipv4 {
        scalar activity_polling_interval("activity_polling_interval", 0) -> i64;
        model counters("counters", 1) -> ipv4::Counters<'a>;
        scalar routing("routing", 2) -> bool;
        scalar multipath("multipath", 3) -> &'a str;
        scalar software_forwarding("software_forwarding", 4) -> &'a str;
        model rpf("rpf", 5) -> ipv4::Rpf<'a>;
    }
}

pub mod ipv4 {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Counters {
            scalar rate_period_decay("rate_period_decay", 0) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Rpf {
            model routes("routes", 0) -> rpf::Routes<'a>;
        }
    }

    pub mod rpf {

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Routes {
                model item (0) -> routes::Item<'a>;
            }
        }

        pub mod routes {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar source_prefix("source_prefix", 0) -> &'a str;
                    model destinations("destinations", 1) -> item::Destinations<'a>;
                }
            }

            pub mod item {

                ::validation::define_archive_list_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Destinations {
                        model item (0) -> destinations::Item<'a>;
                    }
                }

                pub mod destinations {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Item {
                            scalar nexthop("nexthop", 0) -> &'a str;
                            scalar distance("distance", 1) -> i64;
                        }
                    }
                }
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Ipv6 {
        scalar activity_polling_interval("activity_polling_interval", 0) -> i64;
        scalar routing("routing", 1) -> bool;
        scalar software_forwarding("software_forwarding", 2) -> &'a str;
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Vrfs {
        model item (0) -> vrfs::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod vrfs {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            model ipv4("ipv4", 1) -> item::Ipv4<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Ipv4 {
                scalar routing("routing", 0) -> bool;
            }
        }
    }
}
