// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct RouterTrafficEngineering<'a, Mode> {
    pub enabled: ::data_views::RequiredValue<bool, Mode>,
    pub router_id: ::data_views::Field<RouterId<'a, Mode>>,
    pub segment_routing: ::data_views::Field<SegmentRouting<'a, Mode>>,
    pub twamp_light_sender_profile: ::data_views::Field<&'a str>,
    pub flex_algos: ::data_views::Field<FlexAlgos<'a, Mode>>,
}

#[::data_views::data_view]
pub struct RouterId<'a, Mode> {
    pub ipv4: ::data_views::Field<&'a str>,
    pub ipv6: ::data_views::Field<&'a str>,
}

#[::data_views::data_view]
pub struct SegmentRouting<'a, Mode> {
    pub colored_tunnel_rib: ::data_views::Field<bool>,
    pub policy_endpoints: ::data_views::Field<segment_routing::PolicyEndpoints<'a, Mode>>,
}

pub mod segment_routing {

    #[::data_views::data_view(list)]
    pub struct PolicyEndpoints<'a, Mode> (::data_views::Field<policy_endpoints::Item<'a, Mode>>);

    pub mod policy_endpoints {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub address: ::data_views::Field<&'a str>,
            pub colors: ::data_views::Field<item::Colors<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view(indexed_list, primary_key(value))]
            pub struct Colors<'a, Mode> (::data_views::Field<colors::Item<'a, Mode>>);

            pub mod colors {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub value: ::data_views::Field<i64>,
                    pub binding_sid: ::data_views::Field<i64>,
                    pub description: ::data_views::Field<&'a str>,
                    pub name: ::data_views::Field<&'a str>,
                    pub sbfd_remote_discriminator: ::data_views::Field<&'a str>,
                    pub path_group: ::data_views::Field<item::PathGroup<'a, Mode>>,
                }

                pub mod item {

                    #[::data_views::data_view(list)]
                    pub struct PathGroup<'a, Mode> (::data_views::Field<path_group::Item<'a, Mode>>);

                    pub mod path_group {

                        #[::data_views::data_view]
                        pub struct Item<'a, Mode> {
                            pub preference: ::data_views::Field<i64>,
                            pub explicit_null: ::data_views::Field<&'a str>,
                            pub segment_list: ::data_views::Field<item::SegmentList<'a, Mode>>,
                        }

                        pub mod item {

                            #[::data_views::data_view(list)]
                            pub struct SegmentList<'a, Mode> (::data_views::Field<segment_list::Item<'a, Mode>>);

                            pub mod segment_list {

                                #[::data_views::data_view]
                                pub struct Item<'a, Mode> {
                                    pub label_stack: ::data_views::Field<&'a str>,
                                    pub weight: ::data_views::Field<i64>,
                                    pub index: ::data_views::Field<i64>,
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[::data_views::data_view(indexed_list, primary_key(number))]
pub struct FlexAlgos<'a, Mode> (::data_views::Field<flex_algos::Item<'a, Mode>>);

pub mod flex_algos {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub number: ::data_views::Field<i64>,
        pub name: ::data_views::RequiredValue<&'a str, Mode>,
        pub administrative_group: ::data_views::Field<item::AdministrativeGroup<'a, Mode>>,
        pub metric: ::data_views::Field<&'a str>,
        pub priority: ::data_views::Field<i64>,
        pub color: ::data_views::Field<i64>,
        pub srlg_exclude: ::data_views::Field<&'a str>,
    }

    pub mod item {

        #[::data_views::data_view]
        pub struct AdministrativeGroup<'a, Mode> {
            pub include_all: ::data_views::Field<&'a str>,
            pub include_any: ::data_views::Field<&'a str>,
            pub exclude: ::data_views::Field<&'a str>,
        }
    }
}
