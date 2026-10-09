// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
    pub counters_per_entry: ::validated_data::Field<bool>,
    pub entries: ::validated_data::Field<item::Entries<'a, Mode>>,
    pub sequence_numbers: ::validated_data::Field<item::SequenceNumbers<'a, Mode>>,
}

pub mod item {

    #[::validated_data::data_view(list)]
    pub struct Entries<'a, Mode> (::validated_data::Field<entries::Item<'a, Mode>>);

    pub mod entries {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub sequence: ::validated_data::Field<i64>,
            pub action: ::validated_data::Field<&'a str>,
            pub remark: ::validated_data::Field<&'a str>,
            pub source: ::validated_data::Field<&'a str>,
            pub vlan: ::validated_data::Field<i64>,
            pub vlan_mask: ::validated_data::Field<&'a str>,
            pub inner_vlan: ::validated_data::Field<i64>,
            pub inner_vlan_mask: ::validated_data::Field<&'a str>,
            pub log: ::validated_data::Field<bool>,
            pub mirror_session: ::validated_data::Field<&'a str>,
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(sequence))]
    pub struct SequenceNumbers<'a, Mode> (::validated_data::Field<sequence_numbers::Item<'a, Mode>>);

    pub mod sequence_numbers {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub sequence: ::validated_data::Field<i64>,
            pub action: ::validated_data::RequiredValue<&'a str, Mode>,
        }
    }
}
