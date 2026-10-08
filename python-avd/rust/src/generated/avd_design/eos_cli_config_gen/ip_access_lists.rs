// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar name("name", 0) -> &'a str;
        scalar counters_per_entry("counters_per_entry", 1) -> bool;
        model entries("entries", 2) -> item::Entries<'a>;
        scalar permit_response_traffic("permit_response_traffic", 3) -> &'a str;
    }
}

pub mod item {

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Entries {
            model item (0) -> entries::Item<'a>;
        }
    }

    pub mod entries {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar sequence("sequence", 0) -> i64;
                scalar remark("remark", 1) -> &'a str;
                scalar action("action", 2) -> &'a str;
                scalar protocol("protocol", 3) -> &'a str;
                scalar source("source", 4) -> &'a str;
                scalar destination("destination", 5) -> &'a str;
                scalar fragments("fragments", 6) -> bool;
                scalar ttl("ttl", 7) -> i64;
                scalar ttl_match("ttl_match", 8) -> &'a str;
                scalar vlan_inner("vlan_inner", 9) -> bool;
                scalar source_ports_match("source_ports_match", 10) -> &'a str;
                model source_ports("source_ports", 11) -> item::SourcePorts<'a>;
                scalar destination_ports_match("destination_ports_match", 12) -> &'a str;
                model destination_ports("destination_ports", 13) -> item::DestinationPorts<'a>;
                model tcp_flags("tcp_flags", 14) -> item::TcpFlags<'a>;
                scalar copy_captive_portal("copy_captive_portal", 15) -> bool;
                scalar log("log", 16) -> bool;
                scalar icmp_type("icmp_type", 17) -> &'a str;
                scalar icmp_code("icmp_code", 18) -> &'a str;
                scalar nexthop_group("nexthop_group", 19) -> &'a str;
                scalar tracked("tracked", 20) -> bool;
                scalar dscp("dscp", 21) -> &'a str;
                scalar vlan_number("vlan_number", 22) -> i64;
                scalar vlan_mask("vlan_mask", 23) -> &'a str;
                scalar inner_vlan_number("inner_vlan_number", 24) -> i64;
                scalar inner_vlan_mask("inner_vlan_mask", 25) -> &'a str;
            }
        }

        pub mod item {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct SourcePorts {
                    scalar item (0) -> &'a str;
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct DestinationPorts {
                    scalar item (0) -> &'a str;
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct TcpFlags {
                    scalar item (0) -> &'a str;
                }
            }
        }
    }
}
