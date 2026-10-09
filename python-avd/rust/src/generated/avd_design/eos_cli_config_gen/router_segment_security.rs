// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Policies<'a, Mode> (::validated_data::Field<policies::Item<'a, Mode>>);

pub mod policies {

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
                pub application: ::validated_data::RequiredValue<&'a str, Mode>,
                pub action: ::validated_data::RequiredValue<&'a str, Mode>,
                pub log: ::validated_data::Field<bool>,
                pub stateless: ::validated_data::Field<bool>,
                pub next_hop: ::validated_data::Field<&'a str>,
            }
        }
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Vrfs<'a, Mode> (::validated_data::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub segments: ::validated_data::RequiredValue<item::Segments<'a, Mode>, Mode>,
    }

    pub mod item {

        #[::validated_data::data_view(indexed_list, primary_key(name))]
        pub struct Segments<'a, Mode> (::validated_data::Field<segments::Item<'a, Mode>>);

        pub mod segments {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::validated_data::Field<&'a str>,
                pub definition: ::validated_data::Field<item::Definition<'a, Mode>>,
                pub policies: ::validated_data::Field<item::Policies<'a, Mode>>,
                pub fallback_policy: ::validated_data::Field<&'a str>,
            }

            pub mod item {

                #[::validated_data::data_view]
                pub struct Definition<'a, Mode> {
                    pub interfaces: ::validated_data::Field<definition::Interfaces<'a, Mode>>,
                    pub match_lists: ::validated_data::Field<definition::MatchLists<'a, Mode>>,
                }

                pub mod definition {

                    #[::validated_data::data_view(list)]
                    pub struct Interfaces<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(indexed_list, primary_key(address_family))]
                    pub struct MatchLists<'a, Mode> (::validated_data::Field<match_lists::Item<'a, Mode>>);

                    pub mod match_lists {

                        #[::validated_data::data_view]
                        pub struct Item<'a, Mode> {
                            pub address_family: ::validated_data::RequiredValue<&'a str, Mode>,
                            pub covered_prefix_list: ::validated_data::Field<&'a str>,
                            pub prefix: ::validated_data::Field<&'a str>,
                        }
                    }
                }

                #[::validated_data::data_view(indexed_list, primary_key(field_from))]
                pub struct Policies<'a, Mode> (::validated_data::Field<policies::Item<'a, Mode>>);

                pub mod policies {

                    #[::validated_data::data_view]
                    pub struct Item<'a, Mode> {
                        #[data_view(rename = "from")]
                        pub field_from: ::validated_data::Field<&'a str>,
                        pub policy: ::validated_data::Field<&'a str>,
                    }
                }
            }
        }
    }
}
