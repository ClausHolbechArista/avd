// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Attribute32IncludeInAccessReq<'a, Mode> {
    pub hostname: ::validated_data::Field<bool>,
    pub format: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view]
pub struct DynamicAuthorization<'a, Mode> {
    pub port: ::validated_data::Field<i64>,
    pub tls_ssl_profile: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(list)]
pub struct Servers<'a, Mode> (::validated_data::Field<servers::Item<'a, Mode>>);

pub mod servers {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub host: ::validated_data::RequiredValue<&'a str, Mode>,
        pub tls: ::validated_data::Field<item::Tls<'a, Mode>>,
        pub timeout: ::validated_data::Field<i64>,
        pub retransmit: ::validated_data::Field<i64>,
        pub key: ::validated_data::Field<&'a str>,
        pub key_type: ::validated_data::Field<&'a str>,
    }

    pub mod item {

        #[::validated_data::data_view]
        pub struct Tls<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub ssl_profile: ::validated_data::Field<&'a str>,
            pub port: ::validated_data::Field<i64>,
        }
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Vrfs<'a, Mode> (::validated_data::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub servers: ::validated_data::RequiredValue<item::Servers<'a, Mode>, Mode>,
    }

    pub mod item {

        #[::validated_data::data_view(list)]
        pub struct Servers<'a, Mode> (::validated_data::Field<servers::Item<'a, Mode>>);

        pub mod servers {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub host: ::validated_data::RequiredValue<&'a str, Mode>,
                pub tls: ::validated_data::Field<item::Tls<'a, Mode>>,
                pub timeout: ::validated_data::Field<i64>,
                pub retransmit: ::validated_data::Field<i64>,
                pub key: ::validated_data::Field<&'a str>,
                pub key_type: ::validated_data::Field<&'a str>,
            }

            pub mod item {

                #[::validated_data::data_view]
                pub struct Tls<'a, Mode> {
                    pub enabled: ::validated_data::Field<bool>,
                    pub ssl_profile: ::validated_data::Field<&'a str>,
                    pub port: ::validated_data::Field<i64>,
                }
            }
        }
    }
}
