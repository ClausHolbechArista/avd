// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
    pub vrfs: ::validated_data::Field<item::Vrfs<'a, Mode>>,
    pub name_servers: ::validated_data::Field<item::NameServers<'a, Mode>>,
    pub dns_domain: ::validated_data::Field<&'a str>,
    pub ip_domain_lists: ::validated_data::Field<item::IpDomainLists<'a, Mode>>,
}

pub mod item {

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct Vrfs<'a, Mode> (::validated_data::Field<vrfs::Item<'a, Mode>>);

    pub mod vrfs {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub name_servers: ::validated_data::RequiredValue<item::NameServers<'a, Mode>, Mode>,
        }

        pub mod item {

            #[::validated_data::data_view(indexed_list, primary_key(ip_address))]
            pub struct NameServers<'a, Mode> (::validated_data::Field<name_servers::Item<'a, Mode>>);

            pub mod name_servers {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub ip_address: ::validated_data::Field<&'a str>,
                    pub priority: ::validated_data::Field<i64>,
                }
            }
        }
    }

    #[::validated_data::data_view(list, primary_key(ip_address))]
    pub struct NameServers<'a, Mode> (::validated_data::Field<name_servers::Item<'a, Mode>>);

    pub mod name_servers {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub ip_address: ::validated_data::Field<&'a str>,
            pub vrf: ::validated_data::RequiredValue<&'a str, Mode>,
            pub priority: ::validated_data::Field<i64>,
        }
    }

    #[::validated_data::data_view(list)]
    pub struct IpDomainLists<'a, Mode> (::validated_data::Field<&'a str>);
}
