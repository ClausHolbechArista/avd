// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Policies {
        model sand_profiles("sand_profiles", 0) -> policies::SandProfiles<'a>;
    }
}

pub mod policies {

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct SandProfiles {
            model item (0) -> sand_profiles::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod sand_profiles {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                model fields("fields", 1) -> item::Fields<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Fields {
                    model udp("udp", 0) -> fields::Udp<'a>;
                }
            }

            pub mod fields {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Udp {
                        scalar dst_port("dst_port", 0) -> i64;
                        scalar payload_bytes("payload_bytes", 1) -> &'a str;
                        model field_match("match", 2) -> udp::FieldMatch<'a>;
                    }
                }

                pub mod udp {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct FieldMatch {
                            scalar payload_bits("payload_bits", 0) -> &'a str;
                            scalar pattern("pattern", 1) -> &'a str;
                            scalar hash_payload_bytes("hash_payload_bytes", 2) -> &'a str;
                        }
                    }
                }
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Cluster {
        scalar destination_grouping("destination_grouping", 0) -> &'a str;
        scalar prefix_length("prefix_length", 1) -> i64;
        scalar forwarding_type("forwarding_type", 2) -> &'a str;
        scalar load_balance_method_flow_round_robin("load_balance_method_flow_round_robin", 3) -> bool;
        model flow("flow", 4) -> cluster::Flow<'a>;
        model port_groups("port_groups", 5) -> cluster::PortGroups<'a>;
    }
}

pub mod cluster {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Flow {
            scalar monitor("monitor", 0) -> bool;
            scalar source_learning_aging_timeout("source_learning_aging_timeout", 1) -> i64;
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct PortGroups {
            model item (0) -> port_groups::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod port_groups {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar group("group", 0) -> &'a str;
                scalar balance_factor("balance_factor", 1) -> i64;
                scalar interface("interface", 2) -> &'a str;
                model flow("flow", 3) -> item::Flow<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Flow {
                    scalar limit("limit", 0) -> i64;
                    scalar warning("warning", 1) -> i64;
                    model exhaustion_action("exhaustion_action", 2) -> flow::ExhaustionAction<'a>;
                }
            }

            pub mod flow {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct ExhaustionAction {
                        scalar dscp("dscp", 0) -> i64;
                        scalar traffic_class("traffic_class", 1) -> i64;
                    }
                }
            }
        }
    }
}
