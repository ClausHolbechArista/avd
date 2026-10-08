// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar name("name", 0) -> &'a str;
        model entries("entries", 1) -> item::Entries<'a>;
        scalar counters_per_entry("counters_per_entry", 2) -> bool;
        model sequence_numbers("sequence_numbers", 3) -> item::SequenceNumbers<'a>;
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
                scalar source("source", 0) -> &'a str;
                scalar destination("destination", 1) -> &'a str;
                scalar protocol("protocol", 2) -> &'a str;
                scalar hop_limit("hop_limit", 3) -> i64;
                scalar hop_limit_match("hop_limit_match", 4) -> &'a str;
                scalar dscp_mask("dscp_mask", 5) -> &'a str;
                scalar sequence("sequence", 6) -> i64;
                scalar remark("remark", 7) -> &'a str;
                scalar action("action", 8) -> &'a str;
                scalar source_ports_match("source_ports_match", 9) -> &'a str;
                model source_ports("source_ports", 10) -> item::SourcePorts<'a>;
                scalar destination_ports_match("destination_ports_match", 11) -> &'a str;
                model destination_ports("destination_ports", 12) -> item::DestinationPorts<'a>;
                model tcp_flags("tcp_flags", 13) -> item::TcpFlags<'a>;
                scalar copy_captive_portal("copy_captive_portal", 14) -> bool;
                scalar log("log", 15) -> bool;
                scalar icmp_type("icmp_type", 16) -> &'a str;
                scalar icmp_code("icmp_code", 17) -> &'a str;
                scalar nexthop_group("nexthop_group", 18) -> &'a str;
                scalar tracked("tracked", 19) -> bool;
                scalar dscp("dscp", 20) -> &'a str;
                scalar vlan_number("vlan_number", 21) -> i64;
                scalar vlan_mask("vlan_mask", 22) -> &'a str;
                scalar inner_vlan_number("inner_vlan_number", 23) -> i64;
                scalar inner_vlan_mask("inner_vlan_mask", 24) -> &'a str;
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

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct SequenceNumbers {
            model item (0) -> sequence_numbers::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod sequence_numbers {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar sequence("sequence", 0) -> i64;
                scalar action("action", 1) -> &'a str;
            }
        }
    }
}
