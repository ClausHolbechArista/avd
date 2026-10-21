// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "AVDDesign.Dot1xSettings", schema = Dict(1542)))]
pub struct Dot1xSettings<'a, Mode> {
    #[data_view(schema = Bool(493))]
    pub enabled: ::data_views::Field<bool>,
    #[data_view(schema = Dict(1527))]
    pub authentication: ::data_views::Field<Authentication<'a, Mode>>,
    #[data_view(schema = Dict(1528))]
    pub accounting: ::data_views::Field<Accounting<'a, Mode>>,
    #[data_view(schema = Bool(497))]
    pub bypass_bpdu: ::data_views::Field<bool>,
    #[data_view(schema = Bool(498))]
    pub bypass_lldp: ::data_views::Field<bool>,
    #[data_view(schema = Dict(1529))]
    pub dynamic_authorization: ::data_views::Field<DynamicAuthorization<'a, Mode>>,
    #[data_view(schema = Dict(1531))]
    pub mac_based_authentication: ::data_views::Field<MacBasedAuthentication<'a, Mode>>,
    #[data_view(schema = Dict(1532))]
    pub radius_av_pairs: ::data_views::Field<RadiusAvPairs<'a, Mode>>,
    #[data_view(schema = Dict(1540))]
    pub device_profiling: ::data_views::Field<DeviceProfiling<'a, Mode>>,
    #[data_view(schema = Bool(506))]
    pub redistribute_in_evpn: ::data_views::Field<bool>,
    #[data_view(schema = Dict(1541))]
    pub web_authentication: ::data_views::Field<WebAuthentication<'a, Mode>>,
}

#[::data_views::data_view(python(name = "AVDDesign.Dot1xSettings.Authentication", schema = Dict(1527)))]
pub struct Authentication<'a, Mode> {
    #[data_view(schema = List(647))]
    pub radius_groups: ::data_views::Field<authentication::RadiusGroups<'a, Mode>>,
}

pub mod authentication {

    #[::data_views::data_view(list, python(name = "AVDDesign.Dot1xSettings.Authentication.RadiusGroups", schema = List(647)))]
    pub struct RadiusGroups<'a, Mode> (::data_views::Field<&'a str>);
}

#[::data_views::data_view(python(name = "AVDDesign.Dot1xSettings.Accounting", schema = Dict(1528)))]
pub struct Accounting<'a, Mode> {
    #[data_view(schema = Bool(494))]
    pub enabled: ::data_views::Field<bool>,
    #[data_view(schema = Str(1385))]
    pub mode: ::data_views::Field<&'a str>,
    #[data_view(schema = List(648))]
    pub radius_groups: ::data_views::Field<accounting::RadiusGroups<'a, Mode>>,
    #[data_view(schema = Bool(495))]
    pub multicast: ::data_views::Field<bool>,
    #[data_view(schema = Bool(496))]
    pub syslog: ::data_views::Field<bool>,
}

pub mod accounting {

    #[::data_views::data_view(list, python(name = "AVDDesign.Dot1xSettings.Accounting.RadiusGroups", schema = List(648)))]
    pub struct RadiusGroups<'a, Mode> (::data_views::Field<&'a str>);
}

#[::data_views::data_view(python(name = "AVDDesign.Dot1xSettings.DynamicAuthorization", schema = Dict(1529)))]
pub struct DynamicAuthorization<'a, Mode> {
    #[data_view(schema = Bool(499))]
    pub enabled: ::data_views::Field<bool>,
    #[data_view(schema = List(649))]
    pub additional_groups: ::data_views::Field<dynamic_authorization::AdditionalGroups<'a, Mode>>,
}

pub mod dynamic_authorization {

    #[::data_views::data_view(list, python(name = "AVDDesign.Dot1xSettings.DynamicAuthorization.AdditionalGroups", schema = List(649)))]
    pub struct AdditionalGroups<'a, Mode> (::data_views::Field<&'a str>);
}

#[::data_views::data_view(python(name = "AVDDesign.Dot1xSettings.MacBasedAuthentication", schema = Dict(1531)))]
pub struct MacBasedAuthentication<'a, Mode> {
    #[data_view(schema = Dict(1530))]
    pub username_format: ::data_views::Field<mac_based_authentication::UsernameFormat<'a, Mode>>,
    #[data_view(schema = Int(23))]
    pub delay: ::data_views::Field<i64>,
    #[data_view(schema = Int(24))]
    pub hold_period: ::data_views::Field<i64>,
}

pub mod mac_based_authentication {

    #[::data_views::data_view(python(name = "AVDDesign.Dot1xSettings.MacBasedAuthentication.UsernameFormat", schema = Dict(1530)))]
    pub struct UsernameFormat<'a, Mode> {
        #[data_view(schema = Str(1386))]
        pub delimiter: ::data_views::RequiredValue<&'a str, Mode>,
        #[data_view(schema = Str(1387))]
        pub letter_case: ::data_views::RequiredValue<&'a str, Mode>,
    }
}

#[::data_views::data_view(python(name = "AVDDesign.Dot1xSettings.RadiusAvPairs", schema = Dict(1532)))]
pub struct RadiusAvPairs<'a, Mode> {
    #[data_view(schema = Bool(500))]
    pub service_type: ::data_views::Field<bool>,
    #[data_view(schema = Int(25))]
    pub framed_mtu: ::data_views::Field<i64>,
}

#[::data_views::data_view(python(name = "AVDDesign.Dot1xSettings.DeviceProfiling", schema = Dict(1540)))]
pub struct DeviceProfiling<'a, Mode> {
    #[data_view(schema = Bool(501))]
    pub enabled: ::data_views::Field<bool>,
    #[data_view(schema = Dict(1536))]
    pub dhcp: ::data_views::Field<device_profiling::Dhcp<'a, Mode>>,
    #[data_view(schema = Dict(1539))]
    pub lldp: ::data_views::Field<device_profiling::Lldp<'a, Mode>>,
}

pub mod device_profiling {

    #[::data_views::data_view(python(name = "AVDDesign.Dot1xSettings.DeviceProfiling.Dhcp", schema = Dict(1536)))]
    pub struct Dhcp<'a, Mode> {
        #[data_view(schema = Bool(502))]
        pub enabled: ::data_views::Field<bool>,
        #[data_view(schema = Dict(1533))]
        pub hostname: ::data_views::Field<dhcp::Hostname<'a, Mode>>,
        #[data_view(schema = Dict(1534))]
        pub parameter_request_list: ::data_views::Field<dhcp::ParameterRequestList<'a, Mode>>,
        #[data_view(schema = Dict(1535))]
        pub vendor_class_id: ::data_views::Field<dhcp::VendorClassId<'a, Mode>>,
    }

    pub mod dhcp {

        #[::data_views::data_view(python(name = "AVDDesign.Dot1xSettings.DeviceProfiling.Dhcp.Hostname", schema = Dict(1533)))]
        pub struct Hostname<'a, Mode> {
            #[data_view(schema = Bool(37))]
            pub enabled: ::data_views::Field<bool>,
            #[data_view(schema = Bool(503))]
            pub auth_only: ::data_views::Field<bool>,
        }

        #[::data_views::data_view(python(name = "AVDDesign.Dot1xSettings.DeviceProfiling.Dhcp.ParameterRequestList", schema = Dict(1534)))]
        pub struct ParameterRequestList<'a, Mode> {
            #[data_view(schema = Bool(37))]
            pub enabled: ::data_views::Field<bool>,
            #[data_view(schema = Bool(503))]
            pub auth_only: ::data_views::Field<bool>,
        }

        #[::data_views::data_view(python(name = "AVDDesign.Dot1xSettings.DeviceProfiling.Dhcp.VendorClassId", schema = Dict(1535)))]
        pub struct VendorClassId<'a, Mode> {
            #[data_view(schema = Bool(37))]
            pub enabled: ::data_views::Field<bool>,
            #[data_view(schema = Bool(503))]
            pub auth_only: ::data_views::Field<bool>,
        }
    }

    #[::data_views::data_view(python(name = "AVDDesign.Dot1xSettings.DeviceProfiling.Lldp", schema = Dict(1539)))]
    pub struct Lldp<'a, Mode> {
        #[data_view(schema = Bool(504))]
        pub enabled: ::data_views::Field<bool>,
        #[data_view(schema = Dict(1537))]
        pub system_name: ::data_views::Field<lldp::SystemName<'a, Mode>>,
        #[data_view(schema = Dict(1538))]
        pub system_description: ::data_views::Field<lldp::SystemDescription<'a, Mode>>,
    }

    pub mod lldp {

        #[::data_views::data_view(python(name = "AVDDesign.Dot1xSettings.DeviceProfiling.Lldp.SystemName", schema = Dict(1537)))]
        pub struct SystemName<'a, Mode> {
            #[data_view(schema = Bool(37))]
            pub enabled: ::data_views::Field<bool>,
            #[data_view(schema = Bool(505))]
            pub auth_only: ::data_views::Field<bool>,
        }

        #[::data_views::data_view(python(name = "AVDDesign.Dot1xSettings.DeviceProfiling.Lldp.SystemDescription", schema = Dict(1538)))]
        pub struct SystemDescription<'a, Mode> {
            #[data_view(schema = Bool(37))]
            pub enabled: ::data_views::Field<bool>,
            #[data_view(schema = Bool(505))]
            pub auth_only: ::data_views::Field<bool>,
        }
    }
}

#[::data_views::data_view(python(name = "AVDDesign.Dot1xSettings.WebAuthentication", schema = Dict(1541)))]
pub struct WebAuthentication<'a, Mode> {
    #[data_view(schema = Bool(507))]
    pub enabled: ::data_views::RequiredValue<bool, Mode>,
    #[data_view(schema = Str(1388))]
    pub ipv4_acl: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1389))]
    pub url: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1390))]
    pub ssl_profile: ::data_views::Field<&'a str>,
    #[data_view(schema = Bool(508))]
    pub start_limit_infinite: ::data_views::Field<bool>,
}
