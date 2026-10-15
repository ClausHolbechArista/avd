// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(indexed_list, primary_key(id))]
pub struct ProcessIds<'a, Mode> (::validated_data::Field<process_ids::Item<'a, Mode>>);

pub mod process_ids {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub id: ::validated_data::Field<i64>,
        pub vrf: ::validated_data::Field<&'a str>,
        pub router_id: ::validated_data::Field<&'a str>,
        pub redistribute: ::validated_data::Field<item::Redistribute<'a, Mode>>,
        pub auto_cost_reference_bandwidth: ::validated_data::Field<i64>,
    }

    pub mod item {

        #[::validated_data::data_view]
        pub struct Redistribute<'a, Mode> {
            pub bgp: ::validated_data::Field<redistribute::Bgp<'a, Mode>>,
            pub connected: ::validated_data::Field<redistribute::Connected<'a, Mode>>,
            pub isis: ::validated_data::Field<redistribute::Isis<'a, Mode>>,
            pub ospfv3: ::validated_data::Field<redistribute::Ospfv3<'a, Mode>>,
            #[data_view(rename = "static")]
            pub field_static: ::validated_data::Field<redistribute::FieldStatic<'a, Mode>>,
            pub dhcp: ::validated_data::Field<redistribute::Dhcp<'a, Mode>>,
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
            pub struct Isis<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub isis_level: ::validated_data::Field<&'a str>,
                pub route_map: ::validated_data::Field<&'a str>,
                pub include_leaked: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view]
            pub struct Ospfv3<'a, Mode> {
                pub enabled: ::validated_data::Field<bool>,
                pub match_external: ::validated_data::Field<ospfv3::MatchExternal<'a, Mode>>,
                pub match_internal: ::validated_data::Field<ospfv3::MatchInternal<'a, Mode>>,
                pub match_nssa_external: ::validated_data::Field<ospfv3::MatchNssaExternal<'a, Mode>>,
                pub route_map: ::validated_data::Field<&'a str>,
            }

            pub mod ospfv3 {

                #[::validated_data::data_view]
                pub struct MatchExternal<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub route_map: ::validated_data::Field<&'a str>,
                    pub include_leaked: ::validated_data::Field<bool>,
                }

                #[::validated_data::data_view]
                pub struct MatchInternal<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub route_map: ::validated_data::Field<&'a str>,
                }

                #[::validated_data::data_view]
                pub struct MatchNssaExternal<'a, Mode> {
                    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                    pub route_map: ::validated_data::Field<&'a str>,
                }
            }

            #[::validated_data::data_view]
            pub struct FieldStatic<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub route_map: ::validated_data::Field<&'a str>,
                pub include_leaked: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view]
            pub struct Dhcp<'a, Mode> {
                pub enabled: ::validated_data::RequiredValue<bool, Mode>,
                pub route_map: ::validated_data::Field<&'a str>,
            }
        }
    }
}
