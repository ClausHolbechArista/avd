// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct ValidateHardware {
        scalar enabled("enabled", 0) -> bool;
        scalar min_power_supplies("min_power_supplies", 1) -> i64;
        scalar min_fans("min_fans", 2) -> i64;
        scalar min_supervisors("min_supervisors", 3) -> i64;
        scalar min_line_cards("min_line_cards", 4) -> i64;
        scalar min_fabric_cards("min_fabric_cards", 5) -> i64;
        model transceiver_manufacturers("transceiver_manufacturers", 6) -> validate_hardware::TransceiverManufacturers<'a>;
        scalar ignore_no_transceivers("ignore_no_transceivers", 7) -> bool;
    }
}

pub mod validate_hardware {

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct TransceiverManufacturers {
            scalar item (0) -> &'a str;
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct CvTags {
        model device_tags("device_tags", 0) -> cv_tags::DeviceTags<'a>;
        model interface_tags("interface_tags", 1) -> cv_tags::InterfaceTags<'a>;
    }
}

pub mod cv_tags {

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct DeviceTags {
            model item (0) -> device_tags::Item<'a>;
        }
    }

    pub mod device_tags {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                scalar value("value", 1) -> &'a str;
            }
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct InterfaceTags {
            model item (0) -> interface_tags::Item<'a>;
        }
    }

    pub mod interface_tags {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar interface("interface", 0) -> &'a str;
                model tags("tags", 1) -> item::Tags<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Tags {
                    model item (0) -> tags::Item<'a>;
                }
            }

            pub mod tags {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar name("name", 0) -> &'a str;
                        scalar value("value", 1) -> &'a str;
                    }
                }
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct CvPathfinder {
        scalar role("role", 0) -> &'a str;
        scalar region("region", 1) -> &'a str;
        scalar zone("zone", 2) -> &'a str;
        scalar site("site", 3) -> &'a str;
        scalar vtep_ip("vtep_ip", 4) -> &'a str;
        scalar ssl_profile("ssl_profile", 5) -> &'a str;
        scalar address("address", 6) -> &'a str;
        model pathfinders("pathfinders", 7) -> cv_pathfinder::Pathfinders<'a>;
        model interfaces("interfaces", 8) -> cv_pathfinder::Interfaces<'a>;
        model pathgroups("pathgroups", 9) -> cv_pathfinder::Pathgroups<'a>;
        model regions("regions", 10) -> cv_pathfinder::Regions<'a>;
        model vrfs("vrfs", 11) -> cv_pathfinder::Vrfs<'a>;
        model internet_exit_policies("internet_exit_policies", 12) -> cv_pathfinder::InternetExitPolicies<'a>;
        model applications("applications", 13) -> cv_pathfinder::Applications<'a>;
    }
}

pub mod cv_pathfinder {

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Pathfinders {
            model item (0) -> pathfinders::Item<'a>;
        }
    }

    pub mod pathfinders {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar vtep_ip("vtep_ip", 0) -> &'a str;
            }
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Interfaces {
            model item (0) -> interfaces::Item<'a>;
        }
    }

    pub mod interfaces {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                scalar carrier("carrier", 1) -> &'a str;
                scalar circuit_id("circuit_id", 2) -> &'a str;
                scalar pathgroup("pathgroup", 3) -> &'a str;
                scalar public_ip("public_ip", 4) -> &'a str;
            }
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Pathgroups {
            model item (0) -> pathgroups::Item<'a>;
        }
    }

    pub mod pathgroups {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                model carriers("carriers", 1) -> item::Carriers<'a>;
                model imported_carriers("imported_carriers", 2) -> item::ImportedCarriers<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Carriers {
                    model item (0) -> carriers::Item<'a>;
                }
            }

            pub mod carriers {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar name("name", 0) -> &'a str;
                    }
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct ImportedCarriers {
                    model item (0) -> imported_carriers::Item<'a>;
                }
            }

            pub mod imported_carriers {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar name("name", 0) -> &'a str;
                    }
                }
            }
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Regions {
            model item (0) -> regions::Item<'a>;
        }
    }

    pub mod regions {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar id("id", 0) -> i64;
                scalar name("name", 1) -> &'a str;
                model zones("zones", 2) -> item::Zones<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Zones {
                    model item (0) -> zones::Item<'a>;
                }
            }

            pub mod zones {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar id("id", 0) -> i64;
                        scalar name("name", 1) -> &'a str;
                        model sites("sites", 2) -> item::Sites<'a>;
                    }
                }

                pub mod item {

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Sites {
                            model item (0) -> sites::Item<'a>;
                        }
                    }

                    pub mod sites {

                        ::validation::define_archive_dict_view! {
                            #[derive(Clone, Copy, Debug)]
                            pub struct Item {
                                scalar id("id", 0) -> i64;
                                scalar name("name", 1) -> &'a str;
                                model location("location", 2) -> item::Location<'a>;
                            }
                        }

                        pub mod item {

                            ::validation::define_archive_dict_view! {
                                #[derive(Clone, Copy, Debug)]
                                pub struct Location {
                                    scalar address("address", 0) -> &'a str;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Vrfs {
            model item (0) -> vrfs::Item<'a>;
        }
    }

    pub mod vrfs {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                scalar vni("vni", 1) -> i64;
                model avts("avts", 2) -> item::Avts<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Avts {
                    model item (0) -> avts::Item<'a>;
                }
            }

            pub mod avts {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        model constraints("constraints", 0) -> item::Constraints<'a>;
                        scalar description("description", 1) -> &'a str;
                        scalar id("id", 2) -> i64;
                        scalar name("name", 3) -> &'a str;
                        model pathgroups("pathgroups", 4) -> item::Pathgroups<'a>;
                        model application_profiles("application_profiles", 5) -> item::ApplicationProfiles<'a>;
                    }
                }

                pub mod item {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Constraints {
                            scalar jitter("jitter", 0) -> i64;
                            scalar latency("latency", 1) -> i64;
                            scalar lossrate("lossrate", 2) -> &'a str;
                            scalar hop_count("hop_count", 3) -> &'a str;
                        }
                    }

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Pathgroups {
                            model item (0) -> pathgroups::Item<'a>;
                        }
                    }

                    pub mod pathgroups {

                        ::validation::define_archive_dict_view! {
                            #[derive(Clone, Copy, Debug)]
                            pub struct Item {
                                scalar name("name", 0) -> &'a str;
                                scalar preference("preference", 1) -> &'a str;
                            }
                        }
                    }

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct ApplicationProfiles {
                            scalar item (0) -> &'a str;
                        }
                    }
                }
            }
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct InternetExitPolicies {
            model item (0) -> internet_exit_policies::Item<'a>;
        }
    }

    pub mod internet_exit_policies {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                scalar field_type("type", 1) -> &'a str;
                scalar city("city", 2) -> &'a str;
                scalar country("country", 3) -> &'a str;
                scalar upload_bandwidth("upload_bandwidth", 4) -> i64;
                scalar download_bandwidth("download_bandwidth", 5) -> i64;
                scalar firewall("firewall", 6) -> bool;
                scalar ips_control("ips_control", 7) -> bool;
                scalar acceptable_use_policy("acceptable_use_policy", 8) -> bool;
                model vpn_credentials("vpn_credentials", 9) -> item::VpnCredentials<'a>;
                model tunnels("tunnels", 10) -> item::Tunnels<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct VpnCredentials {
                    model item (0) -> vpn_credentials::Item<'a>;
                }
            }

            pub mod vpn_credentials {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar fqdn("fqdn", 0) -> &'a str;
                        scalar vpn_type("vpn_type", 1) -> &'a str;
                        scalar pre_shared_key("pre_shared_key", 2) -> &'a str;
                    }
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Tunnels {
                    model item (0) -> tunnels::Item<'a>;
                }
            }

            pub mod tunnels {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar name("name", 0) -> &'a str;
                        scalar preference("preference", 1) -> &'a str;
                        model endpoint("endpoint", 2) -> item::Endpoint<'a>;
                    }
                }

                pub mod item {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Endpoint {
                            scalar ip_address("ip_address", 0) -> &'a str;
                            scalar datacenter("datacenter", 1) -> &'a str;
                            scalar city("city", 2) -> &'a str;
                            scalar country("country", 3) -> &'a str;
                            scalar region("region", 4) -> &'a str;
                            scalar latitude("latitude", 5) -> &'a str;
                            scalar longitude("longitude", 6) -> &'a str;
                        }
                    }
                }
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Applications {
            model profiles("profiles", 0) -> applications::Profiles<'a>;
            model categories("categories", 1) -> applications::Categories<'a>;
        }
    }

    pub mod applications {

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Profiles {
                model item (0) -> profiles::Item<'a>;
            }
        }

        pub mod profiles {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar name("name", 0) -> &'a str;
                    model builtin_applications("builtin_applications", 1) -> item::BuiltinApplications<'a>;
                    model user_defined_applications("user_defined_applications", 2) -> item::UserDefinedApplications<'a>;
                    model categories("categories", 3) -> item::Categories<'a>;
                    model transport_protocols("transport_protocols", 4) -> item::TransportProtocols<'a>;
                }
            }

            pub mod item {

                ::validation::define_archive_list_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct BuiltinApplications {
                        model item (0) -> builtin_applications::Item<'a>;
                    }
                }

                pub mod builtin_applications {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Item {
                            scalar name("name", 0) -> &'a str;
                            model services("services", 1) -> item::Services<'a>;
                        }
                    }

                    pub mod item {

                        ::validation::define_archive_list_view! {
                            #[derive(Clone, Copy, Debug)]
                            pub struct Services {
                                scalar item (0) -> &'a str;
                            }
                        }
                    }
                }

                ::validation::define_archive_list_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct UserDefinedApplications {
                        model item (0) -> user_defined_applications::Item<'a>;
                    }
                }

                pub mod user_defined_applications {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Item {
                            scalar name("name", 0) -> &'a str;
                        }
                    }
                }

                ::validation::define_archive_list_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Categories {
                        model item (0) -> categories::Item<'a>;
                    }
                }

                pub mod categories {

                    ::validation::define_archive_dict_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Item {
                            scalar category("category", 0) -> &'a str;
                            model services("services", 1) -> item::Services<'a>;
                        }
                    }

                    pub mod item {

                        ::validation::define_archive_list_view! {
                            #[derive(Clone, Copy, Debug)]
                            pub struct Services {
                                scalar item (0) -> &'a str;
                            }
                        }
                    }
                }

                ::validation::define_archive_list_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct TransportProtocols {
                        scalar item (0) -> &'a str;
                    }
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Categories {
                model builtin_applications("builtin_applications", 0) -> categories::BuiltinApplications<'a>;
                model user_defined_applications("user_defined_applications", 1) -> categories::UserDefinedApplications<'a>;
            }
        }

        pub mod categories {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct BuiltinApplications {
                    model item (0) -> builtin_applications::Item<'a>;
                }
            }

            pub mod builtin_applications {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar name("name", 0) -> &'a str;
                        scalar category("category", 1) -> &'a str;
                        model services("services", 2) -> item::Services<'a>;
                    }
                }

                pub mod item {

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Services {
                            scalar item (0) -> &'a str;
                        }
                    }
                }
            }

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct UserDefinedApplications {
                    model item (0) -> user_defined_applications::Item<'a>;
                }
            }

            pub mod user_defined_applications {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar name("name", 0) -> &'a str;
                        scalar category("category", 1) -> &'a str;
                    }
                }
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct DigitalTwin {
        scalar environment("environment", 0) -> &'a str;
        scalar node_type("node_type", 1) -> &'a str;
        scalar ip_addr("ip_addr", 2) -> &'a str;
        scalar version("version", 3) -> &'a str;
        scalar username("username", 4) -> &'a str;
        scalar password("password", 5) -> &'a str;
        scalar internet_access("internet_access", 6) -> bool;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Interfaces {
        model errdisable("errdisable", 0) -> interfaces::Errdisable<'a>;
    }
}

pub mod interfaces {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Errdisable {
            scalar only_avd_interfaces("only_avd_interfaces", 0) -> bool;
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Bgp {
        scalar check_tcp_queues("check_tcp_queues", 0) -> bool;
        scalar minimum_established_time("minimum_established_time", 1) -> i64;
    }
}
