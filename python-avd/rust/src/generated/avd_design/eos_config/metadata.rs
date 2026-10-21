// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct Metadata<'a, Mode> {
    pub is_deployed: ::data_views::Field<bool>,
    pub platform: ::data_views::Field<&'a str>,
    pub system_mac_address: ::data_views::Field<&'a str>,
    pub rack: ::data_views::Field<&'a str>,
    pub pod_name: ::data_views::Field<&'a str>,
    pub dc_name: ::data_views::Field<&'a str>,
    pub fabric_name: ::data_views::Field<&'a str>,
    pub serial_number: ::data_views::Field<&'a str>,
    pub validate_hardware: ::data_views::Field<ValidateHardware<'a, Mode>>,
    pub cv_tags: ::data_views::Field<CvTags<'a, Mode>>,
    pub cv_use_static_config_manifest: ::data_views::Field<bool>,
    pub cv_pathfinder: ::data_views::Field<CvPathfinder<'a, Mode>>,
    pub digital_twin: ::data_views::Field<DigitalTwin<'a, Mode>>,
    pub validate_no_errors_period: ::data_views::Field<i64>,
    pub exclude_as_extra_fabric_validation_target: ::data_views::Field<bool>,
    pub interfaces: ::data_views::Field<Interfaces<'a, Mode>>,
    pub bgp: ::data_views::Field<Bgp<'a, Mode>>,
}

#[::data_views::data_view]
pub struct ValidateHardware<'a, Mode> {
    pub enabled: ::data_views::Field<bool>,
    pub min_power_supplies: ::data_views::Field<i64>,
    pub min_fans: ::data_views::Field<i64>,
    pub min_supervisors: ::data_views::Field<i64>,
    pub min_line_cards: ::data_views::Field<i64>,
    pub min_fabric_cards: ::data_views::Field<i64>,
    pub transceiver_manufacturers: ::data_views::Field<validate_hardware::TransceiverManufacturers<'a, Mode>>,
    pub ignore_no_transceivers: ::data_views::Field<bool>,
}

pub mod validate_hardware {

    #[::data_views::data_view(list)]
    pub struct TransceiverManufacturers<'a, Mode> (::data_views::Field<&'a str>);
}

#[::data_views::data_view]
pub struct CvTags<'a, Mode> {
    pub device_tags: ::data_views::Field<cv_tags::DeviceTags<'a, Mode>>,
    pub interface_tags: ::data_views::Field<cv_tags::InterfaceTags<'a, Mode>>,
}

pub mod cv_tags {

    #[::data_views::data_view(list)]
    pub struct DeviceTags<'a, Mode> (::data_views::Field<device_tags::Item<'a, Mode>>);

    pub mod device_tags {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::data_views::RequiredValue<&'a str, Mode>,
            pub value: ::data_views::RequiredValue<&'a str, Mode>,
        }
    }

    #[::data_views::data_view(list)]
    pub struct InterfaceTags<'a, Mode> (::data_views::Field<interface_tags::Item<'a, Mode>>);

    pub mod interface_tags {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub interface: ::data_views::RequiredValue<&'a str, Mode>,
            pub tags: ::data_views::Field<item::Tags<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view(list)]
            pub struct Tags<'a, Mode> (::data_views::Field<tags::Item<'a, Mode>>);

            pub mod tags {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub name: ::data_views::RequiredValue<&'a str, Mode>,
                    pub value: ::data_views::RequiredValue<&'a str, Mode>,
                }
            }
        }
    }
}

#[::data_views::data_view]
pub struct CvPathfinder<'a, Mode> {
    pub role: ::data_views::Field<&'a str>,
    pub region: ::data_views::Field<&'a str>,
    pub zone: ::data_views::Field<&'a str>,
    pub site: ::data_views::Field<&'a str>,
    pub vtep_ip: ::data_views::Field<&'a str>,
    pub ssl_profile: ::data_views::Field<&'a str>,
    pub address: ::data_views::Field<&'a str>,
    pub pathfinders: ::data_views::Field<cv_pathfinder::Pathfinders<'a, Mode>>,
    pub interfaces: ::data_views::Field<cv_pathfinder::Interfaces<'a, Mode>>,
    pub pathgroups: ::data_views::Field<cv_pathfinder::Pathgroups<'a, Mode>>,
    pub regions: ::data_views::Field<cv_pathfinder::Regions<'a, Mode>>,
    pub vrfs: ::data_views::Field<cv_pathfinder::Vrfs<'a, Mode>>,
    pub internet_exit_policies: ::data_views::Field<cv_pathfinder::InternetExitPolicies<'a, Mode>>,
    pub applications: ::data_views::Field<cv_pathfinder::Applications<'a, Mode>>,
}

pub mod cv_pathfinder {

    #[::data_views::data_view(list)]
    pub struct Pathfinders<'a, Mode> (::data_views::Field<pathfinders::Item<'a, Mode>>);

    pub mod pathfinders {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub vtep_ip: ::data_views::RequiredValue<&'a str, Mode>,
        }
    }

    #[::data_views::data_view(list)]
    pub struct Interfaces<'a, Mode> (::data_views::Field<interfaces::Item<'a, Mode>>);

    pub mod interfaces {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::data_views::Field<&'a str>,
            pub carrier: ::data_views::Field<&'a str>,
            pub circuit_id: ::data_views::Field<&'a str>,
            pub pathgroup: ::data_views::Field<&'a str>,
            pub public_ip: ::data_views::Field<&'a str>,
        }
    }

    #[::data_views::data_view(list)]
    pub struct Pathgroups<'a, Mode> (::data_views::Field<pathgroups::Item<'a, Mode>>);

    pub mod pathgroups {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::data_views::RequiredValue<&'a str, Mode>,
            pub carriers: ::data_views::Field<item::Carriers<'a, Mode>>,
            pub imported_carriers: ::data_views::Field<item::ImportedCarriers<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view(list)]
            pub struct Carriers<'a, Mode> (::data_views::Field<carriers::Item<'a, Mode>>);

            pub mod carriers {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub name: ::data_views::Field<&'a str>,
                }
            }

            #[::data_views::data_view(list)]
            pub struct ImportedCarriers<'a, Mode> (::data_views::Field<imported_carriers::Item<'a, Mode>>);

            pub mod imported_carriers {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub name: ::data_views::Field<&'a str>,
                }
            }
        }
    }

    #[::data_views::data_view(list)]
    pub struct Regions<'a, Mode> (::data_views::Field<regions::Item<'a, Mode>>);

    pub mod regions {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub id: ::data_views::Field<i64>,
            pub name: ::data_views::Field<&'a str>,
            pub zones: ::data_views::Field<item::Zones<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view(list)]
            pub struct Zones<'a, Mode> (::data_views::Field<zones::Item<'a, Mode>>);

            pub mod zones {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub id: ::data_views::Field<i64>,
                    pub name: ::data_views::Field<&'a str>,
                    pub sites: ::data_views::Field<item::Sites<'a, Mode>>,
                }

                pub mod item {

                    #[::data_views::data_view(list)]
                    pub struct Sites<'a, Mode> (::data_views::Field<sites::Item<'a, Mode>>);

                    pub mod sites {

                        #[::data_views::data_view]
                        pub struct Item<'a, Mode> {
                            pub id: ::data_views::Field<i64>,
                            pub name: ::data_views::Field<&'a str>,
                            pub location: ::data_views::Field<item::Location<'a, Mode>>,
                        }

                        pub mod item {

                            #[::data_views::data_view]
                            pub struct Location<'a, Mode> {
                                pub address: ::data_views::Field<&'a str>,
                            }
                        }
                    }
                }
            }
        }
    }

    #[::data_views::data_view(list)]
    pub struct Vrfs<'a, Mode> (::data_views::Field<vrfs::Item<'a, Mode>>);

    pub mod vrfs {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::data_views::Field<&'a str>,
            pub vni: ::data_views::Field<i64>,
            pub avts: ::data_views::Field<item::Avts<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view(list)]
            pub struct Avts<'a, Mode> (::data_views::Field<avts::Item<'a, Mode>>);

            pub mod avts {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub constraints: ::data_views::Field<item::Constraints<'a, Mode>>,
                    pub description: ::data_views::Field<&'a str>,
                    pub id: ::data_views::Field<i64>,
                    pub name: ::data_views::Field<&'a str>,
                    pub pathgroups: ::data_views::Field<item::Pathgroups<'a, Mode>>,
                    pub application_profiles: ::data_views::Field<item::ApplicationProfiles<'a, Mode>>,
                }

                pub mod item {

                    #[::data_views::data_view]
                    pub struct Constraints<'a, Mode> {
                        pub jitter: ::data_views::Field<i64>,
                        pub latency: ::data_views::Field<i64>,
                        pub lossrate: ::data_views::Field<&'a str>,
                        pub hop_count: ::data_views::Field<&'a str>,
                    }

                    #[::data_views::data_view(list)]
                    pub struct Pathgroups<'a, Mode> (::data_views::Field<pathgroups::Item<'a, Mode>>);

                    pub mod pathgroups {

                        #[::data_views::data_view]
                        pub struct Item<'a, Mode> {
                            pub name: ::data_views::Field<&'a str>,
                            pub preference: ::data_views::Field<&'a str>,
                        }
                    }

                    #[::data_views::data_view(list)]
                    pub struct ApplicationProfiles<'a, Mode> (::data_views::Field<&'a str>);
                }
            }
        }
    }

    #[::data_views::data_view(list)]
    pub struct InternetExitPolicies<'a, Mode> (::data_views::Field<internet_exit_policies::Item<'a, Mode>>);

    pub mod internet_exit_policies {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::data_views::RequiredValue<&'a str, Mode>,
            #[data_view(rename = "type")]
            pub field_type: ::data_views::RequiredValue<&'a str, Mode>,
            pub city: ::data_views::RequiredValue<&'a str, Mode>,
            pub country: ::data_views::RequiredValue<&'a str, Mode>,
            pub upload_bandwidth: ::data_views::Field<i64>,
            pub download_bandwidth: ::data_views::Field<i64>,
            pub firewall: ::data_views::RequiredValue<bool, Mode>,
            pub ips_control: ::data_views::RequiredValue<bool, Mode>,
            pub acceptable_use_policy: ::data_views::RequiredValue<bool, Mode>,
            pub vpn_credentials: ::data_views::RequiredValue<item::VpnCredentials<'a, Mode>, Mode>,
            pub tunnels: ::data_views::RequiredValue<item::Tunnels<'a, Mode>, Mode>,
        }

        pub mod item {

            #[::data_views::data_view(list)]
            pub struct VpnCredentials<'a, Mode> (::data_views::Field<vpn_credentials::Item<'a, Mode>>);

            pub mod vpn_credentials {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub fqdn: ::data_views::RequiredValue<&'a str, Mode>,
                    pub vpn_type: ::data_views::RequiredValue<&'a str, Mode>,
                    pub pre_shared_key: ::data_views::RequiredValue<&'a str, Mode>,
                }
            }

            #[::data_views::data_view(list)]
            pub struct Tunnels<'a, Mode> (::data_views::Field<tunnels::Item<'a, Mode>>);

            pub mod tunnels {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub name: ::data_views::RequiredValue<&'a str, Mode>,
                    pub preference: ::data_views::RequiredValue<&'a str, Mode>,
                    pub endpoint: ::data_views::RequiredValue<item::Endpoint<'a, Mode>, Mode>,
                }

                pub mod item {

                    #[::data_views::data_view]
                    pub struct Endpoint<'a, Mode> {
                        pub ip_address: ::data_views::RequiredValue<&'a str, Mode>,
                        pub datacenter: ::data_views::RequiredValue<&'a str, Mode>,
                        pub city: ::data_views::RequiredValue<&'a str, Mode>,
                        pub country: ::data_views::RequiredValue<&'a str, Mode>,
                        pub region: ::data_views::RequiredValue<&'a str, Mode>,
                        pub latitude: ::data_views::RequiredValue<&'a str, Mode>,
                        pub longitude: ::data_views::RequiredValue<&'a str, Mode>,
                    }
                }
            }
        }
    }

    #[::data_views::data_view]
    pub struct Applications<'a, Mode> {
        pub profiles: ::data_views::Field<applications::Profiles<'a, Mode>>,
        pub categories: ::data_views::Field<applications::Categories<'a, Mode>>,
    }

    pub mod applications {

        #[::data_views::data_view(list)]
        pub struct Profiles<'a, Mode> (::data_views::Field<profiles::Item<'a, Mode>>);

        pub mod profiles {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::data_views::Field<&'a str>,
                pub builtin_applications: ::data_views::Field<item::BuiltinApplications<'a, Mode>>,
                pub user_defined_applications: ::data_views::Field<item::UserDefinedApplications<'a, Mode>>,
                pub categories: ::data_views::Field<item::Categories<'a, Mode>>,
                pub transport_protocols: ::data_views::Field<item::TransportProtocols<'a, Mode>>,
            }

            pub mod item {

                #[::data_views::data_view(list)]
                pub struct BuiltinApplications<'a, Mode> (::data_views::Field<builtin_applications::Item<'a, Mode>>);

                pub mod builtin_applications {

                    #[::data_views::data_view]
                    pub struct Item<'a, Mode> {
                        pub name: ::data_views::Field<&'a str>,
                        pub services: ::data_views::Field<item::Services<'a, Mode>>,
                    }

                    pub mod item {

                        #[::data_views::data_view(list)]
                        pub struct Services<'a, Mode> (::data_views::Field<&'a str>);
                    }
                }

                #[::data_views::data_view(list)]
                pub struct UserDefinedApplications<'a, Mode> (::data_views::Field<user_defined_applications::Item<'a, Mode>>);

                pub mod user_defined_applications {

                    #[::data_views::data_view]
                    pub struct Item<'a, Mode> {
                        pub name: ::data_views::Field<&'a str>,
                    }
                }

                #[::data_views::data_view(list)]
                pub struct Categories<'a, Mode> (::data_views::Field<categories::Item<'a, Mode>>);

                pub mod categories {

                    #[::data_views::data_view]
                    pub struct Item<'a, Mode> {
                        pub category: ::data_views::Field<&'a str>,
                        pub services: ::data_views::Field<item::Services<'a, Mode>>,
                    }

                    pub mod item {

                        #[::data_views::data_view(list)]
                        pub struct Services<'a, Mode> (::data_views::Field<&'a str>);
                    }
                }

                #[::data_views::data_view(list)]
                pub struct TransportProtocols<'a, Mode> (::data_views::Field<&'a str>);
            }
        }

        #[::data_views::data_view]
        pub struct Categories<'a, Mode> {
            pub builtin_applications: ::data_views::Field<categories::BuiltinApplications<'a, Mode>>,
            pub user_defined_applications: ::data_views::Field<categories::UserDefinedApplications<'a, Mode>>,
        }

        pub mod categories {

            #[::data_views::data_view(list)]
            pub struct BuiltinApplications<'a, Mode> (::data_views::Field<builtin_applications::Item<'a, Mode>>);

            pub mod builtin_applications {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub name: ::data_views::Field<&'a str>,
                    pub category: ::data_views::Field<&'a str>,
                    pub services: ::data_views::Field<item::Services<'a, Mode>>,
                }

                pub mod item {

                    #[::data_views::data_view(list)]
                    pub struct Services<'a, Mode> (::data_views::Field<&'a str>);
                }
            }

            #[::data_views::data_view(list)]
            pub struct UserDefinedApplications<'a, Mode> (::data_views::Field<user_defined_applications::Item<'a, Mode>>);

            pub mod user_defined_applications {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub name: ::data_views::Field<&'a str>,
                    pub category: ::data_views::Field<&'a str>,
                }
            }
        }
    }
}

#[::data_views::data_view]
pub struct DigitalTwin<'a, Mode> {
    pub environment: ::data_views::Field<&'a str>,
    pub node_type: ::data_views::Field<&'a str>,
    pub ip_addr: ::data_views::Field<&'a str>,
    pub version: ::data_views::Field<&'a str>,
    pub username: ::data_views::Field<&'a str>,
    pub password: ::data_views::Field<&'a str>,
    pub internet_access: ::data_views::Field<bool>,
}

#[::data_views::data_view(python(name = "EOSConfig.Metadata.Interfaces", schema = Dict(640)))]
pub struct Interfaces<'a, Mode> {
    #[data_view(schema = Dict(639))]
    pub errdisable: ::data_views::Field<interfaces::Errdisable<'a, Mode>>,
}

pub mod interfaces {

    #[::data_views::data_view(python(name = "EOSConfig.Metadata.Interfaces.Errdisable", schema = Dict(639)))]
    pub struct Errdisable<'a, Mode> {
        #[data_view(schema = Bool(264))]
        pub only_avd_interfaces: ::data_views::Field<bool>,
    }
}

#[::data_views::data_view(python(name = "EOSConfig.Metadata.Bgp", schema = Dict(641)))]
pub struct Bgp<'a, Mode> {
    #[data_view(schema = Bool(265))]
    pub check_tcp_queues: ::data_views::Field<bool>,
    #[data_view(schema = Int(255))]
    pub minimum_established_time: ::data_views::Field<i64>,
}
