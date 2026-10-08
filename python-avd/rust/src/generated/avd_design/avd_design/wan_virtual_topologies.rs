// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


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
            scalar policy("policy", 1) -> &'a str;
            scalar wan_vni("wan_vni", 2) -> i64;
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct ControlPlaneVirtualTopology {
        scalar name("name", 0) -> &'a str;
        scalar application_profile("application_profile", 1) -> &'a str;
        scalar traffic_class("traffic_class", 2) -> i64;
        scalar dscp("dscp", 3) -> i64;
        scalar lowest_hop_count("lowest_hop_count", 4) -> bool;
        model constraints("constraints", 5) -> control_plane_virtual_topology::Constraints<'a>;
        model outlier_elimination("outlier_elimination", 6) -> super::super::eos_cli_config_gen::router_adaptive_virtual_topology::profiles::item::OutlierElimination<'a>;
        model metric_order("metric_order", 7) -> super::super::eos_cli_config_gen::router_adaptive_virtual_topology::profiles::item::MetricOrder<'a>;
        model path_groups("path_groups", 8) -> control_plane_virtual_topology::PathGroups<'a>;
        model internet_exit("internet_exit", 9) -> control_plane_virtual_topology::InternetExit<'a>;
    }
}

pub mod control_plane_virtual_topology {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Constraints {
            scalar jitter("jitter", 0) -> i64;
            scalar latency("latency", 1) -> i64;
            scalar loss_rate("loss_rate", 2) -> &'a str;
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct PathGroups {
            model item (0) -> path_groups::Item<'a>;
        }
    }

    pub mod path_groups {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                model names("names", 0) -> item::Names<'a>;
                scalar preference("preference", 1) -> &'a str;
            }
        }

        pub mod item {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Names {
                    scalar item (0) -> &'a str;
                }
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct InternetExit {
            scalar policy("policy", 0) -> &'a str;
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Policies {
        model item (0) -> policies::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod policies {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            model application_virtual_topologies("application_virtual_topologies", 1) -> item::ApplicationVirtualTopologies<'a>;
            model default_virtual_topology("default_virtual_topology", 2) -> item::DefaultVirtualTopology<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct ApplicationVirtualTopologies {
                model item (0) -> application_virtual_topologies::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod application_virtual_topologies {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar application_profile("application_profile", 0) -> &'a str;
                    scalar name("name", 1) -> &'a str;
                    scalar id("id", 2) -> i64;
                    scalar traffic_class("traffic_class", 3) -> i64;
                    scalar dscp("dscp", 4) -> i64;
                    scalar lowest_hop_count("lowest_hop_count", 5) -> bool;
                    model constraints("constraints", 6) -> item::Constraints<'a>;
                    model outlier_elimination("outlier_elimination", 7) -> super::super::super::super::super::eos_cli_config_gen::router_adaptive_virtual_topology::profiles::item::OutlierElimination<'a>;
                    model metric_order("metric_order", 8) -> super::super::super::super::super::eos_cli_config_gen::router_adaptive_virtual_topology::profiles::item::MetricOrder<'a>;
                    model path_groups("path_groups", 9) -> item::PathGroups<'a>;
                    model internet_exit("internet_exit", 10) -> item::InternetExit<'a>;
                }
            }

            pub mod item {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Constraints {
                        scalar jitter("jitter", 0) -> i64;
                        scalar latency("latency", 1) -> i64;
                        scalar loss_rate("loss_rate", 2) -> &'a str;
                    }
                }

                ::validation::define_archive_list_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct PathGroups {
                        model item (0) -> path_groups::Item<'a>;
                    }
                }

                pub mod path_groups {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Item {
                            model names("names", 0) -> item::Names<'a>;
                            scalar preference("preference", 1) -> &'a str;
                        }
                    }

                    pub mod item {

                        ::validation::define_archive_list_view! {
                            #[derive(Clone, Copy, Debug)]
                            pub struct Names {
                                scalar item (0) -> &'a str;
                            }
                        }
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct InternetExit {
                        scalar policy("policy", 0) -> &'a str;
                    }
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct DefaultVirtualTopology {
                scalar name("name", 0) -> &'a str;
                scalar drop_unmatched("drop_unmatched", 1) -> bool;
                scalar traffic_class("traffic_class", 2) -> i64;
                scalar dscp("dscp", 3) -> i64;
                scalar lowest_hop_count("lowest_hop_count", 4) -> bool;
                model constraints("constraints", 5) -> default_virtual_topology::Constraints<'a>;
                model outlier_elimination("outlier_elimination", 6) -> super::super::super::super::eos_cli_config_gen::router_adaptive_virtual_topology::profiles::item::OutlierElimination<'a>;
                model metric_order("metric_order", 7) -> super::super::super::super::eos_cli_config_gen::router_adaptive_virtual_topology::profiles::item::MetricOrder<'a>;
                model path_groups("path_groups", 8) -> default_virtual_topology::PathGroups<'a>;
                model internet_exit("internet_exit", 9) -> default_virtual_topology::InternetExit<'a>;
            }
        }

        pub mod default_virtual_topology {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Constraints {
                    scalar jitter("jitter", 0) -> i64;
                    scalar latency("latency", 1) -> i64;
                    scalar loss_rate("loss_rate", 2) -> &'a str;
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct PathGroups {
                    model item (0) -> path_groups::Item<'a>;
                }
            }

            pub mod path_groups {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        model names("names", 0) -> item::Names<'a>;
                        scalar preference("preference", 1) -> &'a str;
                    }
                }

                pub mod item {

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Names {
                            scalar item (0) -> &'a str;
                        }
                    }
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct InternetExit {
                    scalar policy("policy", 0) -> &'a str;
                }
            }
        }
    }
}
