// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(profile), python(name = "AVDDesign.L3InterfaceProfiles", schema = List(669), item_name = "AVDDesign.L3InterfaceProfilesItem"))]
pub struct L3InterfaceProfiles<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view(python(name = "AVDDesign.L3InterfaceProfilesItemBase", schema = Dict(1614)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(1441))]
    pub profile: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1327))]
    pub name: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1328))]
    pub description: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1329))]
    pub ip_address: ::data_views::Field<&'a str>,
    #[data_view(schema = List(631))]
    pub ipv6_addresses: ::data_views::Field<item::Ipv6Addresses<'a, Mode>>,
    #[data_view(schema = Str(1331))]
    pub dhcp_ip: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1332))]
    pub public_ip: ::data_views::Field<&'a str>,
    #[data_view(schema = Int(565))]
    pub encapsulation_dot1q_vlan: ::data_views::Field<i64>,
    #[data_view(schema = Bool(479))]
    pub dhcp_accept_default_route: ::data_views::Field<bool>,
    #[data_view(schema = Bool(480))]
    pub enabled: ::data_views::Field<bool>,
    #[data_view(schema = Str(178))]
    pub speed: ::data_views::Field<&'a str>,
    #[data_view(schema = Int(566))]
    pub receive_bandwidth: ::data_views::Field<i64>,
    #[data_view(schema = Int(567))]
    pub transmit_bandwidth: ::data_views::Field<i64>,
    #[data_view(schema = Str(1333))]
    pub peer: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1334))]
    pub peer_interface: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1335))]
    pub peer_ip: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1336))]
    pub peer_ipv6: ::data_views::Field<&'a str>,
    #[data_view(schema = Dict(1499))]
    pub bgp: ::data_views::Field<item::Bgp<'a, Mode>>,
    #[data_view(schema = Str(1342))]
    pub ipv4_acl_in: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1343))]
    pub ipv4_acl_out: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1344))]
    pub ipv6_acl_in: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1345))]
    pub ipv6_acl_out: ::data_views::Field<&'a str>,
    #[data_view(schema = List(632))]
    pub static_routes: ::data_views::Field<item::StaticRoutes<'a, Mode>>,
    #[data_view(schema = Str(1187))]
    pub qos_profile: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1346))]
    pub wan_carrier: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1347))]
    pub wan_circuit_id: ::data_views::Field<&'a str>,
    #[data_view(schema = Bool(481))]
    pub connected_to_pathfinder: ::data_views::Field<bool>,
    #[data_view(schema = Dict(1502))]
    pub cv_pathfinder_internet_exit: ::data_views::Field<item::CvPathfinderInternetExit<'a, Mode>>,
    #[data_view(schema = Dict(1503))]
    pub rx_queue: ::data_views::Field<item::RxQueue<'a, Mode>>,
    #[data_view(schema = Str(1351))]
    pub raw_eos_cli: ::data_views::Field<&'a str>,
    #[data_view(schema = Dict(1504))]
    pub flow_tracking: ::data_views::Field<item::FlowTracking<'a, Mode>>,
    #[data_view(schema = Dict(1505), relaxed)]
    pub structured_config: ::data_views::Field<super::super::eos_config::ethernet_interfaces::Item<'a, ::data_views::RelaxedValidated>>,
}

pub mod item {

    #[::data_views::data_view(list, python(name = "AVDDesign.L3InterfaceProfilesItemBase.Ipv6Addresses", schema = List(631)))]
    pub struct Ipv6Addresses<'a, Mode> (::data_views::Field<&'a str>);

    #[::data_views::data_view(python(name = "AVDDesign.L3InterfaceProfilesItemBase.Bgp", schema = Dict(1499)))]
    pub struct Bgp<'a, Mode> {
        #[data_view(schema = Str(1337))]
        pub peer_as: ::data_views::RequiredValue<&'a str, Mode>,
        #[data_view(schema = Str(1338))]
        pub ipv4_prefix_list_in: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1339))]
        pub ipv4_prefix_list_out: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1340))]
        pub ipv6_prefix_list_in: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1341))]
        pub ipv6_prefix_list_out: ::data_views::Field<&'a str>,
    }

    #[::data_views::data_view(list, python(name = "AVDDesign.L3InterfaceProfilesItemBase.StaticRoutes", schema = List(632)))]
    pub struct StaticRoutes<'a, Mode> (::data_views::Field<static_routes::Item<'a, Mode>>);

    pub mod static_routes {

        #[::data_views::data_view(python(name = "AVDDesign.L3InterfaceProfilesItemBase.StaticRoutesItem", schema = Dict(1500)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(984))]
            pub prefix: ::data_views::RequiredValue<&'a str, Mode>,
        }
    }

    #[::data_views::data_view(python(name = "AVDDesign.L3InterfaceProfilesItemBase.CvPathfinderInternetExit", schema = Dict(1502)))]
    pub struct CvPathfinderInternetExit<'a, Mode> {
        #[data_view(schema = List(633))]
        pub policies: ::data_views::Field<cv_pathfinder_internet_exit::Policies<'a, Mode>>,
    }

    pub mod cv_pathfinder_internet_exit {

        #[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.L3InterfaceProfilesItemBase.CvPathfinderInternetExit.Policies", schema = List(633), item_name = "AVDDesign.L3InterfaceProfilesItemBase.CvPathfinderInternetExit.PoliciesItem"))]
        pub struct Policies<'a, Mode> (::data_views::Field<policies::Item<'a, Mode>>);

        pub mod policies {

            #[::data_views::data_view(python(name = "AVDDesign.L3InterfaceProfilesItemBase.CvPathfinderInternetExit.PoliciesItemBase", schema = Dict(1501)))]
            pub struct Item<'a, Mode> {
                #[data_view(schema = Str(1199))]
                pub name: ::data_views::Field<&'a str>,
                #[data_view(schema = Str(1348))]
                pub tunnel_interface_numbers: ::data_views::Field<&'a str>,
            }
        }
    }

    #[::data_views::data_view(python(name = "AVDDesign.L3InterfaceProfilesItemBase.RxQueue", schema = Dict(1503)))]
    pub struct RxQueue<'a, Mode> {
        #[data_view(schema = Int(568))]
        pub count: ::data_views::Field<i64>,
        #[data_view(schema = List(634))]
        pub workers: ::data_views::Field<rx_queue::Workers<'a, Mode>>,
        #[data_view(schema = Str(1350))]
        pub mode: ::data_views::Field<&'a str>,
    }

    pub mod rx_queue {

        #[::data_views::data_view(list, python(name = "AVDDesign.L3InterfaceProfilesItemBase.RxQueue.Workers", schema = List(634)))]
        pub struct Workers<'a, Mode> (::data_views::Field<&'a str>);
    }

    #[::data_views::data_view(python(name = "AVDDesign.L3InterfaceProfilesItemBase.FlowTracking", schema = Dict(1504)))]
    pub struct FlowTracking<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub enabled: ::data_views::Field<bool>,
        #[data_view(schema = Str(1155))]
        pub name: ::data_views::Field<&'a str>,
    }
}
