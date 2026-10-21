// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct MatchListInput<'a, Mode> {
    pub prefix_ipv4: ::data_views::Field<PrefixIpv4<'a, Mode>>,
    pub prefix_ipv6: ::data_views::Field<PrefixIpv6<'a, Mode>>,
    pub string: ::data_views::Field<String<'a, Mode>>,
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct PrefixIpv4<'a, Mode> (::data_views::Field<prefix_ipv4::Item<'a, Mode>>);

pub mod prefix_ipv4 {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub prefixes: ::data_views::RequiredValue<item::Prefixes<'a, Mode>, Mode>,
    }

    pub mod item {

        #[::data_views::data_view(list)]
        pub struct Prefixes<'a, Mode> (::data_views::Field<&'a str>);
    }
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct PrefixIpv6<'a, Mode> (::data_views::Field<prefix_ipv6::Item<'a, Mode>>);

pub mod prefix_ipv6 {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub prefixes: ::data_views::RequiredValue<item::Prefixes<'a, Mode>, Mode>,
    }

    pub mod item {

        #[::data_views::data_view(list)]
        pub struct Prefixes<'a, Mode> (::data_views::Field<&'a str>);
    }
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct String<'a, Mode> (::data_views::Field<string::Item<'a, Mode>>);

pub mod string {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub sequence_numbers: ::data_views::RequiredValue<item::SequenceNumbers<'a, Mode>, Mode>,
    }

    pub mod item {

        #[::data_views::data_view(indexed_list, primary_key(sequence))]
        pub struct SequenceNumbers<'a, Mode> (::data_views::Field<sequence_numbers::Item<'a, Mode>>);

        pub mod sequence_numbers {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub sequence: ::data_views::Field<i64>,
                pub match_regex: ::data_views::RequiredValue<&'a str, Mode>,
            }
        }
    }
}
