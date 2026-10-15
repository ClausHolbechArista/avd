// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Ingress<'a, Mode> {
    pub collection: ::validated_data::Field<ingress::Collection<'a, Mode>>,
    pub sample: ::validated_data::Field<ingress::Sample<'a, Mode>>,
}

pub mod ingress {

    #[::validated_data::data_view]
    pub struct Collection<'a, Mode> {
        pub source: ::validated_data::Field<&'a str>,
        pub destination: ::validated_data::Field<&'a str>,
        pub version: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct Sample<'a, Mode> {
        pub rate: ::validated_data::Field<i64>,
        pub tcp_udp_checksum: ::validated_data::Field<sample::TcpUdpChecksum<'a, Mode>>,
    }

    pub mod sample {

        #[::validated_data::data_view]
        pub struct TcpUdpChecksum<'a, Mode> {
            pub value: ::validated_data::Field<i64>,
            pub mask: ::validated_data::Field<&'a str>,
        }
    }
}

#[::validated_data::data_view]
pub struct MarkerVxlan<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub header_word_zero_bit: ::validated_data::Field<i64>,
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Profiles<'a, Mode> (::validated_data::Field<profiles::Item<'a, Mode>>);

pub mod profiles {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub ingress_sample_policy: ::validated_data::Field<&'a str>,
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct SamplePolicies<'a, Mode> (::validated_data::Field<sample_policies::Item<'a, Mode>>);

pub mod sample_policies {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub match_rules: ::validated_data::Field<item::MatchRules<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view(indexed_list, primary_key(name))]
        pub struct MatchRules<'a, Mode> (::validated_data::Field<match_rules::Item<'a, Mode>>);

        pub mod match_rules {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::validated_data::Field<&'a str>,
                #[data_view(rename = "type")]
                pub field_type: ::validated_data::RequiredValue<&'a str, Mode>,
                pub destination_prefix: ::validated_data::Field<&'a str>,
                pub source_prefix: ::validated_data::Field<&'a str>,
                pub protocols: ::validated_data::Field<item::Protocols<'a, Mode>>,
            }

            pub mod item {

                #[::validated_data::data_view(indexed_list, primary_key(protocol))]
                pub struct Protocols<'a, Mode> (::validated_data::Field<protocols::Item<'a, Mode>>);

                pub mod protocols {

                    #[::validated_data::data_view]
                    pub struct Item<'a, Mode> {
                        pub protocol: ::validated_data::Field<&'a str>,
                        pub source_ports: ::validated_data::Field<item::SourcePorts<'a, Mode>>,
                        pub destination_ports: ::validated_data::Field<item::DestinationPorts<'a, Mode>>,
                    }

                    pub mod item {

                        #[::validated_data::data_view(list)]
                        pub struct SourcePorts<'a, Mode> (::validated_data::Field<&'a str>);

                        #[::validated_data::data_view(list)]
                        pub struct DestinationPorts<'a, Mode> (::validated_data::Field<&'a str>);
                    }
                }
            }
        }
    }
}
