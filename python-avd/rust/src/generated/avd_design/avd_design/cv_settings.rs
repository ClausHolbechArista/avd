// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(python(name = "AVDDesign.CvSettings.Cvaas", schema = Dict(1454)))]
pub struct Cvaas<'a, Mode> {
    #[data_view(schema = Bool(453))]
    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
    #[data_view(schema = List(597))]
    pub clusters: ::validated_data::Field<cvaas::Clusters<'a, Mode>>,
}

pub mod cvaas {

    #[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.CvSettings.Cvaas.Clusters", schema = List(597), item_name = "AVDDesign.CvSettings.Cvaas.ClustersItem"))]
    pub struct Clusters<'a, Mode> (::validated_data::Field<clusters::Item<'a, Mode>>);

    pub mod clusters {

        #[::validated_data::data_view(python(name = "AVDDesign.CvSettings.Cvaas.ClustersItemBase", schema = Dict(1453)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(1206))]
            pub name: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1207))]
            pub region: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1208))]
            pub vrf: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1209))]
            pub token_file: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1210))]
            pub source_interface: ::validated_data::Field<&'a str>,
        }
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.CvSettings.OnpremClusters", schema = List(599), item_name = "AVDDesign.CvSettings.OnpremClustersItem"))]
pub struct OnpremClusters<'a, Mode> (::validated_data::Field<onprem_clusters::Item<'a, Mode>>);

pub mod onprem_clusters {

    #[::validated_data::data_view(python(name = "AVDDesign.CvSettings.OnpremClustersItemBase", schema = Dict(1456)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(1211))]
        pub name: ::validated_data::Field<&'a str>,
        #[data_view(schema = List(598))]
        pub servers: ::validated_data::RequiredValue<item::Servers<'a, Mode>, Mode>,
        #[data_view(schema = Str(1208))]
        pub vrf: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1213))]
        pub token_file: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1210))]
        pub source_interface: ::validated_data::Field<&'a str>,
    }

    pub mod item {

        #[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.CvSettings.OnpremClustersItemBase.Servers", schema = List(598), item_name = "AVDDesign.CvSettings.OnpremClustersItemBase.ServersItem"))]
        pub struct Servers<'a, Mode> (::validated_data::Field<servers::Item<'a, Mode>>);

        pub mod servers {

            #[::validated_data::data_view(python(name = "AVDDesign.CvSettings.OnpremClustersItemBase.ServersItemBase", schema = Dict(1455)))]
            pub struct Item<'a, Mode> {
                #[data_view(schema = Str(1212))]
                pub name: ::validated_data::Field<&'a str>,
                #[data_view(schema = Int(527))]
                pub port: ::validated_data::Field<i64>,
            }
        }
    }
}

#[::validated_data::data_view(python(name = "AVDDesign.CvSettings.Terminattr", schema = Dict(1457)))]
pub struct Terminattr<'a, Mode> {
    #[data_view(schema = Str(121))]
    pub ingestexclude: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1214))]
    pub smashexcludes: ::validated_data::Field<&'a str>,
    #[data_view(schema = Bool(454))]
    pub disable_aaa: ::validated_data::Field<bool>,
    #[data_view(schema = List(72))]
    pub cvtargetconfigs: ::validated_data::Field<terminattr::Cvtargetconfigs<'a, Mode>>,
    #[data_view(schema = Bool(36))]
    pub flowdns: ::validated_data::Field<bool>,
    #[data_view(schema = List(73))]
    pub custom_cv_options: ::validated_data::Field<terminattr::CustomCvOptions<'a, Mode>>,
}

pub mod terminattr {

    #[::validated_data::data_view(list, python(name = "AVDDesign.CvSettings.Terminattr.Cvtargetconfigs", schema = List(72)))]
    pub struct Cvtargetconfigs<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(list, python(name = "AVDDesign.CvSettings.Terminattr.CustomCvOptions", schema = List(73)))]
    pub struct CustomCvOptions<'a, Mode> (::validated_data::Field<custom_cv_options::Item<'a, Mode>>);

    pub mod custom_cv_options {

        #[::validated_data::data_view(python(name = "AVDDesign.CvSettings.Terminattr.CustomCvOptionsItem", schema = Dict(123)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(128))]
            pub flag: ::validated_data::RequiredValue<&'a str, Mode>,
            #[data_view(schema = Str(129))]
            pub value: ::validated_data::Field<&'a str>,
        }
    }
}
