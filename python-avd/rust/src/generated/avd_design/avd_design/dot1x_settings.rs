// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(python(name = "AVDDesign.Dot1xSettings", schema = Dict(1542)))]
pub struct Dot1xSettings<'a, Mode> {
    #[data_view(schema = Bool(493))]
    pub enabled: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(1527))]
    pub authentication: ::validated_data::Field<Authentication<'a, Mode>>,
    #[data_view(schema = Dict(1528))]
    pub accounting: ::validated_data::Field<Accounting<'a, Mode>>,
    #[data_view(schema = Bool(497))]
    pub bypass_bpdu: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(498))]
    pub bypass_lldp: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(1529))]
    pub dynamic_authorization: ::validated_data::Field<DynamicAuthorization<'a, Mode>>,
    #[data_view(schema = Dict(1531))]
    pub mac_based_authentication: ::validated_data::Field<MacBasedAuthentication<'a, Mode>>,
    #[data_view(schema = Dict(1532))]
    pub radius_av_pairs: ::validated_data::Field<RadiusAvPairs<'a, Mode>>,
    #[data_view(schema = Dict(1540))]
    pub device_profiling: ::validated_data::Field<DeviceProfiling<'a, Mode>>,
    #[data_view(schema = Bool(506))]
    pub redistribute_in_evpn: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(1541))]
    pub web_authentication: ::validated_data::Field<WebAuthentication<'a, Mode>>,
}

#[::validated_data::data_view(python(name = "AVDDesign.Dot1xSettings.Authentication", schema = Dict(1527)))]
pub struct Authentication<'a, Mode> {
    #[data_view(schema = List(647))]
    pub radius_groups: ::validated_data::Field<authentication::RadiusGroups<'a, Mode>>,
}

pub mod authentication {

    #[::validated_data::data_view(list, python(name = "AVDDesign.Dot1xSettings.Authentication.RadiusGroups", schema = List(647)))]
    pub struct RadiusGroups<'a, Mode> (::validated_data::Field<&'a str>);
}

#[::validated_data::data_view(python(name = "AVDDesign.Dot1xSettings.Accounting", schema = Dict(1528)))]
pub struct Accounting<'a, Mode> {
    #[data_view(schema = Bool(494))]
    pub enabled: ::validated_data::Field<bool>,
    #[data_view(schema = Str(1385))]
    pub mode: ::validated_data::Field<&'a str>,
    #[data_view(schema = List(648))]
    pub radius_groups: ::validated_data::Field<accounting::RadiusGroups<'a, Mode>>,
    #[data_view(schema = Bool(495))]
    pub multicast: ::validated_data::Field<bool>,
    #[data_view(schema = Bool(496))]
    pub syslog: ::validated_data::Field<bool>,
}

pub mod accounting {

    #[::validated_data::data_view(list, python(name = "AVDDesign.Dot1xSettings.Accounting.RadiusGroups", schema = List(648)))]
    pub struct RadiusGroups<'a, Mode> (::validated_data::Field<&'a str>);
}

#[::validated_data::data_view(python(name = "AVDDesign.Dot1xSettings.DynamicAuthorization", schema = Dict(1529)))]
pub struct DynamicAuthorization<'a, Mode> {
    #[data_view(schema = Bool(499))]
    pub enabled: ::validated_data::Field<bool>,
    #[data_view(schema = List(649))]
    pub additional_groups: ::validated_data::Field<dynamic_authorization::AdditionalGroups<'a, Mode>>,
}

pub mod dynamic_authorization {

    #[::validated_data::data_view(list, python(name = "AVDDesign.Dot1xSettings.DynamicAuthorization.AdditionalGroups", schema = List(649)))]
    pub struct AdditionalGroups<'a, Mode> (::validated_data::Field<&'a str>);
}

#[::validated_data::data_view(python(name = "AVDDesign.Dot1xSettings.MacBasedAuthentication", schema = Dict(1531)))]
pub struct MacBasedAuthentication<'a, Mode> {
    #[data_view(schema = Dict(1530))]
    pub username_format: ::validated_data::Field<mac_based_authentication::UsernameFormat<'a, Mode>>,
    #[data_view(schema = Int(23))]
    pub delay: ::validated_data::Field<i64>,
    #[data_view(schema = Int(24))]
    pub hold_period: ::validated_data::Field<i64>,
}

pub mod mac_based_authentication {

    #[::validated_data::data_view(python(name = "AVDDesign.Dot1xSettings.MacBasedAuthentication.UsernameFormat", schema = Dict(1530)))]
    pub struct UsernameFormat<'a, Mode> {
        #[data_view(schema = Str(1386))]
        pub delimiter: ::validated_data::RequiredValue<&'a str, Mode>,
        #[data_view(schema = Str(1387))]
        pub letter_case: ::validated_data::RequiredValue<&'a str, Mode>,
    }
}

#[::validated_data::data_view(python(name = "AVDDesign.Dot1xSettings.RadiusAvPairs", schema = Dict(1532)))]
pub struct RadiusAvPairs<'a, Mode> {
    #[data_view(schema = Bool(500))]
    pub service_type: ::validated_data::Field<bool>,
    #[data_view(schema = Int(25))]
    pub framed_mtu: ::validated_data::Field<i64>,
}

#[::validated_data::data_view(python(name = "AVDDesign.Dot1xSettings.DeviceProfiling", schema = Dict(1540)))]
pub struct DeviceProfiling<'a, Mode> {
    #[data_view(schema = Bool(501))]
    pub enabled: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(1536))]
    pub dhcp: ::validated_data::Field<device_profiling::Dhcp<'a, Mode>>,
    #[data_view(schema = Dict(1539))]
    pub lldp: ::validated_data::Field<device_profiling::Lldp<'a, Mode>>,
}

pub mod device_profiling {

    #[::validated_data::data_view(python(name = "AVDDesign.Dot1xSettings.DeviceProfiling.Dhcp", schema = Dict(1536)))]
    pub struct Dhcp<'a, Mode> {
        #[data_view(schema = Bool(502))]
        pub enabled: ::validated_data::Field<bool>,
        #[data_view(schema = Dict(1533))]
        pub hostname: ::validated_data::Field<dhcp::Hostname<'a, Mode>>,
        #[data_view(schema = Dict(1534))]
        pub parameter_request_list: ::validated_data::Field<dhcp::ParameterRequestList<'a, Mode>>,
        #[data_view(schema = Dict(1535))]
        pub vendor_class_id: ::validated_data::Field<dhcp::VendorClassId<'a, Mode>>,
    }

    pub mod dhcp {

        #[::validated_data::data_view(python(name = "AVDDesign.Dot1xSettings.DeviceProfiling.Dhcp.Hostname", schema = Dict(1533)))]
        pub struct Hostname<'a, Mode> {
            #[data_view(schema = Bool(37))]
            pub enabled: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(503))]
            pub auth_only: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view(python(name = "AVDDesign.Dot1xSettings.DeviceProfiling.Dhcp.ParameterRequestList", schema = Dict(1534)))]
        pub struct ParameterRequestList<'a, Mode> {
            #[data_view(schema = Bool(37))]
            pub enabled: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(503))]
            pub auth_only: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view(python(name = "AVDDesign.Dot1xSettings.DeviceProfiling.Dhcp.VendorClassId", schema = Dict(1535)))]
        pub struct VendorClassId<'a, Mode> {
            #[data_view(schema = Bool(37))]
            pub enabled: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(503))]
            pub auth_only: ::validated_data::Field<bool>,
        }
    }

    #[::validated_data::data_view(python(name = "AVDDesign.Dot1xSettings.DeviceProfiling.Lldp", schema = Dict(1539)))]
    pub struct Lldp<'a, Mode> {
        #[data_view(schema = Bool(504))]
        pub enabled: ::validated_data::Field<bool>,
        #[data_view(schema = Dict(1537))]
        pub system_name: ::validated_data::Field<lldp::SystemName<'a, Mode>>,
        #[data_view(schema = Dict(1538))]
        pub system_description: ::validated_data::Field<lldp::SystemDescription<'a, Mode>>,
    }

    pub mod lldp {

        #[::validated_data::data_view(python(name = "AVDDesign.Dot1xSettings.DeviceProfiling.Lldp.SystemName", schema = Dict(1537)))]
        pub struct SystemName<'a, Mode> {
            #[data_view(schema = Bool(37))]
            pub enabled: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(505))]
            pub auth_only: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view(python(name = "AVDDesign.Dot1xSettings.DeviceProfiling.Lldp.SystemDescription", schema = Dict(1538)))]
        pub struct SystemDescription<'a, Mode> {
            #[data_view(schema = Bool(37))]
            pub enabled: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(505))]
            pub auth_only: ::validated_data::Field<bool>,
        }
    }
}

#[::validated_data::data_view(python(name = "AVDDesign.Dot1xSettings.WebAuthentication", schema = Dict(1541)))]
pub struct WebAuthentication<'a, Mode> {
    #[data_view(schema = Bool(507))]
    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
    #[data_view(schema = Str(1388))]
    pub ipv4_acl: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1389))]
    pub url: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1390))]
    pub ssl_profile: ::validated_data::Field<&'a str>,
    #[data_view(schema = Bool(508))]
    pub start_limit_infinite: ::validated_data::Field<bool>,
}
