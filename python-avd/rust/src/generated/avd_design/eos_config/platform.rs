// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct Platform<'a, Mode> {
    pub trident: ::data_views::Field<Trident<'a, Mode>>,
    pub sand: ::data_views::Field<Sand<'a, Mode>>,
    pub sfe: ::data_views::Field<Sfe<'a, Mode>>,
    pub fap: ::data_views::Field<Fap<'a, Mode>>,
}

#[::data_views::data_view]
pub struct Trident<'a, Mode> {
    pub forwarding_table_partition: ::data_views::Field<&'a str>,
    pub l3: ::data_views::Field<trident::L3<'a, Mode>>,
    pub mmu: ::data_views::Field<trident::Mmu<'a, Mode>>,
}

pub mod trident {

    #[::data_views::data_view]
    pub struct L3<'a, Mode> {
        pub routing_mac_address_per_vlan: ::data_views::Field<bool>,
    }

    #[::data_views::data_view]
    pub struct Mmu<'a, Mode> {
        pub active_profile: ::data_views::Field<&'a str>,
        pub headroom_pool: ::data_views::Field<mmu::HeadroomPool<'a, Mode>>,
        pub queue_profiles: ::data_views::Field<mmu::QueueProfiles<'a, Mode>>,
    }

    pub mod mmu {

        #[::data_views::data_view]
        pub struct HeadroomPool<'a, Mode> {
            pub unit: ::data_views::Field<&'a str>,
            pub limit: ::data_views::Field<i64>,
        }

        #[::data_views::data_view(indexed_list, primary_key(name))]
        pub struct QueueProfiles<'a, Mode> (::data_views::Field<queue_profiles::Item<'a, Mode>>);

        pub mod queue_profiles {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::data_views::Field<&'a str>,
                pub ingress: ::data_views::Field<item::Ingress<'a, Mode>>,
                pub multicast_queues: ::data_views::Field<item::MulticastQueues<'a, Mode>>,
                pub unicast_queues: ::data_views::Field<item::UnicastQueues<'a, Mode>>,
            }

            pub mod item {

                #[::data_views::data_view]
                pub struct Ingress<'a, Mode> {
                    pub priority_groups: ::data_views::Field<ingress::PriorityGroups<'a, Mode>>,
                    pub threshold: ::data_views::Field<&'a str>,
                    pub reserved: ::data_views::Field<ingress::Reserved<'a, Mode>>,
                    pub headroom: ::data_views::Field<ingress::Headroom<'a, Mode>>,
                    pub resume: ::data_views::Field<i64>,
                }

                pub mod ingress {

                    #[::data_views::data_view(indexed_list, primary_key(id))]
                    pub struct PriorityGroups<'a, Mode> (::data_views::Field<priority_groups::Item<'a, Mode>>);

                    pub mod priority_groups {

                        #[::data_views::data_view]
                        pub struct Item<'a, Mode> {
                            pub id: ::data_views::Field<i64>,
                            pub threshold: ::data_views::Field<&'a str>,
                            pub reserved: ::data_views::Field<item::Reserved<'a, Mode>>,
                        }

                        pub mod item {

                            #[::data_views::data_view]
                            pub struct Reserved<'a, Mode> {
                                pub unit: ::data_views::Field<&'a str>,
                                pub memory: ::data_views::Field<i64>,
                            }
                        }
                    }

                    #[::data_views::data_view]
                    pub struct Reserved<'a, Mode> {
                        pub unit: ::data_views::Field<&'a str>,
                        pub memory: ::data_views::Field<i64>,
                    }

                    #[::data_views::data_view]
                    pub struct Headroom<'a, Mode> {
                        pub unit: ::data_views::Field<&'a str>,
                        pub memory: ::data_views::Field<i64>,
                    }
                }

                #[::data_views::data_view(indexed_list, primary_key(id))]
                pub struct MulticastQueues<'a, Mode> (::data_views::Field<multicast_queues::Item<'a, Mode>>);

                pub mod multicast_queues {

                    #[::data_views::data_view]
                    pub struct Item<'a, Mode> {
                        pub id: ::data_views::RequiredValue<i64, Mode>,
                        pub unit: ::data_views::Field<&'a str>,
                        pub reserved: ::data_views::Field<i64>,
                        pub threshold: ::data_views::Field<&'a str>,
                        pub drop: ::data_views::Field<item::Drop<'a, Mode>>,
                    }

                    pub mod item {

                        #[::data_views::data_view]
                        pub struct Drop<'a, Mode> {
                            pub precedence: ::data_views::RequiredValue<i64, Mode>,
                            pub threshold: ::data_views::RequiredValue<&'a str, Mode>,
                        }
                    }
                }

                #[::data_views::data_view(indexed_list, primary_key(id))]
                pub struct UnicastQueues<'a, Mode> (::data_views::Field<unicast_queues::Item<'a, Mode>>);

                pub mod unicast_queues {

                    #[::data_views::data_view]
                    pub struct Item<'a, Mode> {
                        pub id: ::data_views::RequiredValue<i64, Mode>,
                        pub unit: ::data_views::Field<&'a str>,
                        pub reserved: ::data_views::Field<i64>,
                        pub threshold: ::data_views::Field<&'a str>,
                        pub drop: ::data_views::Field<item::Drop<'a, Mode>>,
                    }

                    pub mod item {

                        #[::data_views::data_view]
                        pub struct Drop<'a, Mode> {
                            pub precedence: ::data_views::RequiredValue<i64, Mode>,
                            pub threshold: ::data_views::RequiredValue<&'a str, Mode>,
                        }
                    }
                }
            }
        }
    }
}

#[::data_views::data_view]
pub struct Sand<'a, Mode> {
    pub qos_maps: ::data_views::Field<sand::QosMaps<'a, Mode>>,
    pub lag: ::data_views::Field<sand::Lag<'a, Mode>>,
    pub forwarding_mode: ::data_views::Field<&'a str>,
    pub multicast_replication: ::data_views::Field<sand::MulticastReplication<'a, Mode>>,
    pub mdb_profile: ::data_views::Field<&'a str>,
}

pub mod sand {

    #[::data_views::data_view(list)]
    pub struct QosMaps<'a, Mode> (::data_views::Field<qos_maps::Item<'a, Mode>>);

    pub mod qos_maps {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub traffic_class: ::data_views::Field<i64>,
            pub to_network_qos: ::data_views::Field<i64>,
        }
    }

    #[::data_views::data_view]
    pub struct Lag<'a, Mode> {
        pub hardware_only: ::data_views::Field<bool>,
        pub mode: ::data_views::Field<&'a str>,
    }

    #[::data_views::data_view]
    pub struct MulticastReplication<'a, Mode> {
        pub default: ::data_views::Field<&'a str>,
    }
}

#[::data_views::data_view]
pub struct Sfe<'a, Mode> {
    pub data_plane_cpu_allocation_max: ::data_views::Field<i64>,
    pub interface: ::data_views::Field<sfe::Interface<'a, Mode>>,
}

pub mod sfe {

    #[::data_views::data_view]
    pub struct Interface<'a, Mode> {
        pub profiles: ::data_views::Field<interface::Profiles<'a, Mode>>,
        pub interface_profile: ::data_views::Field<&'a str>,
    }

    pub mod interface {

        #[::data_views::data_view(indexed_list, primary_key(name))]
        pub struct Profiles<'a, Mode> (::data_views::Field<profiles::Item<'a, Mode>>);

        pub mod profiles {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::data_views::Field<&'a str>,
                pub interfaces: ::data_views::Field<item::Interfaces<'a, Mode>>,
            }

            pub mod item {

                #[::data_views::data_view(indexed_list, primary_key(name))]
                pub struct Interfaces<'a, Mode> (::data_views::Field<interfaces::Item<'a, Mode>>);

                pub mod interfaces {

                    #[::data_views::data_view]
                    pub struct Item<'a, Mode> {
                        pub name: ::data_views::Field<&'a str>,
                        pub rx_queue: ::data_views::Field<item::RxQueue<'a, Mode>>,
                    }

                    pub mod item {

                        #[::data_views::data_view]
                        pub struct RxQueue<'a, Mode> {
                            pub count: ::data_views::Field<i64>,
                            pub worker: ::data_views::Field<&'a str>,
                            pub mode: ::data_views::Field<&'a str>,
                        }
                    }
                }
            }
        }
    }
}

#[::data_views::data_view]
pub struct Fap<'a, Mode> {
    pub buffering_egress: ::data_views::Field<fap::BufferingEgress<'a, Mode>>,
    pub voq: ::data_views::Field<fap::Voq<'a, Mode>>,
}

pub mod fap {

    #[::data_views::data_view]
    pub struct BufferingEgress<'a, Mode> {
        pub profile: ::data_views::Field<&'a str>,
    }

    #[::data_views::data_view]
    pub struct Voq<'a, Mode> {
        pub credit_rates_unified: ::data_views::Field<bool>,
    }
}
