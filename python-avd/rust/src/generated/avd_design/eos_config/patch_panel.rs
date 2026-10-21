// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct PatchPanel<'a, Mode> {
    pub connector: ::data_views::Field<Connector<'a, Mode>>,
    pub patches: ::data_views::Field<Patches<'a, Mode>>,
}

#[::data_views::data_view]
pub struct Connector<'a, Mode> {
    pub interface: ::data_views::Field<connector::Interface<'a, Mode>>,
}

pub mod connector {

    #[::data_views::data_view]
    pub struct Interface<'a, Mode> {
        pub patch: ::data_views::Field<interface::Patch<'a, Mode>>,
        pub recovery: ::data_views::Field<interface::Recovery<'a, Mode>>,
    }

    pub mod interface {

        #[::data_views::data_view]
        pub struct Patch<'a, Mode> {
            pub bgp_vpws_remote_failure_errdisable: ::data_views::Field<bool>,
        }

        #[::data_views::data_view]
        pub struct Recovery<'a, Mode> {
            pub review_delay: ::data_views::Field<recovery::ReviewDelay<'a, Mode>>,
        }

        pub mod recovery {

            #[::data_views::data_view]
            pub struct ReviewDelay<'a, Mode> {
                pub min: ::data_views::RequiredValue<i64, Mode>,
                pub max: ::data_views::RequiredValue<i64, Mode>,
            }
        }
    }
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Patches<'a, Mode> (::data_views::Field<patches::Item<'a, Mode>>);

pub mod patches {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub enabled: ::data_views::Field<bool>,
        pub connectors: ::data_views::Field<item::Connectors<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view(indexed_list, primary_key(id))]
        pub struct Connectors<'a, Mode> (::data_views::Field<connectors::Item<'a, Mode>>);

        pub mod connectors {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub id: ::data_views::Field<&'a str>,
                #[data_view(rename = "type")]
                pub field_type: ::data_views::RequiredValue<&'a str, Mode>,
                pub endpoint: ::data_views::RequiredValue<&'a str, Mode>,
            }
        }
    }
}
