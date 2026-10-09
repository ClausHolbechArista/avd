// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Pbr<'a, Mode> (::validated_data::Field<pbr::Item<'a, Mode>>);

pub mod pbr {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub classes: ::validated_data::Field<item::Classes<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view(indexed_list, primary_key(name))]
        pub struct Classes<'a, Mode> (::validated_data::Field<classes::Item<'a, Mode>>);

        pub mod classes {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::validated_data::Field<&'a str>,
                pub index: ::validated_data::Field<i64>,
                pub drop: ::validated_data::Field<bool>,
                pub set: ::validated_data::Field<item::Set<'a, Mode>>,
            }

            pub mod item {

                #[::validated_data::data_view]
                pub struct Set<'a, Mode> {
                    pub nexthop: ::validated_data::Field<set::Nexthop<'a, Mode>>,
                }

                pub mod set {

                    #[::validated_data::data_view]
                    pub struct Nexthop<'a, Mode> {
                        pub ip_address: ::validated_data::Field<&'a str>,
                        pub recursive: ::validated_data::Field<bool>,
                    }
                }
            }
        }
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Qos<'a, Mode> (::validated_data::Field<qos::Item<'a, Mode>>);

pub mod qos {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub classes: ::validated_data::Field<item::Classes<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view(indexed_list, primary_key(name))]
        pub struct Classes<'a, Mode> (::validated_data::Field<classes::Item<'a, Mode>>);

        pub mod classes {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::validated_data::Field<&'a str>,
                pub set: ::validated_data::Field<item::Set<'a, Mode>>,
                pub police: ::validated_data::Field<item::Police<'a, Mode>>,
            }

            pub mod item {

                #[::validated_data::data_view]
                pub struct Set<'a, Mode> {
                    pub cos: ::validated_data::Field<i64>,
                    pub dscp: ::validated_data::Field<&'a str>,
                    pub traffic_class: ::validated_data::Field<i64>,
                    pub drop_precedence: ::validated_data::Field<i64>,
                }

                #[::validated_data::data_view]
                pub struct Police<'a, Mode> {
                    pub rate: ::validated_data::Field<i64>,
                    pub rate_unit: ::validated_data::Field<&'a str>,
                    pub rate_burst_size: ::validated_data::Field<i64>,
                    pub rate_burst_size_unit: ::validated_data::Field<&'a str>,
                    pub action: ::validated_data::Field<police::Action<'a, Mode>>,
                    pub higher_rate: ::validated_data::Field<i64>,
                    pub higher_rate_unit: ::validated_data::Field<&'a str>,
                    pub higher_rate_burst_size: ::validated_data::Field<i64>,
                    pub higher_rate_burst_size_unit: ::validated_data::Field<&'a str>,
                }

                pub mod police {

                    #[::validated_data::data_view]
                    pub struct Action<'a, Mode> {
                        #[data_view(rename = "type")]
                        pub field_type: ::validated_data::Field<&'a str>,
                        pub dscp_value: ::validated_data::Field<&'a str>,
                    }
                }
            }
        }
    }
}

#[::validated_data::data_view]
pub struct CoppSystemPolicy<'a, Mode> {
    pub classes: ::validated_data::Field<copp_system_policy::Classes<'a, Mode>>,
}

pub mod copp_system_policy {

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct Classes<'a, Mode> (::validated_data::Field<classes::Item<'a, Mode>>);

    pub mod classes {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub shape: ::validated_data::Field<i64>,
            pub bandwidth: ::validated_data::Field<i64>,
            pub rate_unit: ::validated_data::Field<&'a str>,
        }
    }
}
