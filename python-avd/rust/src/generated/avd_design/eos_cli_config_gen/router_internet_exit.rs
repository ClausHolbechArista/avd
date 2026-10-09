// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Policies<'a, Mode> (::validated_data::Field<policies::Item<'a, Mode>>);

pub mod policies {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub exit_groups: ::validated_data::Field<item::ExitGroups<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view(indexed_list, primary_key(name))]
        pub struct ExitGroups<'a, Mode> (::validated_data::Field<exit_groups::Item<'a, Mode>>);

        pub mod exit_groups {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::validated_data::Field<&'a str>,
            }
        }
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct ExitGroups<'a, Mode> (::validated_data::Field<exit_groups::Item<'a, Mode>>);

pub mod exit_groups {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub fib_default: ::validated_data::Field<bool>,
        pub local_connections: ::validated_data::Field<item::LocalConnections<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view(indexed_list, primary_key(name))]
        pub struct LocalConnections<'a, Mode> (::validated_data::Field<local_connections::Item<'a, Mode>>);

        pub mod local_connections {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::validated_data::Field<&'a str>,
            }
        }
    }
}
