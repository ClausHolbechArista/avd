// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.CvPathfinderInternetExitPolicies", schema = List(594), item_name = "AVDDesign.CvPathfinderInternetExitPoliciesItem"))]
pub struct CvPathfinderInternetExitPolicies<'a, Mode> (::validated_data::Field<Item<'a, Mode>>);

#[::validated_data::data_view(python(name = "AVDDesign.CvPathfinderInternetExitPoliciesItemBase", schema = Dict(1450)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(1199))]
    pub name: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1200), rename = "type")]
    pub field_type: ::validated_data::RequiredValue<&'a str, Mode>,
    #[data_view(schema = Bool(448))]
    pub fallback_to_system_default: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(1449))]
    pub zscaler: ::validated_data::Field<item::Zscaler<'a, Mode>>,
}

pub mod item {

    #[::validated_data::data_view(python(name = "AVDDesign.CvPathfinderInternetExitPoliciesItemBase.Zscaler", schema = Dict(1449)))]
    pub struct Zscaler<'a, Mode> {
        #[data_view(schema = Str(1201))]
        pub ipsec_key_salt: ::validated_data::RequiredValue<&'a str, Mode>,
        #[data_view(schema = Str(1202))]
        pub domain_name: ::validated_data::RequiredValue<&'a str, Mode>,
        #[data_view(schema = Bool(449))]
        pub encrypt_traffic: ::validated_data::Field<bool>,
        #[data_view(schema = Int(523))]
        pub download_bandwidth: ::validated_data::Field<i64>,
        #[data_view(schema = Int(524))]
        pub upload_bandwidth: ::validated_data::Field<i64>,
        #[data_view(schema = Dict(1448))]
        pub firewall: ::validated_data::Field<zscaler::Firewall<'a, Mode>>,
        #[data_view(schema = Bool(452))]
        pub acceptable_use_policy: ::validated_data::Field<bool>,
    }

    pub mod zscaler {

        #[::validated_data::data_view(python(name = "AVDDesign.CvPathfinderInternetExitPoliciesItemBase.Zscaler.Firewall", schema = Dict(1448)))]
        pub struct Firewall<'a, Mode> {
            #[data_view(schema = Bool(450))]
            pub enabled: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(451))]
            pub ips: ::validated_data::Field<bool>,
        }
    }
}
