// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Destinations<'a, Mode> (::validated_data::Field<destinations::Item<'a, Mode>>);

pub mod destinations {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub database: ::validated_data::Field<&'a str>,
        pub data_retention_policy: ::validated_data::Field<&'a str>,
        pub url: ::validated_data::Field<&'a str>,
        pub username: ::validated_data::Field<&'a str>,
        pub password: ::validated_data::Field<&'a str>,
        pub password_type: ::validated_data::Field<&'a str>,
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct SourceSockets<'a, Mode> (::validated_data::Field<source_sockets::Item<'a, Mode>>);

pub mod source_sockets {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub connection_limit: ::validated_data::Field<i64>,
        pub url: ::validated_data::Field<&'a str>,
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Tags<'a, Mode> (::validated_data::Field<tags::Item<'a, Mode>>);

pub mod tags {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub value: ::validated_data::RequiredValue<&'a str, Mode>,
    }
}
