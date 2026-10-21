// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct TrafficPolicies<'a, Mode> {
    pub cpu_traffic_policy: ::data_views::Field<CpuTrafficPolicy<'a, Mode>>,
    pub vrfs: ::data_views::Field<Vrfs<'a, Mode>>,
    pub options: ::data_views::Field<Options<'a, Mode>>,
    pub field_sets: ::data_views::Field<FieldSets<'a, Mode>>,
    pub policies: ::data_views::Field<Policies<'a, Mode>>,
}

#[::data_views::data_view]
pub struct CpuTrafficPolicy<'a, Mode> {
    pub vrf_all: ::data_views::Field<cpu_traffic_policy::VrfAll<'a, Mode>>,
    pub enforcement_ip_ttl_expired: ::data_views::Field<bool>,
    pub fragment_implicit_permit_disabled: ::data_views::Field<bool>,
}

pub mod cpu_traffic_policy {

    #[::data_views::data_view]
    pub struct VrfAll<'a, Mode> {
        pub name: ::data_views::RequiredValue<&'a str, Mode>,
        pub enforcement_management: ::data_views::Field<bool>,
    }
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Vrfs<'a, Mode> (::data_views::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub cpu: ::data_views::Field<item::Cpu<'a, Mode>>,
        pub traffic_policy_input_physical: ::data_views::Field<&'a str>,
    }

    pub mod item {

        #[::data_views::data_view]
        pub struct Cpu<'a, Mode> {
            pub traffic_policy: ::data_views::RequiredValue<cpu::TrafficPolicy<'a, Mode>, Mode>,
        }

        pub mod cpu {

            #[::data_views::data_view]
            pub struct TrafficPolicy<'a, Mode> {
                pub name: ::data_views::RequiredValue<&'a str, Mode>,
                pub enforcement_management: ::data_views::Field<bool>,
            }
        }
    }
}

#[::data_views::data_view]
pub struct Options<'a, Mode> {
    pub counter_per_interface: ::data_views::Field<bool>,
    pub counter_interface_poll_interval: ::data_views::Field<i64>,
}

#[::data_views::data_view]
pub struct FieldSets<'a, Mode> {
    pub ipv4: ::data_views::Field<field_sets::Ipv4<'a, Mode>>,
    pub ipv6: ::data_views::Field<field_sets::Ipv6<'a, Mode>>,
    pub ports: ::data_views::Field<field_sets::Ports<'a, Mode>>,
}

pub mod field_sets {

    #[::data_views::data_view(indexed_list, primary_key(name))]
    pub struct Ipv4<'a, Mode> (::data_views::Field<ipv4::Item<'a, Mode>>);

    pub mod ipv4 {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::data_views::Field<&'a str>,
            pub prefixes: ::data_views::Field<item::Prefixes<'a, Mode>>,
            #[data_view(rename = "except")]
            pub field_except: ::data_views::Field<item::Except<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view(list)]
            pub struct Prefixes<'a, Mode> (::data_views::Field<&'a str>);

            #[::data_views::data_view(list)]
            pub struct Except<'a, Mode> (::data_views::Field<&'a str>);
        }
    }

    #[::data_views::data_view(indexed_list, primary_key(name))]
    pub struct Ipv6<'a, Mode> (::data_views::Field<ipv6::Item<'a, Mode>>);

    pub mod ipv6 {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::data_views::Field<&'a str>,
            pub prefixes: ::data_views::Field<item::Prefixes<'a, Mode>>,
            #[data_view(rename = "except")]
            pub field_except: ::data_views::Field<item::Except<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view(list)]
            pub struct Prefixes<'a, Mode> (::data_views::Field<&'a str>);

            #[::data_views::data_view(list)]
            pub struct Except<'a, Mode> (::data_views::Field<&'a str>);
        }
    }

    #[::data_views::data_view(indexed_list, primary_key(name))]
    pub struct Ports<'a, Mode> (::data_views::Field<ports::Item<'a, Mode>>);

    pub mod ports {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::data_views::Field<&'a str>,
            pub port_range: ::data_views::Field<&'a str>,
        }
    }
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Policies<'a, Mode> (::data_views::Field<policies::Item<'a, Mode>>);

pub mod policies {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub counters: ::data_views::Field<item::Counters<'a, Mode>>,
        pub matches: ::data_views::Field<item::Matches<'a, Mode>>,
        pub default_actions: ::data_views::Field<item::DefaultActions<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view(list)]
        pub struct Counters<'a, Mode> (::data_views::Field<&'a str>);

        #[::data_views::data_view(indexed_list, primary_key(name))]
        pub struct Matches<'a, Mode> (::data_views::Field<matches::Item<'a, Mode>>);

        pub mod matches {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::data_views::Field<&'a str>,
                #[data_view(rename = "type")]
                pub field_type: ::data_views::RequiredValue<&'a str, Mode>,
                pub source: ::data_views::Field<item::Source<'a, Mode>>,
                pub destination: ::data_views::Field<item::Destination<'a, Mode>>,
                pub ttl: ::data_views::Field<&'a str>,
                pub fragment: ::data_views::Field<item::Fragment<'a, Mode>>,
                pub protocols: ::data_views::Field<item::Protocols<'a, Mode>>,
                pub packet_type: ::data_views::Field<item::PacketType<'a, Mode>>,
                pub actions: ::data_views::Field<item::Actions<'a, Mode>>,
            }

            pub mod item {

                #[::data_views::data_view]
                pub struct Source<'a, Mode> {
                    pub prefixes: ::data_views::Field<source::Prefixes<'a, Mode>>,
                    pub prefix_lists: ::data_views::Field<source::PrefixLists<'a, Mode>>,
                }

                pub mod source {

                    #[::data_views::data_view(list)]
                    pub struct Prefixes<'a, Mode> (::data_views::Field<&'a str>);

                    #[::data_views::data_view(list)]
                    pub struct PrefixLists<'a, Mode> (::data_views::Field<&'a str>);
                }

                #[::data_views::data_view]
                pub struct Destination<'a, Mode> {
                    pub prefixes: ::data_views::Field<destination::Prefixes<'a, Mode>>,
                    pub prefix_lists: ::data_views::Field<destination::PrefixLists<'a, Mode>>,
                }

                pub mod destination {

                    #[::data_views::data_view(list)]
                    pub struct Prefixes<'a, Mode> (::data_views::Field<&'a str>);

                    #[::data_views::data_view(list)]
                    pub struct PrefixLists<'a, Mode> (::data_views::Field<&'a str>);
                }

                #[::data_views::data_view]
                pub struct Fragment<'a, Mode> {
                    pub offset: ::data_views::Field<&'a str>,
                }

                #[::data_views::data_view(indexed_list, primary_key(protocol))]
                pub struct Protocols<'a, Mode> (::data_views::Field<protocols::Item<'a, Mode>>);

                pub mod protocols {

                    #[::data_views::data_view]
                    pub struct Item<'a, Mode> {
                        pub protocol: ::data_views::Field<&'a str>,
                        pub src_port: ::data_views::Field<&'a str>,
                        pub dst_port: ::data_views::Field<&'a str>,
                        pub src_field: ::data_views::Field<&'a str>,
                        pub dst_field: ::data_views::Field<&'a str>,
                        pub flags: ::data_views::Field<item::Flags<'a, Mode>>,
                        pub icmp_type: ::data_views::Field<item::IcmpType<'a, Mode>>,
                        pub enforce_gtsm: ::data_views::Field<bool>,
                    }

                    pub mod item {

                        #[::data_views::data_view(list)]
                        pub struct Flags<'a, Mode> (::data_views::Field<&'a str>);

                        #[::data_views::data_view(list)]
                        pub struct IcmpType<'a, Mode> (::data_views::Field<&'a str>);
                    }
                }

                #[::data_views::data_view]
                pub struct PacketType<'a, Mode> {
                    pub vxlan: ::data_views::Field<&'a str>,
                    pub multicast: ::data_views::Field<bool>,
                }

                #[::data_views::data_view]
                pub struct Actions<'a, Mode> {
                    pub dscp: ::data_views::Field<i64>,
                    pub traffic_class: ::data_views::Field<i64>,
                    pub count: ::data_views::Field<&'a str>,
                    pub drop: ::data_views::Field<bool>,
                    pub log: ::data_views::Field<bool>,
                    pub redirect: ::data_views::Field<actions::Redirect<'a, Mode>>,
                }

                pub mod actions {

                    #[::data_views::data_view]
                    pub struct Redirect<'a, Mode> {
                        pub aggregation_groups: ::data_views::Field<redirect::AggregationGroups<'a, Mode>>,
                        pub interface: ::data_views::Field<&'a str>,
                        pub next_hop: ::data_views::Field<redirect::NextHop<'a, Mode>>,
                    }

                    pub mod redirect {

                        #[::data_views::data_view(list)]
                        pub struct AggregationGroups<'a, Mode> (::data_views::Field<&'a str>);

                        #[::data_views::data_view]
                        pub struct NextHop<'a, Mode> {
                            pub ipv4_addresses: ::data_views::Field<next_hop::Ipv4Addresses<'a, Mode>>,
                            pub ipv6_addresses: ::data_views::Field<next_hop::Ipv6Addresses<'a, Mode>>,
                            pub vrf: ::data_views::Field<&'a str>,
                            pub groups: ::data_views::Field<next_hop::Groups<'a, Mode>>,
                            pub recursive_ipv4_addresses: ::data_views::Field<next_hop::RecursiveIpv4Addresses<'a, Mode>>,
                            pub recursive_ipv6_addresses: ::data_views::Field<next_hop::RecursiveIpv6Addresses<'a, Mode>>,
                            pub ttl: ::data_views::Field<i64>,
                        }

                        pub mod next_hop {

                            #[::data_views::data_view(list)]
                            pub struct Ipv4Addresses<'a, Mode> (::data_views::Field<&'a str>);

                            #[::data_views::data_view(list)]
                            pub struct Ipv6Addresses<'a, Mode> (::data_views::Field<&'a str>);

                            #[::data_views::data_view(list)]
                            pub struct Groups<'a, Mode> (::data_views::Field<&'a str>);

                            #[::data_views::data_view(list)]
                            pub struct RecursiveIpv4Addresses<'a, Mode> (::data_views::Field<&'a str>);

                            #[::data_views::data_view(list)]
                            pub struct RecursiveIpv6Addresses<'a, Mode> (::data_views::Field<&'a str>);
                        }
                    }
                }
            }
        }

        #[::data_views::data_view]
        pub struct DefaultActions<'a, Mode> {
            pub ipv4: ::data_views::Field<default_actions::Ipv4<'a, Mode>>,
            pub ipv6: ::data_views::Field<default_actions::Ipv6<'a, Mode>>,
        }

        pub mod default_actions {

            #[::data_views::data_view]
            pub struct Ipv4<'a, Mode> {
                pub dscp: ::data_views::Field<i64>,
                pub traffic_class: ::data_views::Field<i64>,
                pub count: ::data_views::Field<&'a str>,
                pub drop: ::data_views::Field<bool>,
                pub log: ::data_views::Field<bool>,
            }

            #[::data_views::data_view]
            pub struct Ipv6<'a, Mode> {
                pub dscp: ::data_views::Field<i64>,
                pub traffic_class: ::data_views::Field<i64>,
                pub count: ::data_views::Field<&'a str>,
                pub drop: ::data_views::Field<bool>,
                pub log: ::data_views::Field<bool>,
            }
        }
    }
}
