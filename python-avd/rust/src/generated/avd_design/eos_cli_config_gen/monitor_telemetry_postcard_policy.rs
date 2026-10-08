// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Ingress {
        model collection("collection", 0) -> ingress::Collection<'a>;
        model sample("sample", 1) -> ingress::Sample<'a>;
    }
}

pub mod ingress {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Collection {
            scalar source("source", 0) -> &'a str;
            scalar destination("destination", 1) -> &'a str;
            scalar version("version", 2) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Sample {
            scalar rate("rate", 0) -> i64;
            model tcp_udp_checksum("tcp_udp_checksum", 1) -> sample::TcpUdpChecksum<'a>;
        }
    }

    pub mod sample {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct TcpUdpChecksum {
                scalar value("value", 0) -> i64;
                scalar mask("mask", 1) -> &'a str;
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct MarkerVxlan {
        scalar enabled("enabled", 0) -> bool;
        scalar header_word_zero_bit("header_word_zero_bit", 1) -> i64;
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Profiles {
        model item (0) -> profiles::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod profiles {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar ingress_sample_policy("ingress_sample_policy", 1) -> &'a str;
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct SamplePolicies {
        model item (0) -> sample_policies::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod sample_policies {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            model match_rules("match_rules", 1) -> item::MatchRules<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct MatchRules {
                model item (0) -> match_rules::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod match_rules {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar name("name", 0) -> &'a str;
                    scalar field_type("type", 1) -> &'a str;
                    scalar destination_prefix("destination_prefix", 2) -> &'a str;
                    scalar source_prefix("source_prefix", 3) -> &'a str;
                    model protocols("protocols", 4) -> item::Protocols<'a>;
                }
            }

            pub mod item {

                ::validation::define_archive_indexed_list_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Protocols {
                        model item (0) -> protocols::Item<'a>;
                        primary_key_fields: [0];
                    }
                }

                pub mod protocols {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Item {
                            scalar protocol("protocol", 0) -> &'a str;
                            model source_ports("source_ports", 1) -> item::SourcePorts<'a>;
                            model destination_ports("destination_ports", 2) -> item::DestinationPorts<'a>;
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
                    }
                }
            }
        }
    }
}
