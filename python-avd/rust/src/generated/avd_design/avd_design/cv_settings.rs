// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "AVDDesign.CvSettings", schema = Dict(1458)))]
pub struct CvSettings<'a, Mode> {
    #[data_view(schema = Dict(1454))]
    pub cvaas: ::data_views::Field<Cvaas<'a, Mode>>,
    #[data_view(schema = List(599))]
    pub onprem_clusters: ::data_views::Field<OnpremClusters<'a, Mode>>,
    #[data_view(schema = Dict(1457))]
    pub terminattr: ::data_views::Field<Terminattr<'a, Mode>>,
    #[data_view(schema = Bool(455))]
    pub set_source_interfaces: ::data_views::Field<bool>,
}

#[::data_views::data_view(python(name = "AVDDesign.CvSettings.Cvaas", schema = Dict(1454)))]
pub struct Cvaas<'a, Mode> {
    #[data_view(schema = Bool(453))]
    pub enabled: ::data_views::RequiredValue<bool, Mode>,
    #[data_view(schema = List(597))]
    pub clusters: ::data_views::Field<cvaas::Clusters<'a, Mode>>,
}

pub mod cvaas {

    #[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.CvSettings.Cvaas.Clusters", schema = List(597), item_name = "AVDDesign.CvSettings.Cvaas.ClustersItem"))]
    pub struct Clusters<'a, Mode> (::data_views::Field<clusters::Item<'a, Mode>>);

    pub mod clusters {

        #[::data_views::data_view(python(name = "AVDDesign.CvSettings.Cvaas.ClustersItemBase", schema = Dict(1453)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(1206))]
            pub name: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(1207))]
            pub region: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(1208))]
            pub vrf: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(1209))]
            pub token_file: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(1210))]
            pub source_interface: ::data_views::Field<&'a str>,
        }
    }
}

#[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.CvSettings.OnpremClusters", schema = List(599), item_name = "AVDDesign.CvSettings.OnpremClustersItem"))]
pub struct OnpremClusters<'a, Mode> (::data_views::Field<onprem_clusters::Item<'a, Mode>>);

pub mod onprem_clusters {

    #[::data_views::data_view(python(name = "AVDDesign.CvSettings.OnpremClustersItemBase", schema = Dict(1456)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(1211))]
        pub name: ::data_views::Field<&'a str>,
        #[data_view(schema = List(598))]
        pub servers: ::data_views::RequiredValue<item::Servers<'a, Mode>, Mode>,
        #[data_view(schema = Str(1208))]
        pub vrf: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1213))]
        pub token_file: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1210))]
        pub source_interface: ::data_views::Field<&'a str>,
    }

    pub mod item {

        #[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.CvSettings.OnpremClustersItemBase.Servers", schema = List(598), item_name = "AVDDesign.CvSettings.OnpremClustersItemBase.ServersItem"))]
        pub struct Servers<'a, Mode> (::data_views::Field<servers::Item<'a, Mode>>);

        pub mod servers {

            #[::data_views::data_view(python(name = "AVDDesign.CvSettings.OnpremClustersItemBase.ServersItemBase", schema = Dict(1455)))]
            pub struct Item<'a, Mode> {
                #[data_view(schema = Str(1212))]
                pub name: ::data_views::Field<&'a str>,
                #[data_view(schema = Int(527))]
                pub port: ::data_views::Field<i64>,
            }
        }
    }
}

#[::data_views::data_view(python(name = "AVDDesign.CvSettings.Terminattr", schema = Dict(1457)))]
pub struct Terminattr<'a, Mode> {
    #[data_view(schema = Str(121))]
    pub ingestexclude: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1214))]
    pub smashexcludes: ::data_views::Field<&'a str>,
    #[data_view(schema = Bool(454))]
    pub disable_aaa: ::data_views::Field<bool>,
    #[data_view(schema = List(72))]
    pub cvtargetconfigs: ::data_views::Field<terminattr::Cvtargetconfigs<'a, Mode>>,
    #[data_view(schema = Bool(36))]
    pub flowdns: ::data_views::Field<bool>,
    #[data_view(schema = List(73))]
    pub custom_cv_options: ::data_views::Field<terminattr::CustomCvOptions<'a, Mode>>,
}

pub mod terminattr {

    #[::data_views::data_view(list, python(name = "AVDDesign.CvSettings.Terminattr.Cvtargetconfigs", schema = List(72)))]
    pub struct Cvtargetconfigs<'a, Mode> (::data_views::Field<&'a str>);

    #[::data_views::data_view(list, python(name = "AVDDesign.CvSettings.Terminattr.CustomCvOptions", schema = List(73)))]
    pub struct CustomCvOptions<'a, Mode> (::data_views::Field<custom_cv_options::Item<'a, Mode>>);

    pub mod custom_cv_options {

        #[::data_views::data_view(python(name = "AVDDesign.CvSettings.Terminattr.CustomCvOptionsItem", schema = Dict(123)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(128))]
            pub flag: ::data_views::RequiredValue<&'a str, Mode>,
            #[data_view(schema = Str(129))]
            pub value: ::data_views::Field<&'a str>,
        }
    }
}
