// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct RadiusProxy<'a, Mode> {
    pub client_key: ::data_views::Field<&'a str>,
    pub client_session_idle_timeout: ::data_views::Field<i64>,
    pub dynamic_authorization: ::data_views::Field<bool>,
    pub client_groups: ::data_views::Field<ClientGroups<'a, Mode>>,
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct ClientGroups<'a, Mode> (::data_views::Field<client_groups::Item<'a, Mode>>);

pub mod client_groups {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub server_groups: ::data_views::Field<item::ServerGroups<'a, Mode>>,
        pub vrfs: ::data_views::Field<item::Vrfs<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view(list)]
        pub struct ServerGroups<'a, Mode> (::data_views::Field<&'a str>);

        #[::data_views::data_view(indexed_list, primary_key(name))]
        pub struct Vrfs<'a, Mode> (::data_views::Field<vrfs::Item<'a, Mode>>);

        pub mod vrfs {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::data_views::Field<&'a str>,
                pub ipv4_clients: ::data_views::Field<item::Ipv4Clients<'a, Mode>>,
                pub ipv6_clients: ::data_views::Field<item::Ipv6Clients<'a, Mode>>,
                pub host_clients: ::data_views::Field<item::HostClients<'a, Mode>>,
            }

            pub mod item {

                #[::data_views::data_view(indexed_list, primary_key(address))]
                pub struct Ipv4Clients<'a, Mode> (::data_views::Field<ipv4_clients::Item<'a, Mode>>);

                pub mod ipv4_clients {

                    #[::data_views::data_view]
                    pub struct Item<'a, Mode> {
                        pub address: ::data_views::Field<&'a str>,
                        pub key: ::data_views::Field<&'a str>,
                    }
                }

                #[::data_views::data_view(indexed_list, primary_key(address))]
                pub struct Ipv6Clients<'a, Mode> (::data_views::Field<ipv6_clients::Item<'a, Mode>>);

                pub mod ipv6_clients {

                    #[::data_views::data_view]
                    pub struct Item<'a, Mode> {
                        pub address: ::data_views::Field<&'a str>,
                        pub key: ::data_views::Field<&'a str>,
                    }
                }

                #[::data_views::data_view(indexed_list, primary_key(name))]
                pub struct HostClients<'a, Mode> (::data_views::Field<host_clients::Item<'a, Mode>>);

                pub mod host_clients {

                    #[::data_views::data_view]
                    pub struct Item<'a, Mode> {
                        pub name: ::data_views::Field<&'a str>,
                        pub key: ::data_views::Field<&'a str>,
                    }
                }
            }
        }
    }
}
