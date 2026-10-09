// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub profile: ::validated_data::Field<&'a str>,
    pub name: ::validated_data::Field<&'a str>,
    pub description: ::validated_data::Field<&'a str>,
    pub ip_address: ::validated_data::Field<&'a str>,
    pub ipv6_addresses: ::validated_data::Field<item::Ipv6Addresses<'a, Mode>>,
    pub dhcp_ip: ::validated_data::Field<&'a str>,
    pub public_ip: ::validated_data::Field<&'a str>,
    pub encapsulation_dot1q_vlan: ::validated_data::Field<i64>,
    pub dhcp_accept_default_route: ::validated_data::Field<bool>,
    pub enabled: ::validated_data::Field<bool>,
    pub speed: ::validated_data::Field<&'a str>,
    pub receive_bandwidth: ::validated_data::Field<i64>,
    pub transmit_bandwidth: ::validated_data::Field<i64>,
    pub peer: ::validated_data::Field<&'a str>,
    pub peer_interface: ::validated_data::Field<&'a str>,
    pub peer_ip: ::validated_data::Field<&'a str>,
    pub peer_ipv6: ::validated_data::Field<&'a str>,
    pub bgp: ::validated_data::Field<item::Bgp<'a, Mode>>,
    pub ipv4_acl_in: ::validated_data::Field<&'a str>,
    pub ipv4_acl_out: ::validated_data::Field<&'a str>,
    pub ipv6_acl_in: ::validated_data::Field<&'a str>,
    pub ipv6_acl_out: ::validated_data::Field<&'a str>,
    pub static_routes: ::validated_data::Field<item::StaticRoutes<'a, Mode>>,
    pub qos_profile: ::validated_data::Field<&'a str>,
    pub wan_carrier: ::validated_data::Field<&'a str>,
    pub wan_circuit_id: ::validated_data::Field<&'a str>,
    pub connected_to_pathfinder: ::validated_data::Field<bool>,
    pub cv_pathfinder_internet_exit: ::validated_data::Field<item::CvPathfinderInternetExit<'a, Mode>>,
    pub rx_queue: ::validated_data::Field<item::RxQueue<'a, Mode>>,
    pub raw_eos_cli: ::validated_data::Field<&'a str>,
    pub flow_tracking: ::validated_data::Field<item::FlowTracking<'a, Mode>>,
    #[data_view(relaxed)]
    pub structured_config: ::validated_data::Field<super::super::eos_cli_config_gen::ethernet_interfaces::Item<'a, ::validated_data::RelaxedValidated>>,
}

pub mod item {

    #[::validated_data::data_view(list)]
    pub struct Ipv6Addresses<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view]
    pub struct Bgp<'a, Mode> {
        pub peer_as: ::validated_data::RequiredValue<&'a str, Mode>,
        pub ipv4_prefix_list_in: ::validated_data::Field<&'a str>,
        pub ipv4_prefix_list_out: ::validated_data::Field<&'a str>,
        pub ipv6_prefix_list_in: ::validated_data::Field<&'a str>,
        pub ipv6_prefix_list_out: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view(list)]
    pub struct StaticRoutes<'a, Mode> (::validated_data::Field<static_routes::Item<'a, Mode>>);

    pub mod static_routes {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub prefix: ::validated_data::RequiredValue<&'a str, Mode>,
        }
    }

    #[::validated_data::data_view]
    pub struct CvPathfinderInternetExit<'a, Mode> {
        pub policies: ::validated_data::Field<cv_pathfinder_internet_exit::Policies<'a, Mode>>,
    }

    pub mod cv_pathfinder_internet_exit {

        #[::validated_data::data_view(indexed_list, primary_key(name))]
        pub struct Policies<'a, Mode> (::validated_data::Field<policies::Item<'a, Mode>>);

        pub mod policies {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::validated_data::Field<&'a str>,
                pub tunnel_interface_numbers: ::validated_data::Field<&'a str>,
            }
        }
    }

    #[::validated_data::data_view]
    pub struct RxQueue<'a, Mode> {
        pub count: ::validated_data::Field<i64>,
        pub workers: ::validated_data::Field<rx_queue::Workers<'a, Mode>>,
        pub mode: ::validated_data::Field<&'a str>,
    }

    pub mod rx_queue {

        #[::validated_data::data_view(list)]
        pub struct Workers<'a, Mode> (::validated_data::Field<&'a str>);
    }

    #[::validated_data::data_view]
    pub struct FlowTracking<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub name: ::validated_data::Field<&'a str>,
    }
}
