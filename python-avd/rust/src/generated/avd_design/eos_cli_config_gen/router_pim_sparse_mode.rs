// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Ipv4<'a, Mode> {
    pub bfd: ::validated_data::Field<bool>,
    pub make_before_break: ::validated_data::Field<bool>,
    pub message_hello_address_secondary_ipv6: ::validated_data::Field<bool>,
    pub ssm_range: ::validated_data::Field<&'a str>,
    pub register_local_interface: ::validated_data::Field<&'a str>,
    pub rp_addresses: ::validated_data::Field<ipv4::RpAddresses<'a, Mode>>,
    pub anycast_rps: ::validated_data::Field<ipv4::AnycastRps<'a, Mode>>,
}

pub mod ipv4 {

    #[::validated_data::data_view(list, primary_key(address))]
    pub struct RpAddresses<'a, Mode> (::validated_data::Field<rp_addresses::Item<'a, Mode>>);

    pub mod rp_addresses {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub address: ::validated_data::Field<&'a str>,
            pub groups: ::validated_data::Field<item::Groups<'a, Mode>>,
            pub access_lists: ::validated_data::Field<item::AccessLists<'a, Mode>>,
            pub priority: ::validated_data::Field<i64>,
            pub hashmask: ::validated_data::Field<i64>,
            #[data_view(rename = "override")]
            pub field_override: ::validated_data::Field<bool>,
        }

        pub mod item {

            #[::validated_data::data_view(list)]
            pub struct Groups<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(list)]
            pub struct AccessLists<'a, Mode> (::validated_data::Field<&'a str>);
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(address))]
    pub struct AnycastRps<'a, Mode> (::validated_data::Field<anycast_rps::Item<'a, Mode>>);

    pub mod anycast_rps {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub address: ::validated_data::Field<&'a str>,
            pub other_anycast_rp_addresses: ::validated_data::Field<item::OtherAnycastRpAddresses<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view(indexed_list, primary_key(address))]
            pub struct OtherAnycastRpAddresses<'a, Mode> (::validated_data::Field<other_anycast_rp_addresses::Item<'a, Mode>>);

            pub mod other_anycast_rp_addresses {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub address: ::validated_data::Field<&'a str>,
                    pub register_count: ::validated_data::Field<&'a str>,
                }
            }
        }
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Vrfs<'a, Mode> (::validated_data::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub ipv4: ::validated_data::Field<item::Ipv4<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view]
        pub struct Ipv4<'a, Mode> {
            pub bfd: ::validated_data::Field<bool>,
            pub make_before_break: ::validated_data::Field<bool>,
            pub register_local_interface: ::validated_data::Field<&'a str>,
            pub rp_addresses: ::validated_data::Field<ipv4::RpAddresses<'a, Mode>>,
            pub anycast_rps: ::validated_data::Field<ipv4::AnycastRps<'a, Mode>>,
            pub ssm_range: ::validated_data::Field<&'a str>,
        }

        pub mod ipv4 {

            #[::validated_data::data_view(list)]
            pub struct RpAddresses<'a, Mode> (::validated_data::Field<rp_addresses::Item<'a, Mode>>);

            pub mod rp_addresses {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub address: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub groups: ::validated_data::Field<item::Groups<'a, Mode>>,
                    pub access_lists: ::validated_data::Field<item::AccessLists<'a, Mode>>,
                    pub priority: ::validated_data::Field<i64>,
                    pub hashmask: ::validated_data::Field<i64>,
                    #[data_view(rename = "override")]
                    pub field_override: ::validated_data::Field<bool>,
                }

                pub mod item {

                    #[::validated_data::data_view(list)]
                    pub struct Groups<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(list)]
                    pub struct AccessLists<'a, Mode> (::validated_data::Field<&'a str>);
                }
            }

            #[::validated_data::data_view(indexed_list, primary_key(address))]
            pub struct AnycastRps<'a, Mode> (::validated_data::Field<anycast_rps::Item<'a, Mode>>);

            pub mod anycast_rps {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub address: ::validated_data::Field<&'a str>,
                    pub other_anycast_rp_addresses: ::validated_data::Field<item::OtherAnycastRpAddresses<'a, Mode>>,
                }

                pub mod item {

                    #[::validated_data::data_view(indexed_list, primary_key(address))]
                    pub struct OtherAnycastRpAddresses<'a, Mode> (::validated_data::Field<other_anycast_rp_addresses::Item<'a, Mode>>);

                    pub mod other_anycast_rp_addresses {

                        #[::validated_data::data_view]
                        pub struct Item<'a, Mode> {
                            pub address: ::validated_data::Field<&'a str>,
                            pub register_count: ::validated_data::Field<&'a str>,
                        }
                    }
                }
            }
        }
    }
}
