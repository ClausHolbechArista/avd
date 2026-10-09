// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Categories<'a, Mode> (::validated_data::Field<categories::Item<'a, Mode>>);

pub mod categories {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub applications: ::validated_data::Field<item::Applications<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view(list)]
        pub struct Applications<'a, Mode> (::validated_data::Field<applications::Item<'a, Mode>>);

        pub mod applications {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::validated_data::RequiredValue<&'a str, Mode>,
                pub service: ::validated_data::Field<&'a str>,
            }
        }
    }
}

#[::validated_data::data_view]
pub struct FieldSets<'a, Mode> {
    pub l4_ports: ::validated_data::Field<field_sets::L4Ports<'a, Mode>>,
    pub ipv4_prefixes: ::validated_data::Field<field_sets::Ipv4Prefixes<'a, Mode>>,
}

pub mod field_sets {

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct L4Ports<'a, Mode> (::validated_data::Field<l4_ports::Item<'a, Mode>>);

    pub mod l4_ports {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub port_values: ::validated_data::Field<item::PortValues<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view(list)]
            pub struct PortValues<'a, Mode> (::validated_data::Field<&'a str>);
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct Ipv4Prefixes<'a, Mode> (::validated_data::Field<ipv4_prefixes::Item<'a, Mode>>);

    pub mod ipv4_prefixes {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub prefix_values: ::validated_data::Field<item::PrefixValues<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view(list)]
            pub struct PrefixValues<'a, Mode> (::validated_data::Field<&'a str>);
        }
    }
}

#[::validated_data::data_view]
pub struct Applications<'a, Mode> {
    pub ipv4_applications: ::validated_data::Field<applications::Ipv4Applications<'a, Mode>>,
    pub l4_applications: ::validated_data::Field<applications::L4Applications<'a, Mode>>,
}

pub mod applications {

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct Ipv4Applications<'a, Mode> (::validated_data::Field<ipv4_applications::Item<'a, Mode>>);

    pub mod ipv4_applications {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub src_prefix_set_name: ::validated_data::Field<&'a str>,
            pub dest_prefix_set_name: ::validated_data::Field<&'a str>,
            pub dscp_ranges: ::validated_data::Field<item::DscpRanges<'a, Mode>>,
            pub protocols: ::validated_data::Field<item::Protocols<'a, Mode>>,
            pub protocol_ranges: ::validated_data::Field<item::ProtocolRanges<'a, Mode>>,
            pub udp_src_port_set_name: ::validated_data::Field<&'a str>,
            pub tcp_src_port_set_name: ::validated_data::Field<&'a str>,
            pub udp_dest_port_set_name: ::validated_data::Field<&'a str>,
            pub tcp_dest_port_set_name: ::validated_data::Field<&'a str>,
        }

        pub mod item {

            #[::validated_data::data_view(list)]
            pub struct DscpRanges<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(list)]
            pub struct Protocols<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(list)]
            pub struct ProtocolRanges<'a, Mode> (::validated_data::Field<&'a str>);
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct L4Applications<'a, Mode> (::validated_data::Field<l4_applications::Item<'a, Mode>>);

    pub mod l4_applications {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub protocols: ::validated_data::Field<item::Protocols<'a, Mode>>,
            pub protocol_ranges: ::validated_data::Field<item::ProtocolRanges<'a, Mode>>,
            pub udp_src_port_set_name: ::validated_data::Field<&'a str>,
            pub tcp_src_port_set_name: ::validated_data::Field<&'a str>,
            pub udp_dest_port_set_name: ::validated_data::Field<&'a str>,
            pub tcp_dest_port_set_name: ::validated_data::Field<&'a str>,
        }

        pub mod item {

            #[::validated_data::data_view(list)]
            pub struct Protocols<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(list)]
            pub struct ProtocolRanges<'a, Mode> (::validated_data::Field<&'a str>);
        }
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct ApplicationProfiles<'a, Mode> (::validated_data::Field<application_profiles::Item<'a, Mode>>);

pub mod application_profiles {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub applications: ::validated_data::Field<item::Applications<'a, Mode>>,
        pub application_transports: ::validated_data::Field<item::ApplicationTransports<'a, Mode>>,
        pub categories: ::validated_data::Field<item::Categories<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view(list)]
        pub struct Applications<'a, Mode> (::validated_data::Field<applications::Item<'a, Mode>>);

        pub mod applications {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::validated_data::RequiredValue<&'a str, Mode>,
                pub service: ::validated_data::Field<&'a str>,
            }
        }

        #[::validated_data::data_view(list)]
        pub struct ApplicationTransports<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(list)]
        pub struct Categories<'a, Mode> (::validated_data::Field<categories::Item<'a, Mode>>);

        pub mod categories {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::validated_data::RequiredValue<&'a str, Mode>,
                pub service: ::validated_data::Field<&'a str>,
            }
        }
    }
}
