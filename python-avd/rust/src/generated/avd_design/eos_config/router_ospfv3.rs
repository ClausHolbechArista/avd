// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct RouterOspfv3<'a, Mode> {
    pub router_id: ::data_views::Field<&'a str>,
    pub passive_interface_default: ::data_views::Field<bool>,
    pub auto_cost_reference_bandwidth: ::data_views::Field<i64>,
    pub address_family_ipv4: ::data_views::Field<AddressFamilyIpv4<'a, Mode>>,
    pub address_family_ipv6: ::data_views::Field<AddressFamilyIpv6<'a, Mode>>,
    pub vrfs: ::data_views::Field<Vrfs<'a, Mode>>,
    pub eos_cli: ::data_views::Field<&'a str>,
}

#[::data_views::data_view]
pub struct AddressFamilyIpv4<'a, Mode> {
    pub enabled: ::data_views::RequiredValue<bool, Mode>,
    pub router_id: ::data_views::Field<&'a str>,
    pub passive_interface_default: ::data_views::Field<bool>,
    pub auto_cost_reference_bandwidth: ::data_views::Field<i64>,
    pub redistribute: ::data_views::Field<address_family_ipv4::Redistribute<'a, Mode>>,
}

pub mod address_family_ipv4 {

    #[::data_views::data_view]
    pub struct Redistribute<'a, Mode> {
        pub bgp: ::data_views::Field<redistribute::Bgp<'a, Mode>>,
        pub connected: ::data_views::Field<redistribute::Connected<'a, Mode>>,
        #[data_view(rename = "static")]
        pub field_static: ::data_views::Field<redistribute::FieldStatic<'a, Mode>>,
        pub isis: ::data_views::Field<redistribute::Isis<'a, Mode>>,
        pub ospfv3: ::data_views::Field<redistribute::Ospfv3<'a, Mode>>,
    }

    pub mod redistribute {

        #[::data_views::data_view]
        pub struct Bgp<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub route_map: ::data_views::Field<&'a str>,
            pub include_leaked: ::data_views::Field<bool>,
        }

        #[::data_views::data_view]
        pub struct Connected<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub route_map: ::data_views::Field<&'a str>,
            pub include_leaked: ::data_views::Field<bool>,
        }

        #[::data_views::data_view]
        pub struct FieldStatic<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub route_map: ::data_views::Field<&'a str>,
            pub include_leaked: ::data_views::Field<bool>,
        }

        #[::data_views::data_view]
        pub struct Isis<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub level: ::data_views::Field<&'a str>,
            pub route_map: ::data_views::Field<&'a str>,
            pub include_leaked: ::data_views::Field<bool>,
        }

        #[::data_views::data_view]
        pub struct Ospfv3<'a, Mode> {
            pub leaked: ::data_views::Field<ospfv3::Leaked<'a, Mode>>,
            pub leaked_match_external: ::data_views::Field<ospfv3::LeakedMatchExternal<'a, Mode>>,
            pub leaked_match_nssa_external: ::data_views::Field<ospfv3::LeakedMatchNssaExternal<'a, Mode>>,
        }

        pub mod ospfv3 {

            #[::data_views::data_view]
            pub struct Leaked<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub match_internal: ::data_views::Field<bool>,
                pub route_map: ::data_views::Field<&'a str>,
            }

            #[::data_views::data_view]
            pub struct LeakedMatchExternal<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub route_map: ::data_views::Field<&'a str>,
            }

            #[::data_views::data_view]
            pub struct LeakedMatchNssaExternal<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub route_map: ::data_views::Field<&'a str>,
            }
        }
    }
}

#[::data_views::data_view]
pub struct AddressFamilyIpv6<'a, Mode> {
    pub redistribute: ::data_views::Field<address_family_ipv6::Redistribute<'a, Mode>>,
    pub enabled: ::data_views::RequiredValue<bool, Mode>,
    pub router_id: ::data_views::Field<&'a str>,
    pub passive_interface_default: ::data_views::Field<bool>,
    pub auto_cost_reference_bandwidth: ::data_views::Field<i64>,
}

pub mod address_family_ipv6 {

    #[::data_views::data_view]
    pub struct Redistribute<'a, Mode> {
        pub dhcp: ::data_views::Field<redistribute::Dhcp<'a, Mode>>,
        pub bgp: ::data_views::Field<redistribute::Bgp<'a, Mode>>,
        pub connected: ::data_views::Field<redistribute::Connected<'a, Mode>>,
        #[data_view(rename = "static")]
        pub field_static: ::data_views::Field<redistribute::FieldStatic<'a, Mode>>,
        pub isis: ::data_views::Field<redistribute::Isis<'a, Mode>>,
        pub ospfv3: ::data_views::Field<redistribute::Ospfv3<'a, Mode>>,
    }

    pub mod redistribute {

        #[::data_views::data_view]
        pub struct Dhcp<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub route_map: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view]
        pub struct Bgp<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub route_map: ::data_views::Field<&'a str>,
            pub include_leaked: ::data_views::Field<bool>,
        }

        #[::data_views::data_view]
        pub struct Connected<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub route_map: ::data_views::Field<&'a str>,
            pub include_leaked: ::data_views::Field<bool>,
        }

        #[::data_views::data_view]
        pub struct FieldStatic<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub route_map: ::data_views::Field<&'a str>,
            pub include_leaked: ::data_views::Field<bool>,
        }

        #[::data_views::data_view]
        pub struct Isis<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub level: ::data_views::Field<&'a str>,
            pub route_map: ::data_views::Field<&'a str>,
            pub include_leaked: ::data_views::Field<bool>,
        }

        #[::data_views::data_view]
        pub struct Ospfv3<'a, Mode> {
            pub leaked: ::data_views::Field<ospfv3::Leaked<'a, Mode>>,
            pub leaked_match_external: ::data_views::Field<ospfv3::LeakedMatchExternal<'a, Mode>>,
            pub leaked_match_nssa_external: ::data_views::Field<ospfv3::LeakedMatchNssaExternal<'a, Mode>>,
        }

        pub mod ospfv3 {

            #[::data_views::data_view]
            pub struct Leaked<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub match_internal: ::data_views::Field<bool>,
                pub route_map: ::data_views::Field<&'a str>,
            }

            #[::data_views::data_view]
            pub struct LeakedMatchExternal<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub route_map: ::data_views::Field<&'a str>,
            }

            #[::data_views::data_view]
            pub struct LeakedMatchNssaExternal<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub route_map: ::data_views::Field<&'a str>,
            }
        }
    }
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Vrfs<'a, Mode> (::data_views::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub router_id: ::data_views::Field<&'a str>,
        pub passive_interface_default: ::data_views::Field<bool>,
        pub auto_cost_reference_bandwidth: ::data_views::Field<i64>,
        pub address_family_ipv4: ::data_views::Field<item::AddressFamilyIpv4<'a, Mode>>,
        pub address_family_ipv6: ::data_views::Field<item::AddressFamilyIpv6<'a, Mode>>,
        pub eos_cli: ::data_views::Field<&'a str>,
    }

    pub mod item {

        #[::data_views::data_view]
        pub struct AddressFamilyIpv4<'a, Mode> {
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub router_id: ::data_views::Field<&'a str>,
            pub passive_interface_default: ::data_views::Field<bool>,
            pub auto_cost_reference_bandwidth: ::data_views::Field<i64>,
            pub redistribute: ::data_views::Field<address_family_ipv4::Redistribute<'a, Mode>>,
        }

        pub mod address_family_ipv4 {

            #[::data_views::data_view]
            pub struct Redistribute<'a, Mode> {
                pub bgp: ::data_views::Field<redistribute::Bgp<'a, Mode>>,
                pub connected: ::data_views::Field<redistribute::Connected<'a, Mode>>,
                #[data_view(rename = "static")]
                pub field_static: ::data_views::Field<redistribute::FieldStatic<'a, Mode>>,
                pub isis: ::data_views::Field<redistribute::Isis<'a, Mode>>,
                pub ospfv3: ::data_views::Field<redistribute::Ospfv3<'a, Mode>>,
            }

            pub mod redistribute {

                #[::data_views::data_view]
                pub struct Bgp<'a, Mode> {
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    pub route_map: ::data_views::Field<&'a str>,
                    pub include_leaked: ::data_views::Field<bool>,
                }

                #[::data_views::data_view]
                pub struct Connected<'a, Mode> {
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    pub route_map: ::data_views::Field<&'a str>,
                    pub include_leaked: ::data_views::Field<bool>,
                }

                #[::data_views::data_view]
                pub struct FieldStatic<'a, Mode> {
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    pub route_map: ::data_views::Field<&'a str>,
                    pub include_leaked: ::data_views::Field<bool>,
                }

                #[::data_views::data_view]
                pub struct Isis<'a, Mode> {
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    pub level: ::data_views::Field<&'a str>,
                    pub route_map: ::data_views::Field<&'a str>,
                    pub include_leaked: ::data_views::Field<bool>,
                }

                #[::data_views::data_view]
                pub struct Ospfv3<'a, Mode> {
                    pub leaked: ::data_views::Field<ospfv3::Leaked<'a, Mode>>,
                    pub leaked_match_external: ::data_views::Field<ospfv3::LeakedMatchExternal<'a, Mode>>,
                    pub leaked_match_nssa_external: ::data_views::Field<ospfv3::LeakedMatchNssaExternal<'a, Mode>>,
                }

                pub mod ospfv3 {

                    #[::data_views::data_view]
                    pub struct Leaked<'a, Mode> {
                        pub enabled: ::data_views::RequiredValue<bool, Mode>,
                        pub match_internal: ::data_views::Field<bool>,
                        pub route_map: ::data_views::Field<&'a str>,
                    }

                    #[::data_views::data_view]
                    pub struct LeakedMatchExternal<'a, Mode> {
                        pub enabled: ::data_views::RequiredValue<bool, Mode>,
                        pub route_map: ::data_views::Field<&'a str>,
                    }

                    #[::data_views::data_view]
                    pub struct LeakedMatchNssaExternal<'a, Mode> {
                        pub enabled: ::data_views::RequiredValue<bool, Mode>,
                        pub route_map: ::data_views::Field<&'a str>,
                    }
                }
            }
        }

        #[::data_views::data_view]
        pub struct AddressFamilyIpv6<'a, Mode> {
            pub redistribute: ::data_views::Field<address_family_ipv6::Redistribute<'a, Mode>>,
            pub enabled: ::data_views::RequiredValue<bool, Mode>,
            pub router_id: ::data_views::Field<&'a str>,
            pub passive_interface_default: ::data_views::Field<bool>,
            pub auto_cost_reference_bandwidth: ::data_views::Field<i64>,
        }

        pub mod address_family_ipv6 {

            #[::data_views::data_view]
            pub struct Redistribute<'a, Mode> {
                pub dhcp: ::data_views::Field<redistribute::Dhcp<'a, Mode>>,
                pub bgp: ::data_views::Field<redistribute::Bgp<'a, Mode>>,
                pub connected: ::data_views::Field<redistribute::Connected<'a, Mode>>,
                #[data_view(rename = "static")]
                pub field_static: ::data_views::Field<redistribute::FieldStatic<'a, Mode>>,
                pub isis: ::data_views::Field<redistribute::Isis<'a, Mode>>,
                pub ospfv3: ::data_views::Field<redistribute::Ospfv3<'a, Mode>>,
            }

            pub mod redistribute {

                #[::data_views::data_view]
                pub struct Dhcp<'a, Mode> {
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    pub route_map: ::data_views::Field<&'a str>,
                }

                #[::data_views::data_view]
                pub struct Bgp<'a, Mode> {
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    pub route_map: ::data_views::Field<&'a str>,
                    pub include_leaked: ::data_views::Field<bool>,
                }

                #[::data_views::data_view]
                pub struct Connected<'a, Mode> {
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    pub route_map: ::data_views::Field<&'a str>,
                    pub include_leaked: ::data_views::Field<bool>,
                }

                #[::data_views::data_view]
                pub struct FieldStatic<'a, Mode> {
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    pub route_map: ::data_views::Field<&'a str>,
                    pub include_leaked: ::data_views::Field<bool>,
                }

                #[::data_views::data_view]
                pub struct Isis<'a, Mode> {
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    pub level: ::data_views::Field<&'a str>,
                    pub route_map: ::data_views::Field<&'a str>,
                    pub include_leaked: ::data_views::Field<bool>,
                }

                #[::data_views::data_view]
                pub struct Ospfv3<'a, Mode> {
                    pub leaked: ::data_views::Field<ospfv3::Leaked<'a, Mode>>,
                    pub leaked_match_external: ::data_views::Field<ospfv3::LeakedMatchExternal<'a, Mode>>,
                    pub leaked_match_nssa_external: ::data_views::Field<ospfv3::LeakedMatchNssaExternal<'a, Mode>>,
                }

                pub mod ospfv3 {

                    #[::data_views::data_view]
                    pub struct Leaked<'a, Mode> {
                        pub enabled: ::data_views::RequiredValue<bool, Mode>,
                        pub match_internal: ::data_views::Field<bool>,
                        pub route_map: ::data_views::Field<&'a str>,
                    }

                    #[::data_views::data_view]
                    pub struct LeakedMatchExternal<'a, Mode> {
                        pub enabled: ::data_views::RequiredValue<bool, Mode>,
                        pub route_map: ::data_views::Field<&'a str>,
                    }

                    #[::data_views::data_view]
                    pub struct LeakedMatchNssaExternal<'a, Mode> {
                        pub enabled: ::data_views::RequiredValue<bool, Mode>,
                        pub route_map: ::data_views::Field<&'a str>,
                    }
                }
            }
        }
    }
}
