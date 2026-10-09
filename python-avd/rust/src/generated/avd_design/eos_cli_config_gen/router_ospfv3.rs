// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct AddressFamilyIpv4<'a, Mode> {
    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
    pub router_id: ::validated_data::Field<&'a str>,
    pub passive_interface_default: ::validated_data::Field<bool>,
    pub auto_cost_reference_bandwidth: ::validated_data::Field<i64>,
    pub redistribute: ::validated_data::Field<address_family_ipv4::Redistribute<'a, Mode>>,
}

pub mod address_family_ipv4 {

    #[::validated_data::data_view]
    pub struct Redistribute<'a, Mode> {
        pub bgp: ::validated_data::Field<redistribute::Bgp<'a, Mode>>,
        pub connected: ::validated_data::Field<redistribute::Connected<'a, Mode>>,
        #[data_view(rename = "static")]
        pub field_static: ::validated_data::Field<redistribute::FieldStatic<'a, Mode>>,
        pub isis: ::validated_data::Field<redistribute::Isis<'a, Mode>>,
        pub ospfv3: ::validated_data::Field<redistribute::Ospfv3<'a, Mode>>,
    }

    pub mod redistribute {

        #[::validated_data::data_view]
        pub struct Bgp<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub route_map: ::validated_data::Field<&'a str>,
            pub include_leaked: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct Connected<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub route_map: ::validated_data::Field<&'a str>,
            pub include_leaked: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct FieldStatic<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub route_map: ::validated_data::Field<&'a str>,
            pub include_leaked: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct Isis<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub level: ::validated_data::Field<&'a str>,
            pub route_map: ::validated_data::Field<&'a str>,
            pub include_leaked: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct Ospfv3<'a, Mode> {
            pub leaked: ::validated_data::Field<ospfv3::Leaked<'a, Mode>>,
            pub leaked_match_external: ::validated_data::Field<ospfv3::LeakedMatchExternal<'a, Mode>>,
            pub leaked_match_nssa_external: ::validated_data::Field<ospfv3::LeakedMatchNssaExternal<'a, Mode>>,
        }

        pub mod ospfv3 {

            #[::validated_data::data_view]
            pub struct Leaked<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub match_internal: ::validated_data::Field<bool>,
                pub route_map: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view]
            pub struct LeakedMatchExternal<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub route_map: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view]
            pub struct LeakedMatchNssaExternal<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub route_map: ::validated_data::Field<&'a str>,
            }
        }
    }
}

#[::validated_data::data_view]
pub struct AddressFamilyIpv6<'a, Mode> {
    pub redistribute: ::validated_data::Field<address_family_ipv6::Redistribute<'a, Mode>>,
    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
    pub router_id: ::validated_data::Field<&'a str>,
    pub passive_interface_default: ::validated_data::Field<bool>,
    pub auto_cost_reference_bandwidth: ::validated_data::Field<i64>,
}

pub mod address_family_ipv6 {

    #[::validated_data::data_view]
    pub struct Redistribute<'a, Mode> {
        pub dhcp: ::validated_data::Field<redistribute::Dhcp<'a, Mode>>,
        pub bgp: ::validated_data::Field<redistribute::Bgp<'a, Mode>>,
        pub connected: ::validated_data::Field<redistribute::Connected<'a, Mode>>,
        #[data_view(rename = "static")]
        pub field_static: ::validated_data::Field<redistribute::FieldStatic<'a, Mode>>,
        pub isis: ::validated_data::Field<redistribute::Isis<'a, Mode>>,
        pub ospfv3: ::validated_data::Field<redistribute::Ospfv3<'a, Mode>>,
    }

    pub mod redistribute {

        #[::validated_data::data_view]
        pub struct Dhcp<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub route_map: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view]
        pub struct Bgp<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub route_map: ::validated_data::Field<&'a str>,
            pub include_leaked: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct Connected<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub route_map: ::validated_data::Field<&'a str>,
            pub include_leaked: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct FieldStatic<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub route_map: ::validated_data::Field<&'a str>,
            pub include_leaked: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct Isis<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub level: ::validated_data::Field<&'a str>,
            pub route_map: ::validated_data::Field<&'a str>,
            pub include_leaked: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct Ospfv3<'a, Mode> {
            pub leaked: ::validated_data::Field<ospfv3::Leaked<'a, Mode>>,
            pub leaked_match_external: ::validated_data::Field<ospfv3::LeakedMatchExternal<'a, Mode>>,
            pub leaked_match_nssa_external: ::validated_data::Field<ospfv3::LeakedMatchNssaExternal<'a, Mode>>,
        }

        pub mod ospfv3 {

            #[::validated_data::data_view]
            pub struct Leaked<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub match_internal: ::validated_data::Field<bool>,
                pub route_map: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view]
            pub struct LeakedMatchExternal<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub route_map: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view]
            pub struct LeakedMatchNssaExternal<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub route_map: ::validated_data::Field<&'a str>,
            }
        }
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Vrfs<'a, Mode> (::validated_data::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub router_id: ::validated_data::Field<&'a str>,
        pub passive_interface_default: ::validated_data::Field<bool>,
        pub auto_cost_reference_bandwidth: ::validated_data::Field<i64>,
        pub address_family_ipv4: ::validated_data::Field<item::AddressFamilyIpv4<'a, Mode>>,
        pub address_family_ipv6: ::validated_data::Field<item::AddressFamilyIpv6<'a, Mode>>,
        pub eos_cli: ::validated_data::Field<&'a str>,
    }

    pub mod item {

        #[::validated_data::data_view]
        pub struct AddressFamilyIpv4<'a, Mode> {
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub router_id: ::validated_data::Field<&'a str>,
            pub passive_interface_default: ::validated_data::Field<bool>,
            pub auto_cost_reference_bandwidth: ::validated_data::Field<i64>,
            pub redistribute: ::validated_data::Field<address_family_ipv4::Redistribute<'a, Mode>>,
        }

        pub mod address_family_ipv4 {

            #[::validated_data::data_view]
            pub struct Redistribute<'a, Mode> {
                pub bgp: ::validated_data::Field<redistribute::Bgp<'a, Mode>>,
                pub connected: ::validated_data::Field<redistribute::Connected<'a, Mode>>,
                #[data_view(rename = "static")]
                pub field_static: ::validated_data::Field<redistribute::FieldStatic<'a, Mode>>,
                pub isis: ::validated_data::Field<redistribute::Isis<'a, Mode>>,
                pub ospfv3: ::validated_data::Field<redistribute::Ospfv3<'a, Mode>>,
            }

            pub mod redistribute {

                #[::validated_data::data_view]
                pub struct Bgp<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub route_map: ::validated_data::Field<&'a str>,
                    pub include_leaked: ::validated_data::Field<bool>,
                }

                #[::validated_data::data_view]
                pub struct Connected<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub route_map: ::validated_data::Field<&'a str>,
                    pub include_leaked: ::validated_data::Field<bool>,
                }

                #[::validated_data::data_view]
                pub struct FieldStatic<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub route_map: ::validated_data::Field<&'a str>,
                    pub include_leaked: ::validated_data::Field<bool>,
                }

                #[::validated_data::data_view]
                pub struct Isis<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub level: ::validated_data::Field<&'a str>,
                    pub route_map: ::validated_data::Field<&'a str>,
                    pub include_leaked: ::validated_data::Field<bool>,
                }

                #[::validated_data::data_view]
                pub struct Ospfv3<'a, Mode> {
                    pub leaked: ::validated_data::Field<ospfv3::Leaked<'a, Mode>>,
                    pub leaked_match_external: ::validated_data::Field<ospfv3::LeakedMatchExternal<'a, Mode>>,
                    pub leaked_match_nssa_external: ::validated_data::Field<ospfv3::LeakedMatchNssaExternal<'a, Mode>>,
                }

                pub mod ospfv3 {

                    #[::validated_data::data_view]
                    pub struct Leaked<'a, Mode> {
                        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                        pub match_internal: ::validated_data::Field<bool>,
                        pub route_map: ::validated_data::Field<&'a str>,
                    }

                    #[::validated_data::data_view]
                    pub struct LeakedMatchExternal<'a, Mode> {
                        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                        pub route_map: ::validated_data::Field<&'a str>,
                    }

                    #[::validated_data::data_view]
                    pub struct LeakedMatchNssaExternal<'a, Mode> {
                        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                        pub route_map: ::validated_data::Field<&'a str>,
                    }
                }
            }
        }

        #[::validated_data::data_view]
        pub struct AddressFamilyIpv6<'a, Mode> {
            pub redistribute: ::validated_data::Field<address_family_ipv6::Redistribute<'a, Mode>>,
            pub enabled: ::validated_data::RequiredValue<bool, Mode>,
            pub router_id: ::validated_data::Field<&'a str>,
            pub passive_interface_default: ::validated_data::Field<bool>,
            pub auto_cost_reference_bandwidth: ::validated_data::Field<i64>,
        }

        pub mod address_family_ipv6 {

            #[::validated_data::data_view]
            pub struct Redistribute<'a, Mode> {
                pub dhcp: ::validated_data::Field<redistribute::Dhcp<'a, Mode>>,
                pub bgp: ::validated_data::Field<redistribute::Bgp<'a, Mode>>,
                pub connected: ::validated_data::Field<redistribute::Connected<'a, Mode>>,
                #[data_view(rename = "static")]
                pub field_static: ::validated_data::Field<redistribute::FieldStatic<'a, Mode>>,
                pub isis: ::validated_data::Field<redistribute::Isis<'a, Mode>>,
                pub ospfv3: ::validated_data::Field<redistribute::Ospfv3<'a, Mode>>,
            }

            pub mod redistribute {

                #[::validated_data::data_view]
                pub struct Dhcp<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub route_map: ::validated_data::Field<&'a str>,
                }

                #[::validated_data::data_view]
                pub struct Bgp<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub route_map: ::validated_data::Field<&'a str>,
                    pub include_leaked: ::validated_data::Field<bool>,
                }

                #[::validated_data::data_view]
                pub struct Connected<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub route_map: ::validated_data::Field<&'a str>,
                    pub include_leaked: ::validated_data::Field<bool>,
                }

                #[::validated_data::data_view]
                pub struct FieldStatic<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub route_map: ::validated_data::Field<&'a str>,
                    pub include_leaked: ::validated_data::Field<bool>,
                }

                #[::validated_data::data_view]
                pub struct Isis<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub level: ::validated_data::Field<&'a str>,
                    pub route_map: ::validated_data::Field<&'a str>,
                    pub include_leaked: ::validated_data::Field<bool>,
                }

                #[::validated_data::data_view]
                pub struct Ospfv3<'a, Mode> {
                    pub leaked: ::validated_data::Field<ospfv3::Leaked<'a, Mode>>,
                    pub leaked_match_external: ::validated_data::Field<ospfv3::LeakedMatchExternal<'a, Mode>>,
                    pub leaked_match_nssa_external: ::validated_data::Field<ospfv3::LeakedMatchNssaExternal<'a, Mode>>,
                }

                pub mod ospfv3 {

                    #[::validated_data::data_view]
                    pub struct Leaked<'a, Mode> {
                        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                        pub match_internal: ::validated_data::Field<bool>,
                        pub route_map: ::validated_data::Field<&'a str>,
                    }

                    #[::validated_data::data_view]
                    pub struct LeakedMatchExternal<'a, Mode> {
                        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                        pub route_map: ::validated_data::Field<&'a str>,
                    }

                    #[::validated_data::data_view]
                    pub struct LeakedMatchNssaExternal<'a, Mode> {
                        pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                        pub route_map: ::validated_data::Field<&'a str>,
                    }
                }
            }
        }
    }
}
