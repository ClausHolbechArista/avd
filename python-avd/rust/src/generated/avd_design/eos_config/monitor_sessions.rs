// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
    pub sources: ::validated_data::Field<item::Sources<'a, Mode>>,
    pub destinations: ::validated_data::Field<item::Destinations<'a, Mode>>,
    pub encapsulation_gre_metadata_tx: ::validated_data::Field<bool>,
    pub header_remove_size: ::validated_data::Field<i64>,
    pub access_group: ::validated_data::Field<item::AccessGroup<'a, Mode>>,
    pub rate_limit_per_ingress_chip: ::validated_data::Field<&'a str>,
    pub rate_limit_per_egress_chip: ::validated_data::Field<&'a str>,
    pub sample: ::validated_data::Field<i64>,
    pub truncate: ::validated_data::Field<item::Truncate<'a, Mode>>,
}

pub mod item {

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct Sources<'a, Mode> (::validated_data::Field<sources::Item<'a, Mode>>);

    pub mod sources {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub direction: ::validated_data::Field<&'a str>,
            pub access_group: ::validated_data::Field<item::AccessGroup<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view]
            pub struct AccessGroup<'a, Mode> {
                #[data_view(rename = "type")]
                pub field_type: ::validated_data::Field<&'a str>,
                pub name: ::validated_data::Field<&'a str>,
                pub priority: ::validated_data::Field<i64>,
            }
        }
    }

    #[::validated_data::data_view(list)]
    pub struct Destinations<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view]
    pub struct AccessGroup<'a, Mode> {
        #[data_view(rename = "type")]
        pub field_type: ::validated_data::Field<&'a str>,
        pub name: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view]
    pub struct Truncate<'a, Mode> {
        pub enabled: ::validated_data::Field<bool>,
        pub size: ::validated_data::Field<i64>,
    }
}
