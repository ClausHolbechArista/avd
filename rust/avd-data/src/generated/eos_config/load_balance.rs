// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct LoadBalance<'a, Mode> {
    pub policies: ::data_views::Field<Policies<'a, Mode>>,
    pub cluster: ::data_views::Field<Cluster<'a, Mode>>,
}

#[::data_views::data_view]
pub struct Policies<'a, Mode> {
    pub sand_profiles: ::data_views::Field<policies::SandProfiles<'a, Mode>>,
}

pub mod policies {

    #[::data_views::data_view(indexed_list, primary_key(name))]
    pub struct SandProfiles<'a, Mode> (::data_views::Field<sand_profiles::Item<'a, Mode>>);

    pub mod sand_profiles {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::data_views::Field<&'a str>,
            pub fields: ::data_views::Field<item::Fields<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view]
            pub struct Fields<'a, Mode> {
                pub udp: ::data_views::Field<fields::Udp<'a, Mode>>,
            }

            pub mod fields {

                #[::data_views::data_view]
                pub struct Udp<'a, Mode> {
                    pub dst_port: ::data_views::RequiredValue<i64, Mode>,
                    pub payload_bytes: ::data_views::Field<&'a str>,
                    #[data_view(rename = "match")]
                    pub field_match: ::data_views::Field<udp::FieldMatch<'a, Mode>>,
                }

                pub mod udp {

                    #[::data_views::data_view]
                    pub struct FieldMatch<'a, Mode> {
                        pub payload_bits: ::data_views::RequiredValue<&'a str, Mode>,
                        pub pattern: ::data_views::RequiredValue<&'a str, Mode>,
                        pub hash_payload_bytes: ::data_views::RequiredValue<&'a str, Mode>,
                    }
                }
            }
        }
    }
}

#[::data_views::data_view]
pub struct Cluster<'a, Mode> {
    pub destination_grouping: ::data_views::Field<&'a str>,
    pub prefix_length: ::data_views::Field<i64>,
    pub forwarding_type: ::data_views::Field<&'a str>,
    pub load_balance_method_flow_round_robin: ::data_views::Field<bool>,
    pub flow: ::data_views::Field<cluster::Flow<'a, Mode>>,
    pub port_groups: ::data_views::Field<cluster::PortGroups<'a, Mode>>,
}

pub mod cluster {

    #[::data_views::data_view]
    pub struct Flow<'a, Mode> {
        pub monitor: ::data_views::Field<bool>,
        pub source_learning_aging_timeout: ::data_views::Field<i64>,
    }

    #[::data_views::data_view(indexed_list, primary_key(group))]
    pub struct PortGroups<'a, Mode> (::data_views::Field<port_groups::Item<'a, Mode>>);

    pub mod port_groups {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub group: ::data_views::Field<&'a str>,
            pub balance_factor: ::data_views::Field<i64>,
            pub interface: ::data_views::Field<&'a str>,
            pub flow: ::data_views::Field<item::Flow<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view]
            pub struct Flow<'a, Mode> {
                pub limit: ::data_views::Field<i64>,
                pub warning: ::data_views::Field<i64>,
                pub exhaustion_action: ::data_views::Field<flow::ExhaustionAction<'a, Mode>>,
            }

            pub mod flow {

                #[::data_views::data_view]
                pub struct ExhaustionAction<'a, Mode> {
                    pub dscp: ::data_views::Field<i64>,
                    pub traffic_class: ::data_views::Field<i64>,
                }
            }
        }
    }
}
