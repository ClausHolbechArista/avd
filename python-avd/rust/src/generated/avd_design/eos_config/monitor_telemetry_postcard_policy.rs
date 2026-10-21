// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct MonitorTelemetryPostcardPolicy<'a, Mode> {
    pub disabled: ::data_views::Field<bool>,
    pub ingress: ::data_views::Field<Ingress<'a, Mode>>,
    pub marker_vxlan: ::data_views::Field<MarkerVxlan<'a, Mode>>,
    pub profiles: ::data_views::Field<Profiles<'a, Mode>>,
    pub sample_policies: ::data_views::Field<SamplePolicies<'a, Mode>>,
}

#[::data_views::data_view]
pub struct Ingress<'a, Mode> {
    pub collection: ::data_views::Field<ingress::Collection<'a, Mode>>,
    pub sample: ::data_views::Field<ingress::Sample<'a, Mode>>,
}

pub mod ingress {

    #[::data_views::data_view]
    pub struct Collection<'a, Mode> {
        pub source: ::data_views::Field<&'a str>,
        pub destination: ::data_views::Field<&'a str>,
        pub version: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct Sample<'a, Mode> {
        pub rate: ::data_views::Field<i64>,
        pub tcp_udp_checksum: ::data_views::Field<sample::TcpUdpChecksum<'a, Mode>>,
    }

    pub mod sample {

        #[::data_views::data_view]
        pub struct TcpUdpChecksum<'a, Mode> {
            pub value: ::data_views::Field<i64>,
            pub mask: ::data_views::Field<&'a str>,
        }
    }
}

#[::data_views::data_view]
pub struct MarkerVxlan<'a, Mode> {
    pub enabled: ::data_views::Field<bool>,
    pub header_word_zero_bit: ::data_views::Field<i64>,
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Profiles<'a, Mode> (::data_views::Field<profiles::Item<'a, Mode>>);

pub mod profiles {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub ingress_sample_policy: ::data_views::Field<&'a str>,
    }
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct SamplePolicies<'a, Mode> (::data_views::Field<sample_policies::Item<'a, Mode>>);

pub mod sample_policies {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub match_rules: ::data_views::Field<item::MatchRules<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view(indexed_list, primary_key(name))]
        pub struct MatchRules<'a, Mode> (::data_views::Field<match_rules::Item<'a, Mode>>);

        pub mod match_rules {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::data_views::Field<&'a str>,
                #[data_view(rename = "type")]
                pub field_type: ::data_views::RequiredValue<&'a str, Mode>,
                pub destination_prefix: ::data_views::Field<&'a str>,
                pub source_prefix: ::data_views::Field<&'a str>,
                pub protocols: ::data_views::Field<item::Protocols<'a, Mode>>,
            }

            pub mod item {

                #[::data_views::data_view(indexed_list, primary_key(protocol))]
                pub struct Protocols<'a, Mode> (::data_views::Field<protocols::Item<'a, Mode>>);

                pub mod protocols {

                    #[::data_views::data_view]
                    pub struct Item<'a, Mode> {
                        pub protocol: ::data_views::Field<&'a str>,
                        pub source_ports: ::data_views::Field<item::SourcePorts<'a, Mode>>,
                        pub destination_ports: ::data_views::Field<item::DestinationPorts<'a, Mode>>,
                    }

                    pub mod item {

                        #[::data_views::data_view(list)]
                        pub struct SourcePorts<'a, Mode> (::data_views::Field<&'a str>);

                        #[::data_views::data_view(list)]
                        pub struct DestinationPorts<'a, Mode> (::data_views::Field<&'a str>);
                    }
                }
            }
        }
    }
}
