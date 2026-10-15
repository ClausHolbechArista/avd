// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct CpuTrafficPolicy<'a, Mode> {
    pub vrf_all: ::validated_data::Field<cpu_traffic_policy::VrfAll<'a, Mode>>,
    pub enforcement_ip_ttl_expired: ::validated_data::Field<bool>,
    pub fragment_implicit_permit_disabled: ::validated_data::Field<bool>,
}

pub mod cpu_traffic_policy {

    #[::validated_data::data_view]
    pub struct VrfAll<'a, Mode> {
        pub name: ::validated_data::RequiredValue<&'a str, Mode>,
        pub enforcement_management: ::validated_data::Field<bool>,
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Vrfs<'a, Mode> (::validated_data::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub cpu: ::validated_data::Field<item::Cpu<'a, Mode>>,
        pub traffic_policy_input_physical: ::validated_data::Field<&'a str>,
    }

    pub mod item {

        #[::validated_data::data_view]
        pub struct Cpu<'a, Mode> {
            pub traffic_policy: ::validated_data::RequiredValue<cpu::TrafficPolicy<'a, Mode>, Mode>,
        }

        pub mod cpu {

            #[::validated_data::data_view]
            pub struct TrafficPolicy<'a, Mode> {
                pub name: ::validated_data::RequiredValue<&'a str, Mode>,
                pub enforcement_management: ::validated_data::Field<bool>,
            }
        }
    }
}

#[::validated_data::data_view]
pub struct Options<'a, Mode> {
    pub counter_per_interface: ::validated_data::Field<bool>,
    pub counter_interface_poll_interval: ::validated_data::Field<i64>,
}

#[::validated_data::data_view]
pub struct FieldSets<'a, Mode> {
    pub ipv4: ::validated_data::Field<field_sets::Ipv4<'a, Mode>>,
    pub ipv6: ::validated_data::Field<field_sets::Ipv6<'a, Mode>>,
    pub ports: ::validated_data::Field<field_sets::Ports<'a, Mode>>,
}

pub mod field_sets {

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct Ipv4<'a, Mode> (::validated_data::Field<ipv4::Item<'a, Mode>>);

    pub mod ipv4 {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub prefixes: ::validated_data::Field<item::Prefixes<'a, Mode>>,
            #[data_view(rename = "except")]
            pub field_except: ::validated_data::Field<item::Except<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view(list)]
            pub struct Prefixes<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(list)]
            pub struct Except<'a, Mode> (::validated_data::Field<&'a str>);
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct Ipv6<'a, Mode> (::validated_data::Field<ipv6::Item<'a, Mode>>);

    pub mod ipv6 {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub prefixes: ::validated_data::Field<item::Prefixes<'a, Mode>>,
            #[data_view(rename = "except")]
            pub field_except: ::validated_data::Field<item::Except<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view(list)]
            pub struct Prefixes<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(list)]
            pub struct Except<'a, Mode> (::validated_data::Field<&'a str>);
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct Ports<'a, Mode> (::validated_data::Field<ports::Item<'a, Mode>>);

    pub mod ports {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub port_range: ::validated_data::Field<&'a str>,
        }
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Policies<'a, Mode> (::validated_data::Field<policies::Item<'a, Mode>>);

pub mod policies {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub counters: ::validated_data::Field<item::Counters<'a, Mode>>,
        pub matches: ::validated_data::Field<item::Matches<'a, Mode>>,
        pub default_actions: ::validated_data::Field<item::DefaultActions<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view(list)]
        pub struct Counters<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(indexed_list, primary_key(name))]
        pub struct Matches<'a, Mode> (::validated_data::Field<matches::Item<'a, Mode>>);

        pub mod matches {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::validated_data::Field<&'a str>,
                #[data_view(rename = "type")]
                pub field_type: ::validated_data::RequiredValue<&'a str, Mode>,
                pub source: ::validated_data::Field<item::Source<'a, Mode>>,
                pub destination: ::validated_data::Field<item::Destination<'a, Mode>>,
                pub ttl: ::validated_data::Field<&'a str>,
                pub fragment: ::validated_data::Field<item::Fragment<'a, Mode>>,
                pub protocols: ::validated_data::Field<item::Protocols<'a, Mode>>,
                pub packet_type: ::validated_data::Field<item::PacketType<'a, Mode>>,
                pub actions: ::validated_data::Field<item::Actions<'a, Mode>>,
            }

            pub mod item {

                #[::validated_data::data_view]
                pub struct Source<'a, Mode> {
                    pub prefixes: ::validated_data::Field<source::Prefixes<'a, Mode>>,
                    pub prefix_lists: ::validated_data::Field<source::PrefixLists<'a, Mode>>,
                }

                pub mod source {

                    #[::validated_data::data_view(list)]
                    pub struct Prefixes<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(list)]
                    pub struct PrefixLists<'a, Mode> (::validated_data::Field<&'a str>);
                }

                #[::validated_data::data_view]
                pub struct Destination<'a, Mode> {
                    pub prefixes: ::validated_data::Field<destination::Prefixes<'a, Mode>>,
                    pub prefix_lists: ::validated_data::Field<destination::PrefixLists<'a, Mode>>,
                }

                pub mod destination {

                    #[::validated_data::data_view(list)]
                    pub struct Prefixes<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(list)]
                    pub struct PrefixLists<'a, Mode> (::validated_data::Field<&'a str>);
                }

                #[::validated_data::data_view]
                pub struct Fragment<'a, Mode> {
                    pub offset: ::validated_data::Field<&'a str>,
                }

                #[::validated_data::data_view(indexed_list, primary_key(protocol))]
                pub struct Protocols<'a, Mode> (::validated_data::Field<protocols::Item<'a, Mode>>);

                pub mod protocols {

                    #[::validated_data::data_view]
                    pub struct Item<'a, Mode> {
                        pub protocol: ::validated_data::Field<&'a str>,
                        pub src_port: ::validated_data::Field<&'a str>,
                        pub dst_port: ::validated_data::Field<&'a str>,
                        pub src_field: ::validated_data::Field<&'a str>,
                        pub dst_field: ::validated_data::Field<&'a str>,
                        pub flags: ::validated_data::Field<item::Flags<'a, Mode>>,
                        pub icmp_type: ::validated_data::Field<item::IcmpType<'a, Mode>>,
                        pub enforce_gtsm: ::validated_data::Field<bool>,
                    }

                    pub mod item {

                        #[::validated_data::data_view(list)]
                        pub struct Flags<'a, Mode> (::validated_data::Field<&'a str>);

                        #[::validated_data::data_view(list)]
                        pub struct IcmpType<'a, Mode> (::validated_data::Field<&'a str>);
                    }
                }

                #[::validated_data::data_view]
                pub struct PacketType<'a, Mode> {
                    pub vxlan: ::validated_data::Field<&'a str>,
                    pub multicast: ::validated_data::Field<bool>,
                }

                #[::validated_data::data_view]
                pub struct Actions<'a, Mode> {
                    pub dscp: ::validated_data::Field<i64>,
                    pub traffic_class: ::validated_data::Field<i64>,
                    pub count: ::validated_data::Field<&'a str>,
                    pub drop: ::validated_data::Field<bool>,
                    pub log: ::validated_data::Field<bool>,
                    pub redirect: ::validated_data::Field<actions::Redirect<'a, Mode>>,
                }

                pub mod actions {

                    #[::validated_data::data_view]
                    pub struct Redirect<'a, Mode> {
                        pub aggregation_groups: ::validated_data::Field<redirect::AggregationGroups<'a, Mode>>,
                        pub interface: ::validated_data::Field<&'a str>,
                        pub next_hop: ::validated_data::Field<redirect::NextHop<'a, Mode>>,
                    }

                    pub mod redirect {

                        #[::validated_data::data_view(list)]
                        pub struct AggregationGroups<'a, Mode> (::validated_data::Field<&'a str>);

                        #[::validated_data::data_view]
                        pub struct NextHop<'a, Mode> {
                            pub ipv4_addresses: ::validated_data::Field<next_hop::Ipv4Addresses<'a, Mode>>,
                            pub ipv6_addresses: ::validated_data::Field<next_hop::Ipv6Addresses<'a, Mode>>,
                            pub vrf: ::validated_data::Field<&'a str>,
                            pub groups: ::validated_data::Field<next_hop::Groups<'a, Mode>>,
                            pub recursive_ipv4_addresses: ::validated_data::Field<next_hop::RecursiveIpv4Addresses<'a, Mode>>,
                            pub recursive_ipv6_addresses: ::validated_data::Field<next_hop::RecursiveIpv6Addresses<'a, Mode>>,
                            pub ttl: ::validated_data::Field<i64>,
                        }

                        pub mod next_hop {

                            #[::validated_data::data_view(list)]
                            pub struct Ipv4Addresses<'a, Mode> (::validated_data::Field<&'a str>);

                            #[::validated_data::data_view(list)]
                            pub struct Ipv6Addresses<'a, Mode> (::validated_data::Field<&'a str>);

                            #[::validated_data::data_view(list)]
                            pub struct Groups<'a, Mode> (::validated_data::Field<&'a str>);

                            #[::validated_data::data_view(list)]
                            pub struct RecursiveIpv4Addresses<'a, Mode> (::validated_data::Field<&'a str>);

                            #[::validated_data::data_view(list)]
                            pub struct RecursiveIpv6Addresses<'a, Mode> (::validated_data::Field<&'a str>);
                        }
                    }
                }
            }
        }

        #[::validated_data::data_view]
        pub struct DefaultActions<'a, Mode> {
            pub ipv4: ::validated_data::Field<default_actions::Ipv4<'a, Mode>>,
            pub ipv6: ::validated_data::Field<default_actions::Ipv6<'a, Mode>>,
        }

        pub mod default_actions {

            #[::validated_data::data_view]
            pub struct Ipv4<'a, Mode> {
                pub dscp: ::validated_data::Field<i64>,
                pub traffic_class: ::validated_data::Field<i64>,
                pub count: ::validated_data::Field<&'a str>,
                pub drop: ::validated_data::Field<bool>,
                pub log: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view]
            pub struct Ipv6<'a, Mode> {
                pub dscp: ::validated_data::Field<i64>,
                pub traffic_class: ::validated_data::Field<i64>,
                pub count: ::validated_data::Field<&'a str>,
                pub drop: ::validated_data::Field<bool>,
                pub log: ::validated_data::Field<bool>,
            }
        }
    }
}
