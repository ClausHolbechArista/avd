// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct RouterId {
        scalar ipv4("ipv4", 0) -> &'a str;
        scalar ipv6("ipv6", 1) -> &'a str;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct SegmentRouting {
        scalar colored_tunnel_rib("colored_tunnel_rib", 0) -> bool;
        model policy_endpoints("policy_endpoints", 1) -> segment_routing::PolicyEndpoints<'a>;
    }
}

pub mod segment_routing {

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct PolicyEndpoints {
            model item (0) -> policy_endpoints::Item<'a>;
        }
    }

    pub mod policy_endpoints {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar address("address", 0) -> &'a str;
                model colors("colors", 1) -> item::Colors<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_indexed_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Colors {
                    model item (0) -> colors::Item<'a>;
                    primary_key_fields: [0];
                }
            }

            pub mod colors {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar value("value", 0) -> i64;
                        scalar binding_sid("binding_sid", 1) -> i64;
                        scalar description("description", 2) -> &'a str;
                        scalar name("name", 3) -> &'a str;
                        scalar sbfd_remote_discriminator("sbfd_remote_discriminator", 4) -> &'a str;
                        model path_group("path_group", 5) -> item::PathGroup<'a>;
                    }
                }

                pub mod item {

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct PathGroup {
                            model item (0) -> path_group::Item<'a>;
                        }
                    }

                    pub mod path_group {

                        ::validation::define_archive_dict_view! {
                            #[derive(Clone, Copy, Debug)]
                            pub struct Item {
                                scalar preference("preference", 0) -> i64;
                                scalar explicit_null("explicit_null", 1) -> &'a str;
                                model segment_list("segment_list", 2) -> item::SegmentList<'a>;
                            }
                        }

                        pub mod item {

                            ::validation::define_archive_list_view! {
                                #[derive(Clone, Copy, Debug)]
                                pub struct SegmentList {
                                    model item (0) -> segment_list::Item<'a>;
                                }
                            }

                            pub mod segment_list {

                                ::validation::define_archive_dict_view! {
                                    #[derive(Clone, Copy, Debug)]
                                    pub struct Item {
                                        scalar label_stack("label_stack", 0) -> &'a str;
                                        scalar weight("weight", 1) -> i64;
                                        scalar index("index", 2) -> i64;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct FlexAlgos {
        model item (0) -> flex_algos::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod flex_algos {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar number("number", 0) -> i64;
            scalar name("name", 1) -> &'a str;
            model administrative_group("administrative_group", 2) -> item::AdministrativeGroup<'a>;
            scalar metric("metric", 3) -> &'a str;
            scalar priority("priority", 4) -> i64;
            scalar color("color", 5) -> i64;
            scalar srlg_exclude("srlg_exclude", 6) -> &'a str;
        }
    }

    pub mod item {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct AdministrativeGroup {
                scalar include_all("include_all", 0) -> &'a str;
                scalar include_any("include_any", 1) -> &'a str;
                scalar exclude("exclude", 2) -> &'a str;
            }
        }
    }
}
