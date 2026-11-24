// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct ManagementApiModels<'a, Mode> {
    pub provider: ::data_views::Field<Provider<'a, Mode>>,
}

#[::data_views::data_view]
pub struct Provider<'a, Mode> {
    pub sysdb: ::data_views::Field<provider::Sysdb<'a, Mode>>,
    pub smash: ::data_views::Field<provider::Smash<'a, Mode>>,
    pub macsec: ::data_views::Field<provider::Macsec<'a, Mode>>,
}

pub mod provider {

    #[::data_views::data_view]
    pub struct Sysdb<'a, Mode> {
        pub disabled_paths: ::data_views::Field<sysdb::DisabledPaths<'a, Mode>>,
    }

    pub mod sysdb {

        #[::data_views::data_view(list)]
        pub struct DisabledPaths<'a, Mode> (::data_views::Field<&'a str>);
    }

    #[::data_views::data_view]
    pub struct Smash<'a, Mode> {
        pub paths: ::data_views::Field<smash::Paths<'a, Mode>>,
    }

    pub mod smash {

        #[::data_views::data_view(indexed_list, primary_key(path))]
        pub struct Paths<'a, Mode> (::data_views::Field<paths::Item<'a, Mode>>);

        pub mod paths {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub path: ::data_views::Field<&'a str>,
                pub disabled: ::data_views::Field<bool>,
            }
        }
    }

    #[::data_views::data_view]
    pub struct Macsec<'a, Mode> {
        pub interfaces: ::data_views::Field<bool>,
        pub mka: ::data_views::Field<bool>,
    }
}
