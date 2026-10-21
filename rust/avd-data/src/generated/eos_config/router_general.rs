// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct RouterGeneral<'a, Mode> {
    pub router_id: ::data_views::Field<RouterId<'a, Mode>>,
    pub nexthop_fast_failover: ::data_views::Field<bool>,
    pub software_forwarding_hardware_offload_mtu: ::data_views::Field<i64>,
    pub vrfs: ::data_views::Field<Vrfs<'a, Mode>>,
    pub control_functions: ::data_views::Field<ControlFunctions<'a, Mode>>,
}

#[::data_views::data_view]
pub struct RouterId<'a, Mode> {
    pub ipv4: ::data_views::Field<&'a str>,
    pub ipv6: ::data_views::Field<&'a str>,
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Vrfs<'a, Mode> (::data_views::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub leak_routes: ::data_views::Field<item::LeakRoutes<'a, Mode>>,
        pub routes: ::data_views::Field<item::Routes<'a, Mode>>,
        pub software_forwarding_hardware_offload_mtu: ::data_views::Field<i64>,
    }

    pub mod item {

        #[::data_views::data_view(list)]
        pub struct LeakRoutes<'a, Mode> (::data_views::Field<leak_routes::Item<'a, Mode>>);

        pub mod leak_routes {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub source_vrf: ::data_views::RequiredValue<&'a str, Mode>,
                pub subscribe_policy: ::data_views::Field<&'a str>,
                pub subscribe_rcf: ::data_views::Field<&'a str>,
            }
        }

        #[::data_views::data_view]
        pub struct Routes<'a, Mode> {
            pub dynamic_prefix_lists: ::data_views::Field<routes::DynamicPrefixLists<'a, Mode>>,
        }

        pub mod routes {

            #[::data_views::data_view(list)]
            pub struct DynamicPrefixLists<'a, Mode> (::data_views::Field<dynamic_prefix_lists::Item<'a, Mode>>);

            pub mod dynamic_prefix_lists {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub name: ::data_views::Field<&'a str>,
                }
            }
        }
    }
}

#[::data_views::data_view]
pub struct ControlFunctions<'a, Mode> {
    pub code_units: ::data_views::Field<control_functions::CodeUnits<'a, Mode>>,
}

pub mod control_functions {

    #[::data_views::data_view(indexed_list, primary_key(name))]
    pub struct CodeUnits<'a, Mode> (::data_views::Field<code_units::Item<'a, Mode>>);

    pub mod code_units {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::data_views::Field<&'a str>,
            pub content: ::data_views::RequiredValue<&'a str, Mode>,
        }
    }
}
