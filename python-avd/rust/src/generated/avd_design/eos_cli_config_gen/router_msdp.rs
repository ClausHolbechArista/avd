// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct GroupLimits {
        model item (0) -> group_limits::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod group_limits {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar source_prefix("source_prefix", 0) -> &'a str;
            scalar limit("limit", 1) -> i64;
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Peers {
        model item (0) -> peers::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod peers {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar ipv4_address("ipv4_address", 0) -> &'a str;
            model default_peer("default_peer", 1) -> item::DefaultPeer<'a>;
            scalar local_interface("local_interface", 2) -> &'a str;
            scalar description("description", 3) -> &'a str;
            scalar disabled("disabled", 4) -> bool;
            scalar sa_limit("sa_limit", 5) -> i64;
            model mesh_groups("mesh_groups", 6) -> item::MeshGroups<'a>;
            model keepalive("keepalive", 7) -> item::Keepalive<'a>;
            model sa_filter("sa_filter", 8) -> item::SaFilter<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct DefaultPeer {
                scalar enabled("enabled", 0) -> bool;
                scalar prefix_list("prefix_list", 1) -> &'a str;
            }
        }

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct MeshGroups {
                model item (0) -> mesh_groups::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod mesh_groups {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar name("name", 0) -> &'a str;
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Keepalive {
                scalar keepalive_timer("keepalive_timer", 0) -> i64;
                scalar hold_timer("hold_timer", 1) -> i64;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct SaFilter {
                scalar in_list("in_list", 0) -> &'a str;
                scalar out_list("out_list", 1) -> &'a str;
            }
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Vrfs {
        model item (0) -> vrfs::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod vrfs {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar originator_id_local_interface("originator_id_local_interface", 1) -> &'a str;
            scalar rejected_limit("rejected_limit", 2) -> i64;
            scalar forward_register_packets("forward_register_packets", 3) -> bool;
            scalar connection_retry_interval("connection_retry_interval", 4) -> i64;
            model group_limits("group_limits", 5) -> item::GroupLimits<'a>;
            model peers("peers", 6) -> item::Peers<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct GroupLimits {
                model item (0) -> group_limits::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod group_limits {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar source_prefix("source_prefix", 0) -> &'a str;
                    scalar limit("limit", 1) -> i64;
                }
            }
        }

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Peers {
                model item (0) -> peers::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod peers {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar ipv4_address("ipv4_address", 0) -> &'a str;
                    model default_peer("default_peer", 1) -> item::DefaultPeer<'a>;
                    scalar local_interface("local_interface", 2) -> &'a str;
                    scalar description("description", 3) -> &'a str;
                    scalar disabled("disabled", 4) -> bool;
                    scalar sa_limit("sa_limit", 5) -> i64;
                    model mesh_groups("mesh_groups", 6) -> item::MeshGroups<'a>;
                    model keepalive("keepalive", 7) -> item::Keepalive<'a>;
                    model sa_filter("sa_filter", 8) -> item::SaFilter<'a>;
                }
            }

            pub mod item {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct DefaultPeer {
                        scalar enabled("enabled", 0) -> bool;
                        scalar prefix_list("prefix_list", 1) -> &'a str;
                    }
                }

                ::validation::define_archive_indexed_list_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct MeshGroups {
                        model item (0) -> mesh_groups::Item<'a>;
                        primary_key_fields: [0];
                    }
                }

                pub mod mesh_groups {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Item {
                            scalar name("name", 0) -> &'a str;
                        }
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Keepalive {
                        scalar keepalive_timer("keepalive_timer", 0) -> i64;
                        scalar hold_timer("hold_timer", 1) -> i64;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct SaFilter {
                        scalar in_list("in_list", 0) -> &'a str;
                        scalar out_list("out_list", 1) -> &'a str;
                    }
                }
            }
        }
    }
}
