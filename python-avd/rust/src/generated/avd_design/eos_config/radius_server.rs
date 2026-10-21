// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct RadiusServer<'a, Mode> {
    pub attribute_32_include_in_access_req: ::data_views::Field<Attribute32IncludeInAccessReq<'a, Mode>>,
    pub deadtime: ::data_views::Field<i64>,
    pub dynamic_authorization: ::data_views::Field<DynamicAuthorization<'a, Mode>>,
    pub servers: ::data_views::Field<Servers<'a, Mode>>,
    pub vrfs: ::data_views::Field<Vrfs<'a, Mode>>,
    pub tls_ssl_profile: ::data_views::Field<&'a str>,
}

#[::data_views::data_view]
pub struct Attribute32IncludeInAccessReq<'a, Mode> {
    pub hostname: ::data_views::Field<bool>,
    pub format: ::data_views::Field<&'a str>,
}

#[::data_views::data_view]
pub struct DynamicAuthorization<'a, Mode> {
    pub port: ::data_views::Field<i64>,
    pub tls_ssl_profile: ::data_views::Field<&'a str>,
}

#[::data_views::data_view(list)]
pub struct Servers<'a, Mode> (::data_views::Field<servers::Item<'a, Mode>>);

pub mod servers {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub host: ::data_views::RequiredValue<&'a str, Mode>,
        pub tls: ::data_views::Field<item::Tls<'a, Mode>>,
        pub timeout: ::data_views::Field<i64>,
        pub retransmit: ::data_views::Field<i64>,
        pub key: ::data_views::Field<&'a str>,
        pub key_type: ::data_views::Field<&'a str>,
    }

    pub mod item {

        #[::data_views::data_view(python(name = "EOSConfig.RadiusServer.ServersItem.Tls", schema = Dict(863)))]
        pub struct Tls<'a, Mode> {
            #[data_view(schema = Bool(306))]
            pub enabled: ::data_views::Field<bool>,
            #[data_view(schema = Str(731))]
            pub ssl_profile: ::data_views::Field<&'a str>,
            #[data_view(schema = Int(351))]
            pub port: ::data_views::Field<i64>,
        }
    }
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Vrfs<'a, Mode> (::data_views::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub servers: ::data_views::RequiredValue<item::Servers<'a, Mode>, Mode>,
    }

    pub mod item {

        #[::data_views::data_view(list)]
        pub struct Servers<'a, Mode> (::data_views::Field<servers::Item<'a, Mode>>);

        pub mod servers {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub host: ::data_views::RequiredValue<&'a str, Mode>,
                pub tls: ::data_views::Field<item::Tls<'a, Mode>>,
                pub timeout: ::data_views::Field<i64>,
                pub retransmit: ::data_views::Field<i64>,
                pub key: ::data_views::Field<&'a str>,
                pub key_type: ::data_views::Field<&'a str>,
            }

            pub mod item {

                #[::data_views::data_view]
                pub struct Tls<'a, Mode> {
                    pub enabled: ::data_views::Field<bool>,
                    pub ssl_profile: ::data_views::Field<&'a str>,
                    pub port: ::data_views::Field<i64>,
                }
            }
        }
    }
}
