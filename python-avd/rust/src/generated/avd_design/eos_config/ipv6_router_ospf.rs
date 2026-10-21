// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct Ipv6RouterOspf<'a, Mode> {
    pub process_ids: ::data_views::Field<ProcessIds<'a, Mode>>,
}

#[::data_views::data_view(indexed_list, primary_key(id))]
pub struct ProcessIds<'a, Mode> (::data_views::Field<process_ids::Item<'a, Mode>>);

pub mod process_ids {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub id: ::data_views::Field<i64>,
        pub vrf: ::data_views::Field<&'a str>,
        pub router_id: ::data_views::Field<&'a str>,
        pub redistribute: ::data_views::Field<item::Redistribute<'a, Mode>>,
        pub auto_cost_reference_bandwidth: ::data_views::Field<i64>,
    }

    pub mod item {

        #[::data_views::data_view]
        pub struct Redistribute<'a, Mode> {
            pub bgp: ::data_views::Field<redistribute::Bgp<'a, Mode>>,
            pub connected: ::data_views::Field<redistribute::Connected<'a, Mode>>,
            pub isis: ::data_views::Field<redistribute::Isis<'a, Mode>>,
            pub ospfv3: ::data_views::Field<redistribute::Ospfv3<'a, Mode>>,
            #[data_view(rename = "static")]
            pub field_static: ::data_views::Field<redistribute::FieldStatic<'a, Mode>>,
            pub dhcp: ::data_views::Field<redistribute::Dhcp<'a, Mode>>,
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
            pub struct Isis<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub isis_level: ::data_views::Field<&'a str>,
                pub route_map: ::data_views::Field<&'a str>,
                pub include_leaked: ::data_views::Field<bool>,
            }

            #[::data_views::data_view]
            pub struct Ospfv3<'a, Mode> {
                pub enabled: ::data_views::Field<bool>,
                pub match_external: ::data_views::Field<ospfv3::MatchExternal<'a, Mode>>,
                pub match_internal: ::data_views::Field<ospfv3::MatchInternal<'a, Mode>>,
                pub match_nssa_external: ::data_views::Field<ospfv3::MatchNssaExternal<'a, Mode>>,
                pub route_map: ::data_views::Field<&'a str>,
            }

            pub mod ospfv3 {

                #[::data_views::data_view]
                pub struct MatchExternal<'a, Mode> {
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    pub route_map: ::data_views::Field<&'a str>,
                    pub include_leaked: ::data_views::Field<bool>,
                }

                #[::data_views::data_view]
                pub struct MatchInternal<'a, Mode> {
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    pub route_map: ::data_views::Field<&'a str>,
                }

                #[::data_views::data_view]
                pub struct MatchNssaExternal<'a, Mode> {
                    pub enabled: ::data_views::RequiredValue<bool, Mode>,
                    pub route_map: ::data_views::Field<&'a str>,
                }
            }

            #[::data_views::data_view]
            pub struct FieldStatic<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub route_map: ::data_views::Field<&'a str>,
                pub include_leaked: ::data_views::Field<bool>,
            }

            #[::data_views::data_view]
            pub struct Dhcp<'a, Mode> {
                pub enabled: ::data_views::RequiredValue<bool, Mode>,
                pub route_map: ::data_views::Field<&'a str>,
            }
        }
    }
}
