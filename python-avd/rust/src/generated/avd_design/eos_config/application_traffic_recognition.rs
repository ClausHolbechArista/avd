// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "EOSConfig.ApplicationTrafficRecognition", schema = Dict(77)))]
pub struct ApplicationTrafficRecognition<'a, Mode> {
    #[data_view(schema = List(38))]
    pub categories: ::data_views::Field<Categories<'a, Mode>>,
    #[data_view(schema = Dict(70))]
    pub field_sets: ::data_views::Field<FieldSets<'a, Mode>>,
    #[data_view(schema = Dict(73))]
    pub applications: ::data_views::Field<Applications<'a, Mode>>,
    #[data_view(schema = List(51))]
    pub application_profiles: ::data_views::Field<ApplicationProfiles<'a, Mode>>,
}

#[::data_views::data_view(indexed_list, primary_key(name), python(name = "EOSConfig.ApplicationTrafficRecognition.Categories", schema = List(38), item_name = "EOSConfig.ApplicationTrafficRecognition.CategoriesItem"))]
pub struct Categories<'a, Mode> (::data_views::Field<categories::Item<'a, Mode>>);

pub mod categories {

    #[::data_views::data_view(python(name = "EOSConfig.ApplicationTrafficRecognition.CategoriesItemBase", schema = Dict(67)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(37))]
        pub name: ::data_views::Field<&'a str>,
        #[data_view(schema = List(37))]
        pub applications: ::data_views::Field<item::Applications<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view(list, python(name = "EOSConfig.ApplicationTrafficRecognition.CategoriesItemBase.Applications", schema = List(37)))]
        pub struct Applications<'a, Mode> (::data_views::Field<applications::Item<'a, Mode>>);

        pub mod applications {

            #[::data_views::data_view(python(name = "EOSConfig.ApplicationTrafficRecognition.CategoriesItemBase.ApplicationsItem", schema = Dict(66)))]
            pub struct Item<'a, Mode> {
                #[data_view(schema = Str(38))]
                pub name: ::data_views::RequiredValue<&'a str, Mode>,
                #[data_view(schema = Str(39))]
                pub service: ::data_views::Field<&'a str>,
            }
        }
    }
}

#[::data_views::data_view(python(name = "EOSConfig.ApplicationTrafficRecognition.FieldSets", schema = Dict(70)))]
pub struct FieldSets<'a, Mode> {
    #[data_view(schema = List(40))]
    pub l4_ports: ::data_views::Field<field_sets::L4Ports<'a, Mode>>,
    #[data_view(schema = List(42))]
    pub ipv4_prefixes: ::data_views::Field<field_sets::Ipv4Prefixes<'a, Mode>>,
}

pub mod field_sets {

    #[::data_views::data_view(indexed_list, primary_key(name), python(name = "EOSConfig.ApplicationTrafficRecognition.FieldSets.L4Ports", schema = List(40), item_name = "EOSConfig.ApplicationTrafficRecognition.FieldSets.L4PortsItem"))]
    pub struct L4Ports<'a, Mode> (::data_views::Field<l4_ports::Item<'a, Mode>>);

    pub mod l4_ports {

        #[::data_views::data_view(python(name = "EOSConfig.ApplicationTrafficRecognition.FieldSets.L4PortsItemBase", schema = Dict(68)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(40))]
            pub name: ::data_views::Field<&'a str>,
            #[data_view(schema = List(39))]
            pub port_values: ::data_views::Field<item::PortValues<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view(list, python(name = "EOSConfig.ApplicationTrafficRecognition.FieldSets.L4PortsItemBase.PortValues", schema = List(39)))]
            pub struct PortValues<'a, Mode> (::data_views::Field<&'a str>);
        }
    }

    #[::data_views::data_view(indexed_list, primary_key(name), python(name = "EOSConfig.ApplicationTrafficRecognition.FieldSets.Ipv4Prefixes", schema = List(42), item_name = "EOSConfig.ApplicationTrafficRecognition.FieldSets.Ipv4PrefixesItem"))]
    pub struct Ipv4Prefixes<'a, Mode> (::data_views::Field<ipv4_prefixes::Item<'a, Mode>>);

    pub mod ipv4_prefixes {

        #[::data_views::data_view(python(name = "EOSConfig.ApplicationTrafficRecognition.FieldSets.Ipv4PrefixesItemBase", schema = Dict(69)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(42))]
            pub name: ::data_views::Field<&'a str>,
            #[data_view(schema = List(41))]
            pub prefix_values: ::data_views::Field<item::PrefixValues<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view(list, python(name = "EOSConfig.ApplicationTrafficRecognition.FieldSets.Ipv4PrefixesItemBase.PrefixValues", schema = List(41)))]
            pub struct PrefixValues<'a, Mode> (::data_views::Field<&'a str>);
        }
    }
}

#[::data_views::data_view(python(name = "EOSConfig.ApplicationTrafficRecognition.Applications", schema = Dict(73)))]
pub struct Applications<'a, Mode> {
    #[data_view(schema = List(46))]
    pub ipv4_applications: ::data_views::Field<applications::Ipv4Applications<'a, Mode>>,
    #[data_view(schema = List(47))]
    pub l4_applications: ::data_views::Field<applications::L4Applications<'a, Mode>>,
}

pub mod applications {

    #[::data_views::data_view(indexed_list, primary_key(name), python(name = "EOSConfig.ApplicationTrafficRecognition.Applications.Ipv4Applications", schema = List(46), item_name = "EOSConfig.ApplicationTrafficRecognition.Applications.Ipv4ApplicationsItem"))]
    pub struct Ipv4Applications<'a, Mode> (::data_views::Field<ipv4_applications::Item<'a, Mode>>);

    pub mod ipv4_applications {

        #[::data_views::data_view(python(name = "EOSConfig.ApplicationTrafficRecognition.Applications.Ipv4ApplicationsItemBase", schema = Dict(71)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(44))]
            pub name: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(45))]
            pub src_prefix_set_name: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(46))]
            pub dest_prefix_set_name: ::data_views::Field<&'a str>,
            #[data_view(schema = List(43))]
            pub dscp_ranges: ::data_views::Field<item::DscpRanges<'a, Mode>>,
            #[data_view(schema = List(44))]
            pub protocols: ::data_views::Field<item::Protocols<'a, Mode>>,
            #[data_view(schema = List(45))]
            pub protocol_ranges: ::data_views::Field<item::ProtocolRanges<'a, Mode>>,
            #[data_view(schema = Str(50))]
            pub udp_src_port_set_name: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(51))]
            pub tcp_src_port_set_name: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(52))]
            pub udp_dest_port_set_name: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(53))]
            pub tcp_dest_port_set_name: ::data_views::Field<&'a str>,
        }

        pub mod item {

            #[::data_views::data_view(list, python(name = "EOSConfig.ApplicationTrafficRecognition.Applications.Ipv4ApplicationsItemBase.DscpRanges", schema = List(43)))]
            pub struct DscpRanges<'a, Mode> (::data_views::Field<&'a str>);

            #[::data_views::data_view(list, python(name = "EOSConfig.ApplicationTrafficRecognition.Applications.Ipv4ApplicationsItemBase.Protocols", schema = List(44)))]
            pub struct Protocols<'a, Mode> (::data_views::Field<&'a str>);

            #[::data_views::data_view(list, python(name = "EOSConfig.ApplicationTrafficRecognition.Applications.Ipv4ApplicationsItemBase.ProtocolRanges", schema = List(45)))]
            pub struct ProtocolRanges<'a, Mode> (::data_views::Field<&'a str>);
        }
    }

    #[::data_views::data_view(indexed_list, primary_key(name), python(name = "EOSConfig.ApplicationTrafficRecognition.Applications.L4Applications", schema = List(47), item_name = "EOSConfig.ApplicationTrafficRecognition.Applications.L4ApplicationsItem"))]
    pub struct L4Applications<'a, Mode> (::data_views::Field<l4_applications::Item<'a, Mode>>);

    pub mod l4_applications {

        #[::data_views::data_view(python(name = "EOSConfig.ApplicationTrafficRecognition.Applications.L4ApplicationsItemBase", schema = Dict(72)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(44))]
            pub name: ::data_views::Field<&'a str>,
            #[data_view(schema = List(44))]
            pub protocols: ::data_views::Field<item::Protocols<'a, Mode>>,
            #[data_view(schema = List(45))]
            pub protocol_ranges: ::data_views::Field<item::ProtocolRanges<'a, Mode>>,
            #[data_view(schema = Str(50))]
            pub udp_src_port_set_name: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(51))]
            pub tcp_src_port_set_name: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(52))]
            pub udp_dest_port_set_name: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(53))]
            pub tcp_dest_port_set_name: ::data_views::Field<&'a str>,
        }

        pub mod item {

            #[::data_views::data_view(list, python(name = "EOSConfig.ApplicationTrafficRecognition.Applications.L4ApplicationsItemBase.Protocols", schema = List(44)))]
            pub struct Protocols<'a, Mode> (::data_views::Field<&'a str>);

            #[::data_views::data_view(list, python(name = "EOSConfig.ApplicationTrafficRecognition.Applications.L4ApplicationsItemBase.ProtocolRanges", schema = List(45)))]
            pub struct ProtocolRanges<'a, Mode> (::data_views::Field<&'a str>);
        }
    }
}

#[::data_views::data_view(indexed_list, primary_key(name), python(name = "EOSConfig.ApplicationTrafficRecognition.ApplicationProfiles", schema = List(51), item_name = "EOSConfig.ApplicationTrafficRecognition.ApplicationProfilesItem"))]
pub struct ApplicationProfiles<'a, Mode> (::data_views::Field<application_profiles::Item<'a, Mode>>);

pub mod application_profiles {

    #[::data_views::data_view(python(name = "EOSConfig.ApplicationTrafficRecognition.ApplicationProfilesItemBase", schema = Dict(76)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(54))]
        pub name: ::data_views::Field<&'a str>,
        #[data_view(schema = List(48))]
        pub applications: ::data_views::Field<item::Applications<'a, Mode>>,
        #[data_view(schema = List(49))]
        pub application_transports: ::data_views::Field<item::ApplicationTransports<'a, Mode>>,
        #[data_view(schema = List(50))]
        pub categories: ::data_views::Field<item::Categories<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view(list, python(name = "EOSConfig.ApplicationTrafficRecognition.ApplicationProfilesItemBase.Applications", schema = List(48)))]
        pub struct Applications<'a, Mode> (::data_views::Field<applications::Item<'a, Mode>>);

        pub mod applications {

            #[::data_views::data_view(python(name = "EOSConfig.ApplicationTrafficRecognition.ApplicationProfilesItemBase.ApplicationsItem", schema = Dict(74)))]
            pub struct Item<'a, Mode> {
                #[data_view(schema = Str(55))]
                pub name: ::data_views::RequiredValue<&'a str, Mode>,
                #[data_view(schema = Str(39))]
                pub service: ::data_views::Field<&'a str>,
            }
        }

        #[::data_views::data_view(list, python(name = "EOSConfig.ApplicationTrafficRecognition.ApplicationProfilesItemBase.ApplicationTransports", schema = List(49)))]
        pub struct ApplicationTransports<'a, Mode> (::data_views::Field<&'a str>);

        #[::data_views::data_view(list, python(name = "EOSConfig.ApplicationTrafficRecognition.ApplicationProfilesItemBase.Categories", schema = List(50)))]
        pub struct Categories<'a, Mode> (::data_views::Field<categories::Item<'a, Mode>>);

        pub mod categories {

            #[::data_views::data_view(python(name = "EOSConfig.ApplicationTrafficRecognition.ApplicationProfilesItemBase.CategoriesItem", schema = Dict(75)))]
            pub struct Item<'a, Mode> {
                #[data_view(schema = Str(57))]
                pub name: ::data_views::RequiredValue<&'a str, Mode>,
                #[data_view(schema = Str(39))]
                pub service: ::data_views::Field<&'a str>,
            }
        }
    }
}
