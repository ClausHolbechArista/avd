// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Trident<'a, Mode> {
    pub forwarding_table_partition: ::validated_data::Field<&'a str>,
    pub l3: ::validated_data::Field<trident::L3<'a, Mode>>,
    pub mmu: ::validated_data::Field<trident::Mmu<'a, Mode>>,
}

pub mod trident {

    #[::validated_data::data_view]
    pub struct L3<'a, Mode> {
        pub routing_mac_address_per_vlan: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view]
    pub struct Mmu<'a, Mode> {
        pub active_profile: ::validated_data::Field<&'a str>,
        pub headroom_pool: ::validated_data::Field<mmu::HeadroomPool<'a, Mode>>,
        pub queue_profiles: ::validated_data::Field<mmu::QueueProfiles<'a, Mode>>,
    }

    pub mod mmu {

        #[::validated_data::data_view]
        pub struct HeadroomPool<'a, Mode> {
            pub unit: ::validated_data::Field<&'a str>,
            pub limit: ::validated_data::Field<i64>,
        }

        #[::validated_data::data_view(indexed_list, primary_key(name))]
        pub struct QueueProfiles<'a, Mode> (::validated_data::Field<queue_profiles::Item<'a, Mode>>);

        pub mod queue_profiles {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::validated_data::Field<&'a str>,
                pub ingress: ::validated_data::Field<item::Ingress<'a, Mode>>,
                pub multicast_queues: ::validated_data::Field<item::MulticastQueues<'a, Mode>>,
                pub unicast_queues: ::validated_data::Field<item::UnicastQueues<'a, Mode>>,
            }

            pub mod item {

                #[::validated_data::data_view]
                pub struct Ingress<'a, Mode> {
                    pub priority_groups: ::validated_data::Field<ingress::PriorityGroups<'a, Mode>>,
                    pub threshold: ::validated_data::Field<&'a str>,
                    pub reserved: ::validated_data::Field<ingress::Reserved<'a, Mode>>,
                    pub headroom: ::validated_data::Field<ingress::Headroom<'a, Mode>>,
                    pub resume: ::validated_data::Field<i64>,
                }

                pub mod ingress {

                    #[::validated_data::data_view(indexed_list, primary_key(id))]
                    pub struct PriorityGroups<'a, Mode> (::validated_data::Field<priority_groups::Item<'a, Mode>>);

                    pub mod priority_groups {

                        #[::validated_data::data_view]
                        pub struct Item<'a, Mode> {
                            pub id: ::validated_data::Field<i64>,
                            pub threshold: ::validated_data::Field<&'a str>,
                            pub reserved: ::validated_data::Field<item::Reserved<'a, Mode>>,
                        }

                        pub mod item {

                            #[::validated_data::data_view]
                            pub struct Reserved<'a, Mode> {
                                pub unit: ::validated_data::Field<&'a str>,
                                pub memory: ::validated_data::Field<i64>,
                            }
                        }
                    }

                    #[::validated_data::data_view]
                    pub struct Reserved<'a, Mode> {
                        pub unit: ::validated_data::Field<&'a str>,
                        pub memory: ::validated_data::Field<i64>,
                    }

                    #[::validated_data::data_view]
                    pub struct Headroom<'a, Mode> {
                        pub unit: ::validated_data::Field<&'a str>,
                        pub memory: ::validated_data::Field<i64>,
                    }
                }

                #[::validated_data::data_view(indexed_list, primary_key(id))]
                pub struct MulticastQueues<'a, Mode> (::validated_data::Field<multicast_queues::Item<'a, Mode>>);

                pub mod multicast_queues {

                    #[::validated_data::data_view]
                    pub struct Item<'a, Mode> {
                        pub id: ::validated_data::RequiredValue<i64, Mode>,
                        pub unit: ::validated_data::Field<&'a str>,
                        pub reserved: ::validated_data::Field<i64>,
                        pub threshold: ::validated_data::Field<&'a str>,
                        pub drop: ::validated_data::Field<item::Drop<'a, Mode>>,
                    }

                    pub mod item {

                        #[::validated_data::data_view]
                        pub struct Drop<'a, Mode> {
                            pub precedence: ::validated_data::RequiredValue<i64, Mode>,
                            pub threshold: ::validated_data::RequiredValue<&'a str, Mode>,
                        }
                    }
                }

                #[::validated_data::data_view(indexed_list, primary_key(id))]
                pub struct UnicastQueues<'a, Mode> (::validated_data::Field<unicast_queues::Item<'a, Mode>>);

                pub mod unicast_queues {

                    #[::validated_data::data_view]
                    pub struct Item<'a, Mode> {
                        pub id: ::validated_data::RequiredValue<i64, Mode>,
                        pub unit: ::validated_data::Field<&'a str>,
                        pub reserved: ::validated_data::Field<i64>,
                        pub threshold: ::validated_data::Field<&'a str>,
                        pub drop: ::validated_data::Field<item::Drop<'a, Mode>>,
                    }

                    pub mod item {

                        #[::validated_data::data_view]
                        pub struct Drop<'a, Mode> {
                            pub precedence: ::validated_data::RequiredValue<i64, Mode>,
                            pub threshold: ::validated_data::RequiredValue<&'a str, Mode>,
                        }
                    }
                }
            }
        }
    }
}

#[::validated_data::data_view]
pub struct Sand<'a, Mode> {
    pub qos_maps: ::validated_data::Field<sand::QosMaps<'a, Mode>>,
    pub lag: ::validated_data::Field<sand::Lag<'a, Mode>>,
    pub forwarding_mode: ::validated_data::Field<&'a str>,
    pub multicast_replication: ::validated_data::Field<sand::MulticastReplication<'a, Mode>>,
    pub mdb_profile: ::validated_data::Field<&'a str>,
}

pub mod sand {

    #[::validated_data::data_view(list)]
    pub struct QosMaps<'a, Mode> (::validated_data::Field<qos_maps::Item<'a, Mode>>);

    pub mod qos_maps {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub traffic_class: ::validated_data::Field<i64>,
            pub to_network_qos: ::validated_data::Field<i64>,
        }
    }

    #[::validated_data::data_view]
    pub struct Lag<'a, Mode> {
        pub hardware_only: ::validated_data::Field<bool>,
        pub mode: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view]
    pub struct MulticastReplication<'a, Mode> {
        pub default: ::validated_data::Field<&'a str>,
    }
}

#[::validated_data::data_view]
pub struct Sfe<'a, Mode> {
    pub data_plane_cpu_allocation_max: ::validated_data::Field<i64>,
    pub interface: ::validated_data::Field<sfe::Interface<'a, Mode>>,
}

pub mod sfe {

    #[::validated_data::data_view]
    pub struct Interface<'a, Mode> {
        pub profiles: ::validated_data::Field<interface::Profiles<'a, Mode>>,
        pub interface_profile: ::validated_data::Field<&'a str>,
    }

    pub mod interface {

        #[::validated_data::data_view(indexed_list, primary_key(name))]
        pub struct Profiles<'a, Mode> (::validated_data::Field<profiles::Item<'a, Mode>>);

        pub mod profiles {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::validated_data::Field<&'a str>,
                pub interfaces: ::validated_data::Field<item::Interfaces<'a, Mode>>,
            }

            pub mod item {

                #[::validated_data::data_view(indexed_list, primary_key(name))]
                pub struct Interfaces<'a, Mode> (::validated_data::Field<interfaces::Item<'a, Mode>>);

                pub mod interfaces {

                    #[::validated_data::data_view]
                    pub struct Item<'a, Mode> {
                        pub name: ::validated_data::Field<&'a str>,
                        pub rx_queue: ::validated_data::Field<item::RxQueue<'a, Mode>>,
                    }

                    pub mod item {

                        #[::validated_data::data_view]
                        pub struct RxQueue<'a, Mode> {
                            pub count: ::validated_data::Field<i64>,
                            pub worker: ::validated_data::Field<&'a str>,
                            pub mode: ::validated_data::Field<&'a str>,
                        }
                    }
                }
            }
        }
    }
}

#[::validated_data::data_view]
pub struct Fap<'a, Mode> {
    pub buffering_egress: ::validated_data::Field<fap::BufferingEgress<'a, Mode>>,
    pub voq: ::validated_data::Field<fap::Voq<'a, Mode>>,
}

pub mod fap {

    #[::validated_data::data_view]
    pub struct BufferingEgress<'a, Mode> {
        pub profile: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view]
    pub struct Voq<'a, Mode> {
        pub credit_rates_unified: ::validated_data::Field<bool>,
    }
}
