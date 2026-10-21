// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct IpNameServerGroups<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view]
pub struct Item<'a, Mode> {
    pub name: ::data_views::Field<&'a str>,
    pub vrfs: ::data_views::Field<item::Vrfs<'a, Mode>>,
    pub name_servers: ::data_views::Field<item::NameServers<'a, Mode>>,
    pub dns_domain: ::data_views::Field<&'a str>,
    pub ip_domain_lists: ::data_views::Field<item::IpDomainLists<'a, Mode>>,
}

pub mod item {

    #[::data_views::data_view(indexed_list, primary_key(name))]
    pub struct Vrfs<'a, Mode> (::data_views::Field<vrfs::Item<'a, Mode>>);

    pub mod vrfs {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::data_views::Field<&'a str>,
            pub name_servers: ::data_views::RequiredValue<item::NameServers<'a, Mode>, Mode>,
        }

        pub mod item {

            #[::data_views::data_view(indexed_list, primary_key(ip_address))]
            pub struct NameServers<'a, Mode> (::data_views::Field<name_servers::Item<'a, Mode>>);

            pub mod name_servers {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub ip_address: ::data_views::Field<&'a str>,
                    pub priority: ::data_views::Field<i64>,
                }
            }
        }
    }

    #[::data_views::data_view(list, primary_key(ip_address))]
    pub struct NameServers<'a, Mode> (::data_views::Field<name_servers::Item<'a, Mode>>);

    pub mod name_servers {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub ip_address: ::data_views::Field<&'a str>,
            pub vrf: ::data_views::RequiredValue<&'a str, Mode>,
            pub priority: ::data_views::Field<i64>,
        }
    }

    #[::data_views::data_view(list)]
    pub struct IpDomainLists<'a, Mode> (::data_views::Field<&'a str>);
}
