// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct PolicyMaps<'a, Mode> {
    pub pbr: ::data_views::Field<Pbr<'a, Mode>>,
    pub qos: ::data_views::Field<Qos<'a, Mode>>,
    pub copp_system_policy: ::data_views::Field<CoppSystemPolicy<'a, Mode>>,
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Pbr<'a, Mode> (::data_views::Field<pbr::Item<'a, Mode>>);

pub mod pbr {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub classes: ::data_views::Field<item::Classes<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view(indexed_list, primary_key(name))]
        pub struct Classes<'a, Mode> (::data_views::Field<classes::Item<'a, Mode>>);

        pub mod classes {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::data_views::Field<&'a str>,
                pub index: ::data_views::Field<i64>,
                pub drop: ::data_views::Field<bool>,
                pub set: ::data_views::Field<item::Set<'a, Mode>>,
            }

            pub mod item {

                #[::data_views::data_view]
                pub struct Set<'a, Mode> {
                    pub nexthop: ::data_views::Field<set::Nexthop<'a, Mode>>,
                }

                pub mod set {

                    #[::data_views::data_view]
                    pub struct Nexthop<'a, Mode> {
                        pub ip_address: ::data_views::Field<&'a str>,
                        pub recursive: ::data_views::Field<bool>,
                    }
                }
            }
        }
    }
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Qos<'a, Mode> (::data_views::Field<qos::Item<'a, Mode>>);

pub mod qos {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub classes: ::data_views::Field<item::Classes<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view(indexed_list, primary_key(name))]
        pub struct Classes<'a, Mode> (::data_views::Field<classes::Item<'a, Mode>>);

        pub mod classes {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::data_views::Field<&'a str>,
                pub set: ::data_views::Field<item::Set<'a, Mode>>,
                pub police: ::data_views::Field<item::Police<'a, Mode>>,
            }

            pub mod item {

                #[::data_views::data_view]
                pub struct Set<'a, Mode> {
                    pub cos: ::data_views::Field<i64>,
                    pub dscp: ::data_views::Field<&'a str>,
                    pub traffic_class: ::data_views::Field<i64>,
                    pub drop_precedence: ::data_views::Field<i64>,
                }

                #[::data_views::data_view]
                pub struct Police<'a, Mode> {
                    pub rate: ::data_views::Field<i64>,
                    pub rate_unit: ::data_views::Field<&'a str>,
                    pub rate_burst_size: ::data_views::Field<i64>,
                    pub rate_burst_size_unit: ::data_views::Field<&'a str>,
                    pub action: ::data_views::Field<police::Action<'a, Mode>>,
                    pub higher_rate: ::data_views::Field<i64>,
                    pub higher_rate_unit: ::data_views::Field<&'a str>,
                    pub higher_rate_burst_size: ::data_views::Field<i64>,
                    pub higher_rate_burst_size_unit: ::data_views::Field<&'a str>,
                }

                pub mod police {

                    #[::data_views::data_view]
                    pub struct Action<'a, Mode> {
                        #[data_view(rename = "type")]
                        pub field_type: ::data_views::Field<&'a str>,
                        pub dscp_value: ::data_views::Field<&'a str>,
                    }
                }
            }
        }
    }
}

#[::data_views::data_view]
pub struct CoppSystemPolicy<'a, Mode> {
    pub classes: ::data_views::Field<copp_system_policy::Classes<'a, Mode>>,
}

pub mod copp_system_policy {

    #[::data_views::data_view(indexed_list, primary_key(name))]
    pub struct Classes<'a, Mode> (::data_views::Field<classes::Item<'a, Mode>>);

    pub mod classes {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::data_views::Field<&'a str>,
            pub shape: ::data_views::Field<i64>,
            pub bandwidth: ::data_views::Field<i64>,
            pub rate_unit: ::data_views::Field<&'a str>,
        }
    }
}
