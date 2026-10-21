// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct RouterSegmentSecurity<'a, Mode> {
    pub enabled: ::data_views::Field<bool>,
    pub policies: ::data_views::Field<Policies<'a, Mode>>,
    pub vrfs: ::data_views::Field<Vrfs<'a, Mode>>,
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Policies<'a, Mode> (::data_views::Field<policies::Item<'a, Mode>>);

pub mod policies {

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
                pub application: ::data_views::RequiredValue<&'a str, Mode>,
                pub action: ::data_views::RequiredValue<&'a str, Mode>,
                pub log: ::data_views::Field<bool>,
                pub stateless: ::data_views::Field<bool>,
                pub next_hop: ::data_views::Field<&'a str>,
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
        pub segments: ::data_views::RequiredValue<item::Segments<'a, Mode>, Mode>,
    }

    pub mod item {

        #[::data_views::data_view(indexed_list, primary_key(name))]
        pub struct Segments<'a, Mode> (::data_views::Field<segments::Item<'a, Mode>>);

        pub mod segments {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::data_views::Field<&'a str>,
                pub definition: ::data_views::Field<item::Definition<'a, Mode>>,
                pub policies: ::data_views::Field<item::Policies<'a, Mode>>,
                pub fallback_policy: ::data_views::Field<&'a str>,
            }

            pub mod item {

                #[::data_views::data_view]
                pub struct Definition<'a, Mode> {
                    pub interfaces: ::data_views::Field<definition::Interfaces<'a, Mode>>,
                    pub match_lists: ::data_views::Field<definition::MatchLists<'a, Mode>>,
                }

                pub mod definition {

                    #[::data_views::data_view(list)]
                    pub struct Interfaces<'a, Mode> (::data_views::Field<&'a str>);

                    #[::data_views::data_view(indexed_list, primary_key(address_family))]
                    pub struct MatchLists<'a, Mode> (::data_views::Field<match_lists::Item<'a, Mode>>);

                    pub mod match_lists {

                        #[::data_views::data_view]
                        pub struct Item<'a, Mode> {
                            pub address_family: ::data_views::RequiredValue<&'a str, Mode>,
                            pub covered_prefix_list: ::data_views::Field<&'a str>,
                            pub prefix: ::data_views::Field<&'a str>,
                        }
                    }
                }

                #[::data_views::data_view(indexed_list, primary_key(field_from))]
                pub struct Policies<'a, Mode> (::data_views::Field<policies::Item<'a, Mode>>);

                pub mod policies {

                    #[::data_views::data_view]
                    pub struct Item<'a, Mode> {
                        #[data_view(rename = "from")]
                        pub field_from: ::data_views::Field<&'a str>,
                        pub policy: ::data_views::Field<&'a str>,
                    }
                }
            }
        }
    }
}
