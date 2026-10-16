// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "EOSConfig.ApplicationTrafficRecognition.Categories", schema = List(38), item_name = "EOSConfig.ApplicationTrafficRecognition.CategoriesItem"))]
pub struct Categories<'a, Mode> (::validated_data::Field<categories::Item<'a, Mode>>);

pub mod categories {

    #[::validated_data::data_view(python(name = "EOSConfig.ApplicationTrafficRecognition.CategoriesItemBase", schema = Dict(67)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(37))]
        pub name: ::validated_data::Field<&'a str>,
        #[data_view(schema = List(37))]
        pub applications: ::validated_data::Field<item::Applications<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view(list, python(name = "EOSConfig.ApplicationTrafficRecognition.CategoriesItemBase.Applications", schema = List(37)))]
        pub struct Applications<'a, Mode> (::validated_data::Field<applications::Item<'a, Mode>>);

        pub mod applications {

            #[::validated_data::data_view(python(name = "EOSConfig.ApplicationTrafficRecognition.CategoriesItemBase.ApplicationsItem", schema = Dict(66)))]
            pub struct Item<'a, Mode> {
                #[data_view(schema = Str(38))]
                pub name: ::validated_data::RequiredValue<&'a str, Mode>,
                #[data_view(schema = Str(39))]
                pub service: ::validated_data::Field<&'a str>,
            }
        }
    }
}

#[::validated_data::data_view(python(name = "EOSConfig.ApplicationTrafficRecognition.FieldSets", schema = Dict(70)))]
pub struct FieldSets<'a, Mode> {
    #[data_view(schema = List(40))]
    pub l4_ports: ::validated_data::Field<field_sets::L4Ports<'a, Mode>>,
    #[data_view(schema = List(42))]
    pub ipv4_prefixes: ::validated_data::Field<field_sets::Ipv4Prefixes<'a, Mode>>,
}

pub mod field_sets {

    #[::validated_data::data_view(indexed_list, primary_key(name), python(name = "EOSConfig.ApplicationTrafficRecognition.FieldSets.L4Ports", schema = List(40), item_name = "EOSConfig.ApplicationTrafficRecognition.FieldSets.L4PortsItem"))]
    pub struct L4Ports<'a, Mode> (::validated_data::Field<l4_ports::Item<'a, Mode>>);

    pub mod l4_ports {

        #[::validated_data::data_view(python(name = "EOSConfig.ApplicationTrafficRecognition.FieldSets.L4PortsItemBase", schema = Dict(68)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(40))]
            pub name: ::validated_data::Field<&'a str>,
            #[data_view(schema = List(39))]
            pub port_values: ::validated_data::Field<item::PortValues<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view(list, python(name = "EOSConfig.ApplicationTrafficRecognition.FieldSets.L4PortsItemBase.PortValues", schema = List(39)))]
            pub struct PortValues<'a, Mode> (::validated_data::Field<&'a str>);
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(name), python(name = "EOSConfig.ApplicationTrafficRecognition.FieldSets.Ipv4Prefixes", schema = List(42), item_name = "EOSConfig.ApplicationTrafficRecognition.FieldSets.Ipv4PrefixesItem"))]
    pub struct Ipv4Prefixes<'a, Mode> (::validated_data::Field<ipv4_prefixes::Item<'a, Mode>>);

    pub mod ipv4_prefixes {

        #[::validated_data::data_view(python(name = "EOSConfig.ApplicationTrafficRecognition.FieldSets.Ipv4PrefixesItemBase", schema = Dict(69)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(42))]
            pub name: ::validated_data::Field<&'a str>,
            #[data_view(schema = List(41))]
            pub prefix_values: ::validated_data::Field<item::PrefixValues<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view(list, python(name = "EOSConfig.ApplicationTrafficRecognition.FieldSets.Ipv4PrefixesItemBase.PrefixValues", schema = List(41)))]
            pub struct PrefixValues<'a, Mode> (::validated_data::Field<&'a str>);
        }
    }
}

#[::validated_data::data_view(python(name = "EOSConfig.ApplicationTrafficRecognition.Applications", schema = Dict(73)))]
pub struct Applications<'a, Mode> {
    #[data_view(schema = List(46))]
    pub ipv4_applications: ::validated_data::Field<applications::Ipv4Applications<'a, Mode>>,
    #[data_view(schema = List(47))]
    pub l4_applications: ::validated_data::Field<applications::L4Applications<'a, Mode>>,
}

pub mod applications {

    #[::validated_data::data_view(indexed_list, primary_key(name), python(name = "EOSConfig.ApplicationTrafficRecognition.Applications.Ipv4Applications", schema = List(46), item_name = "EOSConfig.ApplicationTrafficRecognition.Applications.Ipv4ApplicationsItem"))]
    pub struct Ipv4Applications<'a, Mode> (::validated_data::Field<ipv4_applications::Item<'a, Mode>>);

    pub mod ipv4_applications {

        #[::validated_data::data_view(python(name = "EOSConfig.ApplicationTrafficRecognition.Applications.Ipv4ApplicationsItemBase", schema = Dict(71)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(44))]
            pub name: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(45))]
            pub src_prefix_set_name: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(46))]
            pub dest_prefix_set_name: ::validated_data::Field<&'a str>,
            #[data_view(schema = List(43))]
            pub dscp_ranges: ::validated_data::Field<item::DscpRanges<'a, Mode>>,
            #[data_view(schema = List(44))]
            pub protocols: ::validated_data::Field<item::Protocols<'a, Mode>>,
            #[data_view(schema = List(45))]
            pub protocol_ranges: ::validated_data::Field<item::ProtocolRanges<'a, Mode>>,
            #[data_view(schema = Str(50))]
            pub udp_src_port_set_name: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(51))]
            pub tcp_src_port_set_name: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(52))]
            pub udp_dest_port_set_name: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(53))]
            pub tcp_dest_port_set_name: ::validated_data::Field<&'a str>,
        }

        pub mod item {

            #[::validated_data::data_view(list, python(name = "EOSConfig.ApplicationTrafficRecognition.Applications.Ipv4ApplicationsItemBase.DscpRanges", schema = List(43)))]
            pub struct DscpRanges<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(list, python(name = "EOSConfig.ApplicationTrafficRecognition.Applications.Ipv4ApplicationsItemBase.Protocols", schema = List(44)))]
            pub struct Protocols<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(list, python(name = "EOSConfig.ApplicationTrafficRecognition.Applications.Ipv4ApplicationsItemBase.ProtocolRanges", schema = List(45)))]
            pub struct ProtocolRanges<'a, Mode> (::validated_data::Field<&'a str>);
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(name), python(name = "EOSConfig.ApplicationTrafficRecognition.Applications.L4Applications", schema = List(47), item_name = "EOSConfig.ApplicationTrafficRecognition.Applications.L4ApplicationsItem"))]
    pub struct L4Applications<'a, Mode> (::validated_data::Field<l4_applications::Item<'a, Mode>>);

    pub mod l4_applications {

        #[::validated_data::data_view(python(name = "EOSConfig.ApplicationTrafficRecognition.Applications.L4ApplicationsItemBase", schema = Dict(72)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(44))]
            pub name: ::validated_data::Field<&'a str>,
            #[data_view(schema = List(44))]
            pub protocols: ::validated_data::Field<item::Protocols<'a, Mode>>,
            #[data_view(schema = List(45))]
            pub protocol_ranges: ::validated_data::Field<item::ProtocolRanges<'a, Mode>>,
            #[data_view(schema = Str(50))]
            pub udp_src_port_set_name: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(51))]
            pub tcp_src_port_set_name: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(52))]
            pub udp_dest_port_set_name: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(53))]
            pub tcp_dest_port_set_name: ::validated_data::Field<&'a str>,
        }

        pub mod item {

            #[::validated_data::data_view(list, python(name = "EOSConfig.ApplicationTrafficRecognition.Applications.L4ApplicationsItemBase.Protocols", schema = List(44)))]
            pub struct Protocols<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(list, python(name = "EOSConfig.ApplicationTrafficRecognition.Applications.L4ApplicationsItemBase.ProtocolRanges", schema = List(45)))]
            pub struct ProtocolRanges<'a, Mode> (::validated_data::Field<&'a str>);
        }
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "EOSConfig.ApplicationTrafficRecognition.ApplicationProfiles", schema = List(51), item_name = "EOSConfig.ApplicationTrafficRecognition.ApplicationProfilesItem"))]
pub struct ApplicationProfiles<'a, Mode> (::validated_data::Field<application_profiles::Item<'a, Mode>>);

pub mod application_profiles {

    #[::validated_data::data_view(python(name = "EOSConfig.ApplicationTrafficRecognition.ApplicationProfilesItemBase", schema = Dict(76)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(54))]
        pub name: ::validated_data::Field<&'a str>,
        #[data_view(schema = List(48))]
        pub applications: ::validated_data::Field<item::Applications<'a, Mode>>,
        #[data_view(schema = List(49))]
        pub application_transports: ::validated_data::Field<item::ApplicationTransports<'a, Mode>>,
        #[data_view(schema = List(50))]
        pub categories: ::validated_data::Field<item::Categories<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view(list, python(name = "EOSConfig.ApplicationTrafficRecognition.ApplicationProfilesItemBase.Applications", schema = List(48)))]
        pub struct Applications<'a, Mode> (::validated_data::Field<applications::Item<'a, Mode>>);

        pub mod applications {

            #[::validated_data::data_view(python(name = "EOSConfig.ApplicationTrafficRecognition.ApplicationProfilesItemBase.ApplicationsItem", schema = Dict(74)))]
            pub struct Item<'a, Mode> {
                #[data_view(schema = Str(55))]
                pub name: ::validated_data::RequiredValue<&'a str, Mode>,
                #[data_view(schema = Str(39))]
                pub service: ::validated_data::Field<&'a str>,
            }
        }

        #[::validated_data::data_view(list, python(name = "EOSConfig.ApplicationTrafficRecognition.ApplicationProfilesItemBase.ApplicationTransports", schema = List(49)))]
        pub struct ApplicationTransports<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(list, python(name = "EOSConfig.ApplicationTrafficRecognition.ApplicationProfilesItemBase.Categories", schema = List(50)))]
        pub struct Categories<'a, Mode> (::validated_data::Field<categories::Item<'a, Mode>>);

        pub mod categories {

            #[::validated_data::data_view(python(name = "EOSConfig.ApplicationTrafficRecognition.ApplicationProfilesItemBase.CategoriesItem", schema = Dict(75)))]
            pub struct Item<'a, Mode> {
                #[data_view(schema = Str(57))]
                pub name: ::validated_data::RequiredValue<&'a str, Mode>,
                #[data_view(schema = Str(39))]
                pub service: ::validated_data::Field<&'a str>,
            }
        }
    }
}
