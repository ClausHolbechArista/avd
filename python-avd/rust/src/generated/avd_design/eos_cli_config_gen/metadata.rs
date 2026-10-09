// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct ValidateHardware<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub min_power_supplies: ::validated_data::Field<i64>,
    pub min_fans: ::validated_data::Field<i64>,
    pub min_supervisors: ::validated_data::Field<i64>,
    pub min_line_cards: ::validated_data::Field<i64>,
    pub min_fabric_cards: ::validated_data::Field<i64>,
    pub transceiver_manufacturers: ::validated_data::Field<validate_hardware::TransceiverManufacturers<'a, Mode>>,
    pub ignore_no_transceivers: ::validated_data::Field<bool>,
}

pub mod validate_hardware {

    #[::validated_data::data_view(list)]
    pub struct TransceiverManufacturers<'a, Mode> (::validated_data::Field<&'a str>);
}

#[::validated_data::data_view]
pub struct CvTags<'a, Mode> {
    pub device_tags: ::validated_data::Field<cv_tags::DeviceTags<'a, Mode>>,
    pub interface_tags: ::validated_data::Field<cv_tags::InterfaceTags<'a, Mode>>,
}

pub mod cv_tags {

    #[::validated_data::data_view(list)]
    pub struct DeviceTags<'a, Mode> (::validated_data::Field<device_tags::Item<'a, Mode>>);

    pub mod device_tags {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::RequiredValue<&'a str, Mode>,
            pub value: ::validated_data::RequiredValue<&'a str, Mode>,
        }
    }

    #[::validated_data::data_view(list)]
    pub struct InterfaceTags<'a, Mode> (::validated_data::Field<interface_tags::Item<'a, Mode>>);

    pub mod interface_tags {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub interface: ::validated_data::RequiredValue<&'a str, Mode>,
            pub tags: ::validated_data::Field<item::Tags<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view(list)]
            pub struct Tags<'a, Mode> (::validated_data::Field<tags::Item<'a, Mode>>);

            pub mod tags {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub name: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub value: ::validated_data::RequiredValue<&'a str, Mode>,
                }
            }
        }
    }
}

#[::validated_data::data_view]
pub struct CvPathfinder<'a, Mode> {
    pub role: ::validated_data::Field<&'a str>,
    pub region: ::validated_data::Field<&'a str>,
    pub zone: ::validated_data::Field<&'a str>,
    pub site: ::validated_data::Field<&'a str>,
    pub vtep_ip: ::validated_data::Field<&'a str>,
    pub ssl_profile: ::validated_data::Field<&'a str>,
    pub address: ::validated_data::Field<&'a str>,
    pub pathfinders: ::validated_data::Field<cv_pathfinder::Pathfinders<'a, Mode>>,
    pub interfaces: ::validated_data::Field<cv_pathfinder::Interfaces<'a, Mode>>,
    pub pathgroups: ::validated_data::Field<cv_pathfinder::Pathgroups<'a, Mode>>,
    pub regions: ::validated_data::Field<cv_pathfinder::Regions<'a, Mode>>,
    pub vrfs: ::validated_data::Field<cv_pathfinder::Vrfs<'a, Mode>>,
    pub internet_exit_policies: ::validated_data::Field<cv_pathfinder::InternetExitPolicies<'a, Mode>>,
    pub applications: ::validated_data::Field<cv_pathfinder::Applications<'a, Mode>>,
}

pub mod cv_pathfinder {

    #[::validated_data::data_view(list)]
    pub struct Pathfinders<'a, Mode> (::validated_data::Field<pathfinders::Item<'a, Mode>>);

    pub mod pathfinders {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub vtep_ip: ::validated_data::RequiredValue<&'a str, Mode>,
        }
    }

    #[::validated_data::data_view(list)]
    pub struct Interfaces<'a, Mode> (::validated_data::Field<interfaces::Item<'a, Mode>>);

    pub mod interfaces {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub carrier: ::validated_data::Field<&'a str>,
            pub circuit_id: ::validated_data::Field<&'a str>,
            pub pathgroup: ::validated_data::Field<&'a str>,
            pub public_ip: ::validated_data::Field<&'a str>,
        }
    }

    #[::validated_data::data_view(list)]
    pub struct Pathgroups<'a, Mode> (::validated_data::Field<pathgroups::Item<'a, Mode>>);

    pub mod pathgroups {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::RequiredValue<&'a str, Mode>,
            pub carriers: ::validated_data::Field<item::Carriers<'a, Mode>>,
            pub imported_carriers: ::validated_data::Field<item::ImportedCarriers<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view(list)]
            pub struct Carriers<'a, Mode> (::validated_data::Field<carriers::Item<'a, Mode>>);

            pub mod carriers {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub name: ::validated_data::Field<&'a str>,
                }
            }

            #[::validated_data::data_view(list)]
            pub struct ImportedCarriers<'a, Mode> (::validated_data::Field<imported_carriers::Item<'a, Mode>>);

            pub mod imported_carriers {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub name: ::validated_data::Field<&'a str>,
                }
            }
        }
    }

    #[::validated_data::data_view(list)]
    pub struct Regions<'a, Mode> (::validated_data::Field<regions::Item<'a, Mode>>);

    pub mod regions {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub id: ::validated_data::Field<i64>,
            pub name: ::validated_data::Field<&'a str>,
            pub zones: ::validated_data::Field<item::Zones<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view(list)]
            pub struct Zones<'a, Mode> (::validated_data::Field<zones::Item<'a, Mode>>);

            pub mod zones {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub id: ::validated_data::Field<i64>,
                    pub name: ::validated_data::Field<&'a str>,
                    pub sites: ::validated_data::Field<item::Sites<'a, Mode>>,
                }

                pub mod item {

                    #[::validated_data::data_view(list)]
                    pub struct Sites<'a, Mode> (::validated_data::Field<sites::Item<'a, Mode>>);

                    pub mod sites {

                        #[::validated_data::data_view]
                        pub struct Item<'a, Mode> {
                            pub id: ::validated_data::Field<i64>,
                            pub name: ::validated_data::Field<&'a str>,
                            pub location: ::validated_data::Field<item::Location<'a, Mode>>,
                        }

                        pub mod item {

                            #[::validated_data::data_view]
                            pub struct Location<'a, Mode> {
                                pub address: ::validated_data::Field<&'a str>,
                            }
                        }
                    }
                }
            }
        }
    }

    #[::validated_data::data_view(list)]
    pub struct Vrfs<'a, Mode> (::validated_data::Field<vrfs::Item<'a, Mode>>);

    pub mod vrfs {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub vni: ::validated_data::Field<i64>,
            pub avts: ::validated_data::Field<item::Avts<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view(list)]
            pub struct Avts<'a, Mode> (::validated_data::Field<avts::Item<'a, Mode>>);

            pub mod avts {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub constraints: ::validated_data::Field<item::Constraints<'a, Mode>>,
                    pub description: ::validated_data::Field<&'a str>,
                    pub id: ::validated_data::Field<i64>,
                    pub name: ::validated_data::Field<&'a str>,
                    pub pathgroups: ::validated_data::Field<item::Pathgroups<'a, Mode>>,
                    pub application_profiles: ::validated_data::Field<item::ApplicationProfiles<'a, Mode>>,
                }

                pub mod item {

                    #[::validated_data::data_view]
                    pub struct Constraints<'a, Mode> {
                        pub jitter: ::validated_data::Field<i64>,
                        pub latency: ::validated_data::Field<i64>,
                        pub lossrate: ::validated_data::Field<&'a str>,
                        pub hop_count: ::validated_data::Field<&'a str>,
                    }

                    #[::validated_data::data_view(list)]
                    pub struct Pathgroups<'a, Mode> (::validated_data::Field<pathgroups::Item<'a, Mode>>);

                    pub mod pathgroups {

                        #[::validated_data::data_view]
                        pub struct Item<'a, Mode> {
                            pub name: ::validated_data::Field<&'a str>,
                            pub preference: ::validated_data::Field<&'a str>,
                        }
                    }

                    #[::validated_data::data_view(list)]
                    pub struct ApplicationProfiles<'a, Mode> (::validated_data::Field<&'a str>);
                }
            }
        }
    }

    #[::validated_data::data_view(list)]
    pub struct InternetExitPolicies<'a, Mode> (::validated_data::Field<internet_exit_policies::Item<'a, Mode>>);

    pub mod internet_exit_policies {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::RequiredValue<&'a str, Mode>,
            #[data_view(rename = "type")]
            pub field_type: ::validated_data::RequiredValue<&'a str, Mode>,
            pub city: ::validated_data::RequiredValue<&'a str, Mode>,
            pub country: ::validated_data::RequiredValue<&'a str, Mode>,
            pub upload_bandwidth: ::validated_data::Field<i64>,
            pub download_bandwidth: ::validated_data::Field<i64>,
            pub firewall: ::validated_data::RequiredValue<bool, Mode>,
            pub ips_control: ::validated_data::RequiredValue<bool, Mode>,
            pub acceptable_use_policy: ::validated_data::RequiredValue<bool, Mode>,
            pub vpn_credentials: ::validated_data::RequiredValue<item::VpnCredentials<'a, Mode>, Mode>,
            pub tunnels: ::validated_data::RequiredValue<item::Tunnels<'a, Mode>, Mode>,
        }

        pub mod item {

            #[::validated_data::data_view(list)]
            pub struct VpnCredentials<'a, Mode> (::validated_data::Field<vpn_credentials::Item<'a, Mode>>);

            pub mod vpn_credentials {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub fqdn: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub vpn_type: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub pre_shared_key: ::validated_data::RequiredValue<&'a str, Mode>,
                }
            }

            #[::validated_data::data_view(list)]
            pub struct Tunnels<'a, Mode> (::validated_data::Field<tunnels::Item<'a, Mode>>);

            pub mod tunnels {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub name: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub preference: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub endpoint: ::validated_data::RequiredValue<item::Endpoint<'a, Mode>, Mode>,
                }

                pub mod item {

                    #[::validated_data::data_view]
                    pub struct Endpoint<'a, Mode> {
                        pub ip_address: ::validated_data::RequiredValue<&'a str, Mode>,
                        pub datacenter: ::validated_data::RequiredValue<&'a str, Mode>,
                        pub city: ::validated_data::RequiredValue<&'a str, Mode>,
                        pub country: ::validated_data::RequiredValue<&'a str, Mode>,
                        pub region: ::validated_data::RequiredValue<&'a str, Mode>,
                        pub latitude: ::validated_data::RequiredValue<&'a str, Mode>,
                        pub longitude: ::validated_data::RequiredValue<&'a str, Mode>,
                    }
                }
            }
        }
    }

    #[::validated_data::data_view]
    pub struct Applications<'a, Mode> {
        pub profiles: ::validated_data::Field<applications::Profiles<'a, Mode>>,
        pub categories: ::validated_data::Field<applications::Categories<'a, Mode>>,
    }

    pub mod applications {

        #[::validated_data::data_view(list)]
        pub struct Profiles<'a, Mode> (::validated_data::Field<profiles::Item<'a, Mode>>);

        pub mod profiles {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::validated_data::Field<&'a str>,
                pub builtin_applications: ::validated_data::Field<item::BuiltinApplications<'a, Mode>>,
                pub user_defined_applications: ::validated_data::Field<item::UserDefinedApplications<'a, Mode>>,
                pub categories: ::validated_data::Field<item::Categories<'a, Mode>>,
                pub transport_protocols: ::validated_data::Field<item::TransportProtocols<'a, Mode>>,
            }

            pub mod item {

                #[::validated_data::data_view(list)]
                pub struct BuiltinApplications<'a, Mode> (::validated_data::Field<builtin_applications::Item<'a, Mode>>);

                pub mod builtin_applications {

                    #[::validated_data::data_view]
                    pub struct Item<'a, Mode> {
                        pub name: ::validated_data::Field<&'a str>,
                        pub services: ::validated_data::Field<item::Services<'a, Mode>>,
                    }

                    pub mod item {

                        #[::validated_data::data_view(list)]
                        pub struct Services<'a, Mode> (::validated_data::Field<&'a str>);
                    }
                }

                #[::validated_data::data_view(list)]
                pub struct UserDefinedApplications<'a, Mode> (::validated_data::Field<user_defined_applications::Item<'a, Mode>>);

                pub mod user_defined_applications {

                    #[::validated_data::data_view]
                    pub struct Item<'a, Mode> {
                        pub name: ::validated_data::Field<&'a str>,
                    }
                }

                #[::validated_data::data_view(list)]
                pub struct Categories<'a, Mode> (::validated_data::Field<categories::Item<'a, Mode>>);

                pub mod categories {

                    #[::validated_data::data_view]
                    pub struct Item<'a, Mode> {
                        pub category: ::validated_data::Field<&'a str>,
                        pub services: ::validated_data::Field<item::Services<'a, Mode>>,
                    }

                    pub mod item {

                        #[::validated_data::data_view(list)]
                        pub struct Services<'a, Mode> (::validated_data::Field<&'a str>);
                    }
                }

                #[::validated_data::data_view(list)]
                pub struct TransportProtocols<'a, Mode> (::validated_data::Field<&'a str>);
            }
        }

        #[::validated_data::data_view]
        pub struct Categories<'a, Mode> {
            pub builtin_applications: ::validated_data::Field<categories::BuiltinApplications<'a, Mode>>,
            pub user_defined_applications: ::validated_data::Field<categories::UserDefinedApplications<'a, Mode>>,
        }

        pub mod categories {

            #[::validated_data::data_view(list)]
            pub struct BuiltinApplications<'a, Mode> (::validated_data::Field<builtin_applications::Item<'a, Mode>>);

            pub mod builtin_applications {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub name: ::validated_data::Field<&'a str>,
                    pub category: ::validated_data::Field<&'a str>,
                    pub services: ::validated_data::Field<item::Services<'a, Mode>>,
                }

                pub mod item {

                    #[::validated_data::data_view(list)]
                    pub struct Services<'a, Mode> (::validated_data::Field<&'a str>);
                }
            }

            #[::validated_data::data_view(list)]
            pub struct UserDefinedApplications<'a, Mode> (::validated_data::Field<user_defined_applications::Item<'a, Mode>>);

            pub mod user_defined_applications {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub name: ::validated_data::Field<&'a str>,
                    pub category: ::validated_data::Field<&'a str>,
                }
            }
        }
    }
}

#[::validated_data::data_view]
pub struct DigitalTwin<'a, Mode> {
    pub environment: ::validated_data::Field<&'a str>,
    pub node_type: ::validated_data::Field<&'a str>,
    pub ip_addr: ::validated_data::Field<&'a str>,
    pub version: ::validated_data::Field<&'a str>,
    pub username: ::validated_data::Field<&'a str>,
    pub password: ::validated_data::Field<&'a str>,
    pub internet_access: ::validated_data::Field<bool>,
}

#[::validated_data::data_view]
pub struct Interfaces<'a, Mode> {
    pub errdisable: ::validated_data::Field<interfaces::Errdisable<'a, Mode>>,
}

pub mod interfaces {

    #[::validated_data::data_view]
    pub struct Errdisable<'a, Mode> {
        pub only_avd_interfaces: ::validated_data::Field<bool>,
    }
}

#[::validated_data::data_view]
pub struct Bgp<'a, Mode> {
    pub check_tcp_queues: ::validated_data::Field<bool>,
    pub minimum_established_time: ::validated_data::Field<i64>,
}
