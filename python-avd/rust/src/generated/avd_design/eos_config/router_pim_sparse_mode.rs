// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct RouterPimSparseMode<'a, Mode> {
    pub ipv4: ::data_views::Field<Ipv4<'a, Mode>>,
    pub vrfs: ::data_views::Field<Vrfs<'a, Mode>>,
}

#[::data_views::data_view]
pub struct Ipv4<'a, Mode> {
    pub bfd: ::data_views::Field<bool>,
    pub make_before_break: ::data_views::Field<bool>,
    pub message_hello_address_secondary_ipv6: ::data_views::Field<bool>,
    pub ssm_range: ::data_views::Field<&'a str>,
    pub register_local_interface: ::data_views::Field<&'a str>,
    pub rp_addresses: ::data_views::Field<ipv4::RpAddresses<'a, Mode>>,
    pub anycast_rps: ::data_views::Field<ipv4::AnycastRps<'a, Mode>>,
}

pub mod ipv4 {

    #[::data_views::data_view(list, primary_key(address))]
    pub struct RpAddresses<'a, Mode> (::data_views::Field<rp_addresses::Item<'a, Mode>>);

    pub mod rp_addresses {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub address: ::data_views::Field<&'a str>,
            pub groups: ::data_views::Field<item::Groups<'a, Mode>>,
            pub access_lists: ::data_views::Field<item::AccessLists<'a, Mode>>,
            pub priority: ::data_views::Field<i64>,
            pub hashmask: ::data_views::Field<i64>,
            #[data_view(rename = "override")]
            pub field_override: ::data_views::Field<bool>,
        }

        pub mod item {

            #[::data_views::data_view(list)]
            pub struct Groups<'a, Mode> (::data_views::Field<&'a str>);

            #[::data_views::data_view(list)]
            pub struct AccessLists<'a, Mode> (::data_views::Field<&'a str>);
        }
    }

    #[::data_views::data_view(indexed_list, primary_key(address))]
    pub struct AnycastRps<'a, Mode> (::data_views::Field<anycast_rps::Item<'a, Mode>>);

    pub mod anycast_rps {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub address: ::data_views::Field<&'a str>,
            pub other_anycast_rp_addresses: ::data_views::Field<item::OtherAnycastRpAddresses<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view(indexed_list, primary_key(address))]
            pub struct OtherAnycastRpAddresses<'a, Mode> (::data_views::Field<other_anycast_rp_addresses::Item<'a, Mode>>);

            pub mod other_anycast_rp_addresses {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub address: ::data_views::Field<&'a str>,
                    pub register_count: ::data_views::Field<&'a str>,
                }
            }
        }
    }
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Vrfs<'a, Mode> (::data_views::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub ipv4: ::data_views::Field<item::Ipv4<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view]
        pub struct Ipv4<'a, Mode> {
            pub bfd: ::data_views::Field<bool>,
            pub make_before_break: ::data_views::Field<bool>,
            pub register_local_interface: ::data_views::Field<&'a str>,
            pub rp_addresses: ::data_views::Field<ipv4::RpAddresses<'a, Mode>>,
            pub anycast_rps: ::data_views::Field<ipv4::AnycastRps<'a, Mode>>,
            pub ssm_range: ::data_views::Field<&'a str>,
        }

        pub mod ipv4 {

            #[::data_views::data_view(list)]
            pub struct RpAddresses<'a, Mode> (::data_views::Field<rp_addresses::Item<'a, Mode>>);

            pub mod rp_addresses {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub address: ::data_views::RequiredValue<&'a str, Mode>,
                    pub groups: ::data_views::Field<item::Groups<'a, Mode>>,
                    pub access_lists: ::data_views::Field<item::AccessLists<'a, Mode>>,
                    pub priority: ::data_views::Field<i64>,
                    pub hashmask: ::data_views::Field<i64>,
                    #[data_view(rename = "override")]
                    pub field_override: ::data_views::Field<bool>,
                }

                pub mod item {

                    #[::data_views::data_view(list)]
                    pub struct Groups<'a, Mode> (::data_views::Field<&'a str>);

                    #[::data_views::data_view(list)]
                    pub struct AccessLists<'a, Mode> (::data_views::Field<&'a str>);
                }
            }

            #[::data_views::data_view(indexed_list, primary_key(address))]
            pub struct AnycastRps<'a, Mode> (::data_views::Field<anycast_rps::Item<'a, Mode>>);

            pub mod anycast_rps {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub address: ::data_views::Field<&'a str>,
                    pub other_anycast_rp_addresses: ::data_views::Field<item::OtherAnycastRpAddresses<'a, Mode>>,
                }

                pub mod item {

                    #[::data_views::data_view(indexed_list, primary_key(address))]
                    pub struct OtherAnycastRpAddresses<'a, Mode> (::data_views::Field<other_anycast_rp_addresses::Item<'a, Mode>>);

                    pub mod other_anycast_rp_addresses {

                        #[::data_views::data_view]
                        pub struct Item<'a, Mode> {
                            pub address: ::data_views::Field<&'a str>,
                            pub register_count: ::data_views::Field<&'a str>,
                        }
                    }
                }
            }
        }
    }
}
