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
            pub protocol: ::validated_data::Field<&'a str>,
            pub source: ::validated_data::Field<&'a str>,
            pub destination: ::validated_data::Field<&'a str>,
            pub hop_limit: ::validated_data::Field<i64>,
            pub hop_limit_match: ::validated_data::Field<&'a str>,
            pub dscp_mask: ::validated_data::Field<&'a str>,
            pub sequence: ::validated_data::Field<i64>,
            pub remark: ::validated_data::Field<&'a str>,
            pub action: ::validated_data::Field<&'a str>,
            pub source_ports_match: ::validated_data::Field<&'a str>,
            pub source_ports: ::validated_data::Field<item::SourcePorts<'a, Mode>>,
            pub destination_ports_match: ::validated_data::Field<&'a str>,
            pub destination_ports: ::validated_data::Field<item::DestinationPorts<'a, Mode>>,
            pub tcp_flags: ::validated_data::Field<item::TcpFlags<'a, Mode>>,
            pub copy_captive_portal: ::validated_data::Field<bool>,
            pub log: ::validated_data::Field<bool>,
            pub icmp_type: ::validated_data::Field<&'a str>,
            pub icmp_code: ::validated_data::Field<&'a str>,
            pub nexthop_group: ::validated_data::Field<&'a str>,
            pub tracked: ::validated_data::Field<bool>,
            pub dscp: ::validated_data::Field<&'a str>,
            pub vlan_number: ::validated_data::Field<i64>,
            pub vlan_mask: ::validated_data::Field<&'a str>,
            pub inner_vlan_number: ::validated_data::Field<i64>,
            pub inner_vlan_mask: ::validated_data::Field<&'a str>,
        }

        pub mod item {

            #[::validated_data::data_view(list)]
            pub struct SourcePorts<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(list)]
            pub struct DestinationPorts<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(list)]
            pub struct TcpFlags<'a, Mode> (::validated_data::Field<&'a str>);
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
