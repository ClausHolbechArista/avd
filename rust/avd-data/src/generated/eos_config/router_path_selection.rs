// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct RouterPathSelection<'a, Mode> {
    pub peer_dynamic_source: ::data_views::Field<&'a str>,
    pub mtu_discovery_interval: ::data_views::Field<i64>,
    pub mtu_discovery_hosts: ::data_views::Field<MtuDiscoveryHosts<'a, Mode>>,
    pub path_groups: ::data_views::Field<PathGroups<'a, Mode>>,
    pub load_balance_policies: ::data_views::Field<LoadBalancePolicies<'a, Mode>>,
    pub policies: ::data_views::Field<Policies<'a, Mode>>,
    pub vrfs: ::data_views::Field<Vrfs<'a, Mode>>,
    pub tcp_mss_ceiling: ::data_views::Field<TcpMssCeiling<'a, Mode>>,
    pub interfaces: ::data_views::Field<Interfaces<'a, Mode>>,
}

#[::data_views::data_view]
pub struct MtuDiscoveryHosts<'a, Mode> {
    pub enabled: ::data_views::Field<bool>,
    pub fragmentation_needed_rate_limit: ::data_views::Field<i64>,
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct PathGroups<'a, Mode> (::data_views::Field<path_groups::Item<'a, Mode>>);

pub mod path_groups {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub id: ::data_views::Field<i64>,
        pub ipsec_profile: ::data_views::Field<&'a str>,
        pub flow_assignment: ::data_views::Field<&'a str>,
        pub local_interfaces: ::data_views::Field<item::LocalInterfaces<'a, Mode>>,
        pub local_ips: ::data_views::Field<item::LocalIps<'a, Mode>>,
        pub dynamic_peers: ::data_views::Field<item::DynamicPeers<'a, Mode>>,
        pub static_peers: ::data_views::Field<item::StaticPeers<'a, Mode>>,
        pub keepalive: ::data_views::Field<item::Keepalive<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view(indexed_list, primary_key(name))]
        pub struct LocalInterfaces<'a, Mode> (::data_views::Field<local_interfaces::Item<'a, Mode>>);

        pub mod local_interfaces {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::data_views::Field<&'a str>,
                pub public_address: ::data_views::Field<&'a str>,
                pub stun: ::data_views::Field<item::Stun<'a, Mode>>,
            }

            pub mod item {

                #[::data_views::data_view]
                pub struct Stun<'a, Mode> {
                    pub server_profiles: ::data_views::RequiredValue<stun::ServerProfiles<'a, Mode>, Mode>,
                }

                pub mod stun {

                    #[::data_views::data_view(list)]
                    pub struct ServerProfiles<'a, Mode> (::data_views::Field<&'a str>);
                }
            }
        }

        #[::data_views::data_view(indexed_list, primary_key(ip_address))]
        pub struct LocalIps<'a, Mode> (::data_views::Field<local_ips::Item<'a, Mode>>);

        pub mod local_ips {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub ip_address: ::data_views::Field<&'a str>,
                pub public_address: ::data_views::Field<&'a str>,
                pub stun: ::data_views::Field<item::Stun<'a, Mode>>,
            }

            pub mod item {

                #[::data_views::data_view]
                pub struct Stun<'a, Mode> {
                    pub server_profiles: ::data_views::RequiredValue<stun::ServerProfiles<'a, Mode>, Mode>,
                }

                pub mod stun {

                    #[::data_views::data_view(list)]
                    pub struct ServerProfiles<'a, Mode> (::data_views::Field<&'a str>);
                }
            }
        }

        #[::data_views::data_view]
        pub struct DynamicPeers<'a, Mode> {
            pub enabled: ::data_views::Field<bool>,
            pub ip_local: ::data_views::Field<bool>,
            pub ipsec: ::data_views::Field<bool>,
        }

        #[::data_views::data_view(indexed_list, primary_key(router_ip))]
        pub struct StaticPeers<'a, Mode> (::data_views::Field<static_peers::Item<'a, Mode>>);

        pub mod static_peers {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub router_ip: ::data_views::Field<&'a str>,
                pub name: ::data_views::Field<&'a str>,
                pub ipv4_addresses: ::data_views::Field<item::Ipv4Addresses<'a, Mode>>,
            }

            pub mod item {

                #[::data_views::data_view(list)]
                pub struct Ipv4Addresses<'a, Mode> (::data_views::Field<&'a str>);
            }
        }

        #[::data_views::data_view]
        pub struct Keepalive<'a, Mode> {
            pub auto: ::data_views::Field<bool>,
            pub interval: ::data_views::Field<i64>,
            pub failure_threshold: ::data_views::Field<i64>,
        }
    }
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct LoadBalancePolicies<'a, Mode> (::data_views::Field<load_balance_policies::Item<'a, Mode>>);

pub mod load_balance_policies {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub lowest_hop_count: ::data_views::Field<bool>,
        pub jitter: ::data_views::Field<i64>,
        pub latency: ::data_views::Field<i64>,
        pub loss_rate: ::data_views::Field<&'a str>,
        pub path_groups: ::data_views::Field<item::PathGroups<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view(indexed_list, primary_key(name))]
        pub struct PathGroups<'a, Mode> (::data_views::Field<path_groups::Item<'a, Mode>>);

        pub mod path_groups {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::data_views::Field<&'a str>,
                pub priority: ::data_views::Field<i64>,
            }
        }
    }
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Policies<'a, Mode> (::data_views::Field<policies::Item<'a, Mode>>);

pub mod policies {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub default_match: ::data_views::Field<item::DefaultMatch<'a, Mode>>,
        pub rules: ::data_views::Field<item::Rules<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view]
        pub struct DefaultMatch<'a, Mode> {
            pub load_balance: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view(indexed_list, primary_key(id))]
        pub struct Rules<'a, Mode> (::data_views::Field<rules::Item<'a, Mode>>);

        pub mod rules {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub id: ::data_views::Field<i64>,
                pub application_profile: ::data_views::RequiredValue<&'a str, Mode>,
                pub load_balance: ::data_views::Field<&'a str>,
            }
        }
    }
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Vrfs<'a, Mode> (::data_views::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub path_selection_policy: ::data_views::Field<&'a str>,
    }
}

#[::data_views::data_view]
pub struct TcpMssCeiling<'a, Mode> {
    pub ipv4: ::data_views::Field<&'a str>,
    pub direction: ::data_views::Field<&'a str>,
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Interfaces<'a, Mode> (::data_views::Field<interfaces::Item<'a, Mode>>);

pub mod interfaces {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub metric_bandwidth: ::data_views::Field<item::MetricBandwidth<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view]
        pub struct MetricBandwidth<'a, Mode> {
            pub receive: ::data_views::Field<i64>,
            pub transmit: ::data_views::Field<i64>,
        }
    }
}
