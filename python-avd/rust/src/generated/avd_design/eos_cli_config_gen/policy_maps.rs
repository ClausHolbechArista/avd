// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Pbr {
        model item (0) -> pbr::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod pbr {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            model classes("classes", 1) -> item::Classes<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Classes {
                model item (0) -> classes::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod classes {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar name("name", 0) -> &'a str;
                    scalar index("index", 1) -> i64;
                    scalar drop("drop", 2) -> bool;
                    model set("set", 3) -> item::Set<'a>;
                }
            }

            pub mod item {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Set {
                        model nexthop("nexthop", 0) -> set::Nexthop<'a>;
                    }
                }

                pub mod set {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Nexthop {
                            scalar ip_address("ip_address", 0) -> &'a str;
                            scalar recursive("recursive", 1) -> bool;
                        }
                    }
                }
            }
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Qos {
        model item (0) -> qos::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod qos {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            model classes("classes", 1) -> item::Classes<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Classes {
                model item (0) -> classes::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod classes {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar name("name", 0) -> &'a str;
                    model set("set", 1) -> item::Set<'a>;
                    model police("police", 2) -> item::Police<'a>;
                }
            }

            pub mod item {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Set {
                        scalar cos("cos", 0) -> i64;
                        scalar dscp("dscp", 1) -> &'a str;
                        scalar traffic_class("traffic_class", 2) -> i64;
                        scalar drop_precedence("drop_precedence", 3) -> i64;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Police {
                        scalar rate("rate", 0) -> i64;
                        scalar rate_unit("rate_unit", 1) -> &'a str;
                        scalar rate_burst_size("rate_burst_size", 2) -> i64;
                        scalar rate_burst_size_unit("rate_burst_size_unit", 3) -> &'a str;
                        model action("action", 4) -> police::Action<'a>;
                        scalar higher_rate("higher_rate", 5) -> i64;
                        scalar higher_rate_unit("higher_rate_unit", 6) -> &'a str;
                        scalar higher_rate_burst_size("higher_rate_burst_size", 7) -> i64;
                        scalar higher_rate_burst_size_unit("higher_rate_burst_size_unit", 8) -> &'a str;
                    }
                }

                pub mod police {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Action {
                            scalar field_type("type", 0) -> &'a str;
                            scalar dscp_value("dscp_value", 1) -> &'a str;
                        }
                    }
                }
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct CoppSystemPolicy {
        model classes("classes", 0) -> copp_system_policy::Classes<'a>;
    }
}

pub mod copp_system_policy {

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Classes {
            model item (0) -> classes::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod classes {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                scalar shape("shape", 1) -> i64;
                scalar bandwidth("bandwidth", 2) -> i64;
                scalar rate_unit("rate_unit", 3) -> &'a str;
            }
        }
    }
}
