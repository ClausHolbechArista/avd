// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Trident {
        scalar forwarding_table_partition("forwarding_table_partition", 0) -> &'a str;
        model l3("l3", 1) -> trident::L3<'a>;
        model mmu("mmu", 2) -> trident::Mmu<'a>;
    }
}

pub mod trident {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct L3 {
            scalar routing_mac_address_per_vlan("routing_mac_address_per_vlan", 0) -> bool;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Mmu {
            scalar active_profile("active_profile", 0) -> &'a str;
            model headroom_pool("headroom_pool", 1) -> mmu::HeadroomPool<'a>;
            model queue_profiles("queue_profiles", 2) -> mmu::QueueProfiles<'a>;
        }
    }

    pub mod mmu {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct HeadroomPool {
                scalar unit("unit", 0) -> &'a str;
                scalar limit("limit", 1) -> i64;
            }
        }

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct QueueProfiles {
                model item (0) -> queue_profiles::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod queue_profiles {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar name("name", 0) -> &'a str;
                    model ingress("ingress", 1) -> item::Ingress<'a>;
                    model multicast_queues("multicast_queues", 2) -> item::MulticastQueues<'a>;
                    model unicast_queues("unicast_queues", 3) -> item::UnicastQueues<'a>;
                }
            }

            pub mod item {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Ingress {
                        model priority_groups("priority_groups", 0) -> ingress::PriorityGroups<'a>;
                        scalar threshold("threshold", 1) -> &'a str;
                        model reserved("reserved", 2) -> ingress::Reserved<'a>;
                        model headroom("headroom", 3) -> ingress::Headroom<'a>;
                        scalar resume("resume", 4) -> i64;
                    }
                }

                pub mod ingress {

                    ::validation::define_archive_indexed_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct PriorityGroups {
                            model item (0) -> priority_groups::Item<'a>;
                            primary_key_fields: [0];
                        }
                    }

                    pub mod priority_groups {

                        ::validation::define_archive_dict_view! {
                            #[derive(Clone, Copy, Debug)]
                            pub struct Item {
                                scalar id("id", 0) -> i64;
                                scalar threshold("threshold", 1) -> &'a str;
                                model reserved("reserved", 2) -> item::Reserved<'a>;
                            }
                        }

                        pub mod item {

                            ::validation::define_archive_dict_view! {
                                #[derive(Clone, Copy, Debug)]
                                pub struct Reserved {
                                    scalar unit("unit", 0) -> &'a str;
                                    scalar memory("memory", 1) -> i64;
                                }
                            }
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Reserved {
                            scalar unit("unit", 0) -> &'a str;
                            scalar memory("memory", 1) -> i64;
                        }
                    }

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Headroom {
                            scalar unit("unit", 0) -> &'a str;
                            scalar memory("memory", 1) -> i64;
                        }
                    }
                }

                ::validation::define_archive_indexed_list_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct MulticastQueues {
                        model item (0) -> multicast_queues::Item<'a>;
                        primary_key_fields: [0];
                    }
                }

                pub mod multicast_queues {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Item {
                            scalar id("id", 0) -> i64;
                            scalar unit("unit", 1) -> &'a str;
                            scalar reserved("reserved", 2) -> i64;
                            scalar threshold("threshold", 3) -> &'a str;
                            model drop("drop", 4) -> item::Drop<'a>;
                        }
                    }

                    pub mod item {

                        ::validation::define_archive_dict_view! {
                            #[derive(Clone, Copy, Debug)]
                            pub struct Drop {
                                scalar precedence("precedence", 0) -> i64;
                                scalar threshold("threshold", 1) -> &'a str;
                            }
                        }
                    }
                }

                ::validation::define_archive_indexed_list_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct UnicastQueues {
                        model item (0) -> unicast_queues::Item<'a>;
                        primary_key_fields: [0];
                    }
                }

                pub mod unicast_queues {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Item {
                            scalar id("id", 0) -> i64;
                            scalar unit("unit", 1) -> &'a str;
                            scalar reserved("reserved", 2) -> i64;
                            scalar threshold("threshold", 3) -> &'a str;
                            model drop("drop", 4) -> item::Drop<'a>;
                        }
                    }

                    pub mod item {

                        ::validation::define_archive_dict_view! {
                            #[derive(Clone, Copy, Debug)]
                            pub struct Drop {
                                scalar precedence("precedence", 0) -> i64;
                                scalar threshold("threshold", 1) -> &'a str;
                            }
                        }
                    }
                }
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Sand {
        model qos_maps("qos_maps", 0) -> sand::QosMaps<'a>;
        model lag("lag", 1) -> sand::Lag<'a>;
        scalar forwarding_mode("forwarding_mode", 2) -> &'a str;
        model multicast_replication("multicast_replication", 3) -> sand::MulticastReplication<'a>;
        scalar mdb_profile("mdb_profile", 4) -> &'a str;
    }
}

pub mod sand {

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct QosMaps {
            model item (0) -> qos_maps::Item<'a>;
        }
    }

    pub mod qos_maps {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar traffic_class("traffic_class", 0) -> i64;
                scalar to_network_qos("to_network_qos", 1) -> i64;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Lag {
            scalar hardware_only("hardware_only", 0) -> bool;
            scalar mode("mode", 1) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct MulticastReplication {
            scalar default("default", 0) -> &'a str;
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Sfe {
        scalar data_plane_cpu_allocation_max("data_plane_cpu_allocation_max", 0) -> i64;
        model interface("interface", 1) -> sfe::Interface<'a>;
    }
}

pub mod sfe {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Interface {
            model profiles("profiles", 0) -> interface::Profiles<'a>;
            scalar interface_profile("interface_profile", 1) -> &'a str;
        }
    }

    pub mod interface {

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Profiles {
                model item (0) -> profiles::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod profiles {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar name("name", 0) -> &'a str;
                    model interfaces("interfaces", 1) -> item::Interfaces<'a>;
                }
            }

            pub mod item {

                ::validation::define_archive_indexed_list_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Interfaces {
                        model item (0) -> interfaces::Item<'a>;
                        primary_key_fields: [0];
                    }
                }

                pub mod interfaces {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Item {
                            scalar name("name", 0) -> &'a str;
                            model rx_queue("rx_queue", 1) -> item::RxQueue<'a>;
                        }
                    }

                    pub mod item {

                        ::validation::define_archive_dict_view! {
                            #[derive(Clone, Copy, Debug)]
                            pub struct RxQueue {
                                scalar count("count", 0) -> i64;
                                scalar worker("worker", 1) -> &'a str;
                                scalar mode("mode", 2) -> &'a str;
                            }
                        }
                    }
                }
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Fap {
        model buffering_egress("buffering_egress", 0) -> fap::BufferingEgress<'a>;
        model voq("voq", 1) -> fap::Voq<'a>;
    }
}

pub mod fap {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct BufferingEgress {
            scalar profile("profile", 0) -> &'a str;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Voq {
            scalar credit_rates_unified("credit_rates_unified", 0) -> bool;
        }
    }
}
