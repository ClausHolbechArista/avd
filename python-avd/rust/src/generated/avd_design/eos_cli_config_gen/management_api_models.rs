// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Provider<'a, Mode> {
    pub sysdb: ::validated_data::Field<provider::Sysdb<'a, Mode>>,
    pub smash: ::validated_data::Field<provider::Smash<'a, Mode>>,
    pub macsec: ::validated_data::Field<provider::Macsec<'a, Mode>>,
}

pub mod provider {

    #[::validated_data::data_view]
    pub struct Sysdb<'a, Mode> {
        pub disabled_paths: ::validated_data::Field<sysdb::DisabledPaths<'a, Mode>>,
    }

    pub mod sysdb {

        #[::validated_data::data_view(list)]
        pub struct DisabledPaths<'a, Mode> (::validated_data::Field<&'a str>);
    }

    #[::validated_data::data_view]
    pub struct Smash<'a, Mode> {
        pub paths: ::validated_data::Field<smash::Paths<'a, Mode>>,
    }

    pub mod smash {

        #[::validated_data::data_view(indexed_list, primary_key(path))]
        pub struct Paths<'a, Mode> (::validated_data::Field<paths::Item<'a, Mode>>);

        pub mod paths {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub path: ::validated_data::Field<&'a str>,
                pub disabled: ::validated_data::Field<bool>,
            }
        }
    }

    #[::validated_data::data_view]
    pub struct Macsec<'a, Mode> {
        pub interfaces: ::validated_data::Field<bool>,
        pub mka: ::validated_data::Field<bool>,
    }
}
