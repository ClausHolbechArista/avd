// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Policies<'a, Mode> {
    pub sand_profiles: ::validated_data::Field<policies::SandProfiles<'a, Mode>>,
}

pub mod policies {

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct SandProfiles<'a, Mode> (::validated_data::Field<sand_profiles::Item<'a, Mode>>);

    pub mod sand_profiles {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub fields: ::validated_data::Field<item::Fields<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view]
            pub struct Fields<'a, Mode> {
                pub udp: ::validated_data::Field<fields::Udp<'a, Mode>>,
            }

            pub mod fields {

                #[::validated_data::data_view]
                pub struct Udp<'a, Mode> {
                    pub dst_port: ::validated_data::RequiredValue<i64, Mode>,
                    pub payload_bytes: ::validated_data::Field<&'a str>,
                    #[data_view(rename = "match")]
                    pub field_match: ::validated_data::Field<udp::FieldMatch<'a, Mode>>,
                }

                pub mod udp {

                    #[::validated_data::data_view]
                    pub struct FieldMatch<'a, Mode> {
                        pub payload_bits: ::validated_data::RequiredValue<&'a str, Mode>,
                        pub pattern: ::validated_data::RequiredValue<&'a str, Mode>,
                        pub hash_payload_bytes: ::validated_data::RequiredValue<&'a str, Mode>,
                    }
                }
            }
        }
    }
}

#[::validated_data::data_view]
pub struct Cluster<'a, Mode> {
    pub destination_grouping: ::validated_data::Field<&'a str>,
    pub prefix_length: ::validated_data::Field<i64>,
    pub forwarding_type: ::validated_data::Field<&'a str>,
    pub load_balance_method_flow_round_robin: ::validated_data::Field<bool>,
    pub flow: ::validated_data::Field<cluster::Flow<'a, Mode>>,
    pub port_groups: ::validated_data::Field<cluster::PortGroups<'a, Mode>>,
}

pub mod cluster {

    #[::validated_data::data_view]
    pub struct Flow<'a, Mode> {
        pub monitor: ::validated_data::Field<bool>,
        pub source_learning_aging_timeout: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view(indexed_list, primary_key(group))]
    pub struct PortGroups<'a, Mode> (::validated_data::Field<port_groups::Item<'a, Mode>>);

    pub mod port_groups {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub group: ::validated_data::Field<&'a str>,
            pub balance_factor: ::validated_data::Field<i64>,
            pub interface: ::validated_data::Field<&'a str>,
            pub flow: ::validated_data::Field<item::Flow<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view]
            pub struct Flow<'a, Mode> {
                pub limit: ::validated_data::Field<i64>,
                pub warning: ::validated_data::Field<i64>,
                pub exhaustion_action: ::validated_data::Field<flow::ExhaustionAction<'a, Mode>>,
            }

            pub mod flow {

                #[::validated_data::data_view]
                pub struct ExhaustionAction<'a, Mode> {
                    pub dscp: ::validated_data::Field<i64>,
                    pub traffic_class: ::validated_data::Field<i64>,
                }
            }
        }
    }
}
