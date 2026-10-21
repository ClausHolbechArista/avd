// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct RouterInternetExit<'a, Mode> {
    pub policies: ::data_views::Field<Policies<'a, Mode>>,
    pub exit_groups: ::data_views::Field<ExitGroups<'a, Mode>>,
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Policies<'a, Mode> (::data_views::Field<policies::Item<'a, Mode>>);

pub mod policies {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub exit_groups: ::data_views::Field<item::ExitGroups<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view(indexed_list, primary_key(name))]
        pub struct ExitGroups<'a, Mode> (::data_views::Field<exit_groups::Item<'a, Mode>>);

        pub mod exit_groups {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::data_views::Field<&'a str>,
            }
        }
    }
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct ExitGroups<'a, Mode> (::data_views::Field<exit_groups::Item<'a, Mode>>);

pub mod exit_groups {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub fib_default: ::data_views::Field<bool>,
        pub local_connections: ::data_views::Field<item::LocalConnections<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view(indexed_list, primary_key(name))]
        pub struct LocalConnections<'a, Mode> (::data_views::Field<local_connections::Item<'a, Mode>>);

        pub mod local_connections {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::data_views::Field<&'a str>,
            }
        }
    }
}
