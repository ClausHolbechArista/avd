// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.CvPathfinderInternetExitPolicies", schema = List(594), item_name = "AVDDesign.CvPathfinderInternetExitPoliciesItem"))]
pub struct CvPathfinderInternetExitPolicies<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view(python(name = "AVDDesign.CvPathfinderInternetExitPoliciesItemBase", schema = Dict(1450)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(1199))]
    pub name: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1200), rename = "type")]
    pub field_type: ::data_views::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Bool(448))]
    pub fallback_to_system_default: ::data_views::Field<bool>,
    #[data_view(schema = Dict(1449))]
    pub zscaler: ::data_views::Field<item::Zscaler<'a, Mode>>,
}

pub mod item {

    #[::data_views::data_view(python(name = "AVDDesign.CvPathfinderInternetExitPoliciesItemBase.Zscaler", schema = Dict(1449)))]
    pub struct Zscaler<'a, Mode> {
        #[data_view(schema = Str(1201))]
        pub ipsec_key_salt: ::data_views::RequiredValue<&'a str, Mode>,
        #[data_view(schema = Str(1202))]
        pub domain_name: ::data_views::RequiredValue<&'a str, Mode>,
        #[data_view(schema = Bool(449))]
        pub encrypt_traffic: ::data_views::Field<bool>,
        #[data_view(schema = Int(523))]
        pub download_bandwidth: ::data_views::Field<i64>,
        #[data_view(schema = Int(524))]
        pub upload_bandwidth: ::data_views::Field<i64>,
        #[data_view(schema = Dict(1448))]
        pub firewall: ::data_views::Field<zscaler::Firewall<'a, Mode>>,
        #[data_view(schema = Bool(452))]
        pub acceptable_use_policy: ::data_views::Field<bool>,
    }

    pub mod zscaler {

        #[::data_views::data_view(python(name = "AVDDesign.CvPathfinderInternetExitPoliciesItemBase.Zscaler.Firewall", schema = Dict(1448)))]
        pub struct Firewall<'a, Mode> {
            #[data_view(schema = Bool(450))]
            pub enabled: ::data_views::Field<bool>,
            #[data_view(schema = Bool(451))]
            pub ips: ::data_views::Field<bool>,
        }
    }
}
