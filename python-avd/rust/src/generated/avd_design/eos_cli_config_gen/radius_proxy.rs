// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct ClientGroups<'a, Mode> (::validated_data::Field<client_groups::Item<'a, Mode>>);

pub mod client_groups {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub server_groups: ::validated_data::Field<item::ServerGroups<'a, Mode>>,
        pub vrfs: ::validated_data::Field<item::Vrfs<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view(list)]
        pub struct ServerGroups<'a, Mode> (::validated_data::Field<&'a str>);

        #[::validated_data::data_view(indexed_list, primary_key(name))]
        pub struct Vrfs<'a, Mode> (::validated_data::Field<vrfs::Item<'a, Mode>>);

        pub mod vrfs {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::validated_data::Field<&'a str>,
                pub ipv4_clients: ::validated_data::Field<item::Ipv4Clients<'a, Mode>>,
                pub ipv6_clients: ::validated_data::Field<item::Ipv6Clients<'a, Mode>>,
                pub host_clients: ::validated_data::Field<item::HostClients<'a, Mode>>,
            }

            pub mod item {

                #[::validated_data::data_view(indexed_list, primary_key(address))]
                pub struct Ipv4Clients<'a, Mode> (::validated_data::Field<ipv4_clients::Item<'a, Mode>>);

                pub mod ipv4_clients {

                    #[::validated_data::data_view]
                    pub struct Item<'a, Mode> {
                        pub address: ::validated_data::Field<&'a str>,
                        pub key: ::validated_data::Field<&'a str>,
                    }
                }

                #[::validated_data::data_view(indexed_list, primary_key(address))]
                pub struct Ipv6Clients<'a, Mode> (::validated_data::Field<ipv6_clients::Item<'a, Mode>>);

                pub mod ipv6_clients {

                    #[::validated_data::data_view]
                    pub struct Item<'a, Mode> {
                        pub address: ::validated_data::Field<&'a str>,
                        pub key: ::validated_data::Field<&'a str>,
                    }
                }

                #[::validated_data::data_view(indexed_list, primary_key(name))]
                pub struct HostClients<'a, Mode> (::validated_data::Field<host_clients::Item<'a, Mode>>);

                pub mod host_clients {

                    #[::validated_data::data_view]
                    pub struct Item<'a, Mode> {
                        pub name: ::validated_data::Field<&'a str>,
                        pub key: ::validated_data::Field<&'a str>,
                    }
                }
            }
        }
    }
}
