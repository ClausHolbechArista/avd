// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct PrefixIpv4<'a, Mode> (::validated_data::Field<prefix_ipv4::Item<'a, Mode>>);

pub mod prefix_ipv4 {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub prefixes: ::validated_data::RequiredValue<item::Prefixes<'a, Mode>, Mode>,
    }

    pub mod item {

        #[::validated_data::data_view(list)]
        pub struct Prefixes<'a, Mode> (::validated_data::Field<&'a str>);
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct PrefixIpv6<'a, Mode> (::validated_data::Field<prefix_ipv6::Item<'a, Mode>>);

pub mod prefix_ipv6 {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub prefixes: ::validated_data::RequiredValue<item::Prefixes<'a, Mode>, Mode>,
    }

    pub mod item {

        #[::validated_data::data_view(list)]
        pub struct Prefixes<'a, Mode> (::validated_data::Field<&'a str>);
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct String<'a, Mode> (::validated_data::Field<string::Item<'a, Mode>>);

pub mod string {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub sequence_numbers: ::validated_data::RequiredValue<item::SequenceNumbers<'a, Mode>, Mode>,
    }

    pub mod item {

        #[::validated_data::data_view(indexed_list, primary_key(sequence))]
        pub struct SequenceNumbers<'a, Mode> (::validated_data::Field<sequence_numbers::Item<'a, Mode>>);

        pub mod sequence_numbers {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub sequence: ::validated_data::Field<i64>,
                pub match_regex: ::validated_data::RequiredValue<&'a str, Mode>,
            }
        }
    }
}
