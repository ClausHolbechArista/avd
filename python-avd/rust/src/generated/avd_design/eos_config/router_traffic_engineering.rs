// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct RouterId<'a, Mode> {
    pub ipv4: ::validated_data::Field<&'a str>,
    pub ipv6: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view]
pub struct SegmentRouting<'a, Mode> {
    pub colored_tunnel_rib: ::validated_data::Field<bool>,
    pub policy_endpoints: ::validated_data::Field<segment_routing::PolicyEndpoints<'a, Mode>>,
}

pub mod segment_routing {

    #[::validated_data::data_view(list)]
    pub struct PolicyEndpoints<'a, Mode> (::validated_data::Field<policy_endpoints::Item<'a, Mode>>);

    pub mod policy_endpoints {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub address: ::validated_data::Field<&'a str>,
            pub colors: ::validated_data::Field<item::Colors<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view(indexed_list, primary_key(value))]
            pub struct Colors<'a, Mode> (::validated_data::Field<colors::Item<'a, Mode>>);

            pub mod colors {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub value: ::validated_data::Field<i64>,
                    pub binding_sid: ::validated_data::Field<i64>,
                    pub description: ::validated_data::Field<&'a str>,
                    pub name: ::validated_data::Field<&'a str>,
                    pub sbfd_remote_discriminator: ::validated_data::Field<&'a str>,
                    pub path_group: ::validated_data::Field<item::PathGroup<'a, Mode>>,
                }

                pub mod item {

                    #[::validated_data::data_view(list)]
                    pub struct PathGroup<'a, Mode> (::validated_data::Field<path_group::Item<'a, Mode>>);

                    pub mod path_group {

                        #[::validated_data::data_view]
                        pub struct Item<'a, Mode> {
                            pub preference: ::validated_data::Field<i64>,
                            pub explicit_null: ::validated_data::Field<&'a str>,
                            pub segment_list: ::validated_data::Field<item::SegmentList<'a, Mode>>,
                        }

                        pub mod item {

                            #[::validated_data::data_view(list)]
                            pub struct SegmentList<'a, Mode> (::validated_data::Field<segment_list::Item<'a, Mode>>);

                            pub mod segment_list {

                                #[::validated_data::data_view]
                                pub struct Item<'a, Mode> {
                                    pub label_stack: ::validated_data::Field<&'a str>,
                                    pub weight: ::validated_data::Field<i64>,
                                    pub index: ::validated_data::Field<i64>,
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[::validated_data::data_view(indexed_list, primary_key(number))]
pub struct FlexAlgos<'a, Mode> (::validated_data::Field<flex_algos::Item<'a, Mode>>);

pub mod flex_algos {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub number: ::validated_data::Field<i64>,
        pub name: ::validated_data::RequiredValue<&'a str, Mode>,
        pub administrative_group: ::validated_data::Field<item::AdministrativeGroup<'a, Mode>>,
        pub metric: ::validated_data::Field<&'a str>,
        pub priority: ::validated_data::Field<i64>,
        pub color: ::validated_data::Field<i64>,
        pub srlg_exclude: ::validated_data::Field<&'a str>,
    }

    pub mod item {

        #[::validated_data::data_view]
        pub struct AdministrativeGroup<'a, Mode> {
            pub include_all: ::validated_data::Field<&'a str>,
            pub include_any: ::validated_data::Field<&'a str>,
            pub exclude: ::validated_data::Field<&'a str>,
        }
    }
}
