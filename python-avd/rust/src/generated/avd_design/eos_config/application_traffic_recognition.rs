// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "EOSConfig.ApplicationTrafficRecognition.Categories", item_name = "EOSConfig.ApplicationTrafficRecognition.CategoriesItem"))]
pub struct Categories<'a, Mode> (::validated_data::Field<categories::Item<'a, Mode>>);

pub mod categories {

    #[::validated_data::data_view(python(name = "EOSConfig.ApplicationTrafficRecognition.CategoriesItemBase"))]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub applications: ::validated_data::Field<item::Applications<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view(list, python(name = "EOSConfig.ApplicationTrafficRecognition.CategoriesItemBase.Applications"))]
        pub struct Applications<'a, Mode> (::validated_data::Field<applications::Item<'a, Mode>>);

        pub mod applications {

            #[::validated_data::data_view(python(name = "EOSConfig.ApplicationTrafficRecognition.CategoriesItemBase.ApplicationsItem"))]
            pub struct Item<'a, Mode> {
                pub name: ::validated_data::RequiredValue<&'a str, Mode>,
                pub service: ::validated_data::Field<&'a str>,
            }
        }
    }
}

#[::validated_data::data_view(python(name = "EOSConfig.ApplicationTrafficRecognition.FieldSets"))]
pub struct FieldSets<'a, Mode> {
    pub l4_ports: ::validated_data::Field<field_sets::L4Ports<'a, Mode>>,
    pub ipv4_prefixes: ::validated_data::Field<field_sets::Ipv4Prefixes<'a, Mode>>,
}

pub mod field_sets {

    #[::validated_data::data_view(indexed_list, primary_key(name), python(name = "EOSConfig.ApplicationTrafficRecognition.FieldSets.L4Ports", item_name = "EOSConfig.ApplicationTrafficRecognition.FieldSets.L4PortsItem"))]
    pub struct L4Ports<'a, Mode> (::validated_data::Field<l4_ports::Item<'a, Mode>>);

    pub mod l4_ports {

        #[::validated_data::data_view(python(name = "EOSConfig.ApplicationTrafficRecognition.FieldSets.L4PortsItemBase"))]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub port_values: ::validated_data::Field<item::PortValues<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view(list, python(name = "EOSConfig.ApplicationTrafficRecognition.FieldSets.L4PortsItemBase.PortValues"))]
            pub struct PortValues<'a, Mode> (::validated_data::Field<&'a str>);
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(name), python(name = "EOSConfig.ApplicationTrafficRecognition.FieldSets.Ipv4Prefixes", item_name = "EOSConfig.ApplicationTrafficRecognition.FieldSets.Ipv4PrefixesItem"))]
    pub struct Ipv4Prefixes<'a, Mode> (::validated_data::Field<ipv4_prefixes::Item<'a, Mode>>);

    pub mod ipv4_prefixes {

        #[::validated_data::data_view(python(name = "EOSConfig.ApplicationTrafficRecognition.FieldSets.Ipv4PrefixesItemBase"))]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub prefix_values: ::validated_data::Field<item::PrefixValues<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view(list, python(name = "EOSConfig.ApplicationTrafficRecognition.FieldSets.Ipv4PrefixesItemBase.PrefixValues"))]
            pub struct PrefixValues<'a, Mode> (::validated_data::Field<&'a str>);
        }
    }
}

#[::validated_data::data_view(python(name = "EOSConfig.ApplicationTrafficRecognition.Applications"))]
pub struct Applications<'a, Mode> {
    pub ipv4_applications: ::validated_data::Field<applications::Ipv4Applications<'a, Mode>>,
    pub l4_applications: ::validated_data::Field<applications::L4Applications<'a, Mode>>,
}

pub mod applications {

    #[::validated_data::data_view(indexed_list, primary_key(name), python(name = "EOSConfig.ApplicationTrafficRecognition.Applications.Ipv4Applications", item_name = "EOSConfig.ApplicationTrafficRecognition.Applications.Ipv4ApplicationsItem"))]
    pub struct Ipv4Applications<'a, Mode> (::validated_data::Field<ipv4_applications::Item<'a, Mode>>);

    pub mod ipv4_applications {

        #[::validated_data::data_view(python(name = "EOSConfig.ApplicationTrafficRecognition.Applications.Ipv4ApplicationsItemBase"))]
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

            #[::validated_data::data_view(list, python(name = "EOSConfig.ApplicationTrafficRecognition.Applications.Ipv4ApplicationsItemBase.DscpRanges"))]
            pub struct DscpRanges<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(list, python(name = "EOSConfig.ApplicationTrafficRecognition.Applications.Ipv4ApplicationsItemBase.Protocols"))]
            pub struct Protocols<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(list, python(name = "EOSConfig.ApplicationTrafficRecognition.Applications.Ipv4ApplicationsItemBase.ProtocolRanges"))]
            pub struct ProtocolRanges<'a, Mode> (::validated_data::Field<&'a str>);
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(name), python(name = "EOSConfig.ApplicationTrafficRecognition.Applications.L4Applications", item_name = "EOSConfig.ApplicationTrafficRecognition.Applications.L4ApplicationsItem"))]
    pub struct L4Applications<'a, Mode> (::validated_data::Field<l4_applications::Item<'a, Mode>>);

    pub mod l4_applications {

        #[::validated_data::data_view(python(name = "EOSConfig.ApplicationTrafficRecognition.Applications.L4ApplicationsItemBase"))]
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

            #[::validated_data::data_view(list, python(name = "EOSConfig.ApplicationTrafficRecognition.Applications.L4ApplicationsItemBase.Protocols"))]
            pub struct Protocols<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(list, python(name = "EOSConfig.ApplicationTrafficRecognition.Applications.L4ApplicationsItemBase.ProtocolRanges"))]
            pub struct ProtocolRanges<'a, Mode> (::validated_data::Field<&'a str>);
        }
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "EOSConfig.ApplicationTrafficRecognition.ApplicationProfiles", item_name = "EOSConfig.ApplicationTrafficRecognition.ApplicationProfilesItem"))]
pub struct ApplicationProfiles<'a, Mode> (::validated_data::Field<application_profiles::Item<'a, Mode>>);

pub mod application_profiles {

    #[::validated_data::data_view(python(name = "EOSConfig.ApplicationTrafficRecognition.ApplicationProfilesItemBase"))]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub applications: ::validated_data::Field<item::Applications<'a, Mode>>,
        pub application_transports: ::validated_data::Field<item::ApplicationTransports<'a, Mode>>,
        pub categories: ::validated_data::Field<item::Categories<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view(list, python(name = "EOSConfig.ApplicationTrafficRecognition.ApplicationProfilesItemBase.Applications"))]
        pub struct Applications<'a, Mode> (::validated_data::Field<applications::Item<'a, Mode>>);

        pub mod applications {

            #[::validated_data::data_view(python(name = "EOSConfig.ApplicationTrafficRecognition.ApplicationProfilesItemBase.ApplicationsItem"))]
            pub struct Item<'a, Mode> {
                pub name: ::validated_data::RequiredValue<&'a str, Mode>,
                pub service: ::validated_data::Field<&'a str>,
            }
        }

        #[::validated_data::data_view(list, python(name = "EOSConfig.ApplicationTrafficRecognition.ApplicationProfilesItemBase.ApplicationTransports"))]
        pub struct ApplicationTransports<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(list, python(name = "EOSConfig.ApplicationTrafficRecognition.ApplicationProfilesItemBase.Categories"))]
        pub struct Categories<'a, Mode> (::validated_data::Field<categories::Item<'a, Mode>>);

        pub mod categories {

            #[::validated_data::data_view(python(name = "EOSConfig.ApplicationTrafficRecognition.ApplicationProfilesItemBase.CategoriesItem"))]
            pub struct Item<'a, Mode> {
                pub name: ::validated_data::RequiredValue<&'a str, Mode>,
                pub service: ::validated_data::Field<&'a str>,
            }
        }
    }
}
