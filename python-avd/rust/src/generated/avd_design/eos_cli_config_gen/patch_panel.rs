// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Connector<'a, Mode> {
    pub interface: ::validated_data::Field<connector::Interface<'a, Mode>>,
}

pub mod connector {

    #[::validated_data::data_view]
    pub struct Interface<'a, Mode> {
        pub patch: ::validated_data::Field<interface::Patch<'a, Mode>>,
        pub recovery: ::validated_data::Field<interface::Recovery<'a, Mode>>,
    }

    pub mod interface {

        #[::validated_data::data_view]
        pub struct Patch<'a, Mode> {
            pub bgp_vpws_remote_failure_errdisable: ::validated_data::Field<bool>,
        }

        #[::validated_data::data_view]
        pub struct Recovery<'a, Mode> {
            pub review_delay: ::validated_data::Field<recovery::ReviewDelay<'a, Mode>>,
        }

        pub mod recovery {

            #[::validated_data::data_view]
            pub struct ReviewDelay<'a, Mode> {
                pub min: ::validated_data::RequiredValue<i64, Mode>,
                pub max: ::validated_data::RequiredValue<i64, Mode>,
            }
        }
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Patches<'a, Mode> (::validated_data::Field<patches::Item<'a, Mode>>);

pub mod patches {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub enabled: ::validated_data::Field<bool>,
        pub connectors: ::validated_data::Field<item::Connectors<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view(indexed_list, primary_key(id))]
        pub struct Connectors<'a, Mode> (::validated_data::Field<connectors::Item<'a, Mode>>);

        pub mod connectors {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub id: ::validated_data::Field<&'a str>,
                #[data_view(rename = "type")]
                pub field_type: ::validated_data::RequiredValue<&'a str, Mode>,
                pub endpoint: ::validated_data::RequiredValue<&'a str, Mode>,
            }
        }
    }
}
