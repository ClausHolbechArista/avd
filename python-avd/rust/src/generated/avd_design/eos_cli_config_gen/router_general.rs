// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct RouterId<'a, Mode> {
    pub ipv4: ::validated_data::Field<&'a str>,
    pub ipv6: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Vrfs<'a, Mode> (::validated_data::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub leak_routes: ::validated_data::Field<item::LeakRoutes<'a, Mode>>,
        pub routes: ::validated_data::Field<item::Routes<'a, Mode>>,
        pub software_forwarding_hardware_offload_mtu: ::validated_data::Field<i64>,
    }

    pub mod item {

        #[::validated_data::data_view(list)]
        pub struct LeakRoutes<'a, Mode> (::validated_data::Field<leak_routes::Item<'a, Mode>>);

        pub mod leak_routes {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub source_vrf: ::validated_data::RequiredValue<&'a str, Mode>,
                pub subscribe_policy: ::validated_data::Field<&'a str>,
                pub subscribe_rcf: ::validated_data::Field<&'a str>,
            }
        }

        #[::validated_data::data_view]
        pub struct Routes<'a, Mode> {
            pub dynamic_prefix_lists: ::validated_data::Field<routes::DynamicPrefixLists<'a, Mode>>,
        }

        pub mod routes {

            #[::validated_data::data_view(list)]
            pub struct DynamicPrefixLists<'a, Mode> (::validated_data::Field<dynamic_prefix_lists::Item<'a, Mode>>);

            pub mod dynamic_prefix_lists {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub name: ::validated_data::Field<&'a str>,
                }
            }
        }
    }
}

#[::validated_data::data_view]
pub struct ControlFunctions<'a, Mode> {
    pub code_units: ::validated_data::Field<control_functions::CodeUnits<'a, Mode>>,
}

pub mod control_functions {

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct CodeUnits<'a, Mode> (::validated_data::Field<code_units::Item<'a, Mode>>);

    pub mod code_units {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub content: ::validated_data::RequiredValue<&'a str, Mode>,
        }
    }
}
