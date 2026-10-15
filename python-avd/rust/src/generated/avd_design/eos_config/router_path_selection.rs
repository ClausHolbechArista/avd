// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct MtuDiscoveryHosts<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub fragmentation_needed_rate_limit: ::validated_data::Field<i64>,
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct PathGroups<'a, Mode> (::validated_data::Field<path_groups::Item<'a, Mode>>);

pub mod path_groups {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub id: ::validated_data::Field<i64>,
        pub ipsec_profile: ::validated_data::Field<&'a str>,
        pub flow_assignment: ::validated_data::Field<&'a str>,
        pub local_interfaces: ::validated_data::Field<item::LocalInterfaces<'a, Mode>>,
        pub local_ips: ::validated_data::Field<item::LocalIps<'a, Mode>>,
        pub dynamic_peers: ::validated_data::Field<item::DynamicPeers<'a, Mode>>,
        pub static_peers: ::validated_data::Field<item::StaticPeers<'a, Mode>>,
        pub keepalive: ::validated_data::Field<item::Keepalive<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view(indexed_list, primary_key(name))]
        pub struct LocalInterfaces<'a, Mode> (::validated_data::Field<local_interfaces::Item<'a, Mode>>);

        pub mod local_interfaces {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::validated_data::Field<&'a str>,
                pub public_address: ::validated_data::Field<&'a str>,
                pub stun: ::validated_data::Field<item::Stun<'a, Mode>>,
            }

            pub mod item {

                #[::validated_data::data_view]
                pub struct Stun<'a, Mode> {
                    pub server_profiles: ::validated_data::RequiredValue<stun::ServerProfiles<'a, Mode>, Mode>,
                }

                pub mod stun {

                    #[::validated_data::data_view(list)]
                    pub struct ServerProfiles<'a, Mode> (::validated_data::Field<&'a str>);
                }
            }
        }

        #[::validated_data::data_view(indexed_list, primary_key(ip_address))]
        pub struct LocalIps<'a, Mode> (::validated_data::Field<local_ips::Item<'a, Mode>>);

        pub mod local_ips {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub ip_address: ::validated_data::Field<&'a str>,
                pub public_address: ::validated_data::Field<&'a str>,
                pub stun: ::validated_data::Field<item::Stun<'a, Mode>>,
            }

            pub mod item {

                #[::validated_data::data_view]
                pub struct Stun<'a, Mode> {
                    pub server_profiles: ::validated_data::RequiredValue<stun::ServerProfiles<'a, Mode>, Mode>,
                }

                pub mod stun {

                    #[::validated_data::data_view(list)]
                    pub struct ServerProfiles<'a, Mode> (::validated_data::Field<&'a str>);
                }
            }
        }

        #[::validated_data::data_view]
        pub struct DynamicPeers<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub ip_local: ::validated_data::Field<bool>,
            pub ipsec: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view(indexed_list, primary_key(router_ip))]
        pub struct StaticPeers<'a, Mode> (::validated_data::Field<static_peers::Item<'a, Mode>>);

        pub mod static_peers {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub router_ip: ::validated_data::Field<&'a str>,
                pub name: ::validated_data::Field<&'a str>,
                pub ipv4_addresses: ::validated_data::Field<item::Ipv4Addresses<'a, Mode>>,
            }

            pub mod item {

                #[::validated_data::data_view(list)]
                pub struct Ipv4Addresses<'a, Mode> (::validated_data::Field<&'a str>);
            }
        }

        #[::validated_data::data_view]
        pub struct Keepalive<'a, Mode> {
            pub auto: ::validated_data::Field<bool>,
            pub interval: ::validated_data::Field<i64>,
            pub failure_threshold: ::validated_data::Field<i64>,
        }
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct LoadBalancePolicies<'a, Mode> (::validated_data::Field<load_balance_policies::Item<'a, Mode>>);

pub mod load_balance_policies {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub lowest_hop_count: ::validated_data::Field<bool>,
        pub jitter: ::validated_data::Field<i64>,
        pub latency: ::validated_data::Field<i64>,
        pub loss_rate: ::validated_data::Field<&'a str>,
        pub path_groups: ::validated_data::Field<item::PathGroups<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view(indexed_list, primary_key(name))]
        pub struct PathGroups<'a, Mode> (::validated_data::Field<path_groups::Item<'a, Mode>>);

        pub mod path_groups {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::validated_data::Field<&'a str>,
                pub priority: ::validated_data::Field<i64>,
            }
        }
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Policies<'a, Mode> (::validated_data::Field<policies::Item<'a, Mode>>);

pub mod policies {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub default_match: ::validated_data::Field<item::DefaultMatch<'a, Mode>>,
        pub rules: ::validated_data::Field<item::Rules<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view]
        pub struct DefaultMatch<'a, Mode> {
            pub load_balance: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view(indexed_list, primary_key(id))]
        pub struct Rules<'a, Mode> (::validated_data::Field<rules::Item<'a, Mode>>);

        pub mod rules {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub id: ::validated_data::Field<i64>,
                pub application_profile: ::validated_data::RequiredValue<&'a str, Mode>,
                pub load_balance: ::validated_data::Field<&'a str>,
            }
        }
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Vrfs<'a, Mode> (::validated_data::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub path_selection_policy: ::validated_data::Field<&'a str>,
    }
}

#[::validated_data::data_view]
pub struct TcpMssCeiling<'a, Mode> {
    pub ipv4: ::validated_data::Field<&'a str>,
    pub direction: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Interfaces<'a, Mode> (::validated_data::Field<interfaces::Item<'a, Mode>>);

pub mod interfaces {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub metric_bandwidth: ::validated_data::Field<item::MetricBandwidth<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view]
        pub struct MetricBandwidth<'a, Mode> {
            pub receive: ::validated_data::Field<i64>,
            pub transmit: ::validated_data::Field<i64>,
        }
    }
}
