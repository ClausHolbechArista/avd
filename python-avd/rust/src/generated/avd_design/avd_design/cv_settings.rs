// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Cvaas<'a, Mode> {
    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
    pub clusters: ::validated_data::Field<cvaas::Clusters<'a, Mode>>,
}

pub mod cvaas {

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct Clusters<'a, Mode> (::validated_data::Field<clusters::Item<'a, Mode>>);

    pub mod clusters {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub region: ::validated_data::Field<&'a str>,
            pub vrf: ::validated_data::Field<&'a str>,
            pub token_file: ::validated_data::Field<&'a str>,
            pub source_interface: ::validated_data::Field<&'a str>,
        }
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct OnpremClusters<'a, Mode> (::validated_data::Field<onprem_clusters::Item<'a, Mode>>);

pub mod onprem_clusters {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub servers: ::validated_data::RequiredValue<item::Servers<'a, Mode>, Mode>,
        pub vrf: ::validated_data::Field<&'a str>,
        pub token_file: ::validated_data::Field<&'a str>,
        pub source_interface: ::validated_data::Field<&'a str>,
    }

    pub mod item {

        #[::validated_data::data_view(indexed_list, primary_key(name))]
        pub struct Servers<'a, Mode> (::validated_data::Field<servers::Item<'a, Mode>>);

        pub mod servers {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::validated_data::Field<&'a str>,
                pub port: ::validated_data::Field<i64>,
            }
        }
    }
}

#[::validated_data::data_view]
pub struct Terminattr<'a, Mode> {
    pub ingestexclude: ::validated_data::Field<&'a str>,
    pub smashexcludes: ::validated_data::Field<&'a str>,
    pub disable_aaa: ::validated_data::Field<bool>,
    pub cvtargetconfigs: ::validated_data::Field<terminattr::Cvtargetconfigs<'a, Mode>>,
    pub flowdns: ::validated_data::Field<bool>,
    pub custom_cv_options: ::validated_data::Field<terminattr::CustomCvOptions<'a, Mode>>,
}

pub mod terminattr {

    #[::validated_data::data_view(list)]
    pub struct Cvtargetconfigs<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view(list)]
    pub struct CustomCvOptions<'a, Mode> (::validated_data::Field<custom_cv_options::Item<'a, Mode>>);

    pub mod custom_cv_options {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub flag: ::validated_data::RequiredValue<&'a str, Mode>,
            pub value: ::validated_data::Field<&'a str>,
        }
    }
}
