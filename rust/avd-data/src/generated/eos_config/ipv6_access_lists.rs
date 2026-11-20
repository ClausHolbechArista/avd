// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Ipv6AccessLists<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view]
pub struct Item<'a, Mode> {
    pub name: ::data_views::Field<&'a str>,
    pub counters_per_entry: ::data_views::Field<bool>,
    pub entries: ::data_views::Field<item::Entries<'a, Mode>>,
    pub sequence_numbers: ::data_views::Field<item::SequenceNumbers<'a, Mode>>,
}

pub mod item {

    #[::data_views::data_view(list)]
    pub struct Entries<'a, Mode> (::data_views::Field<entries::Item<'a, Mode>>);

    pub mod entries {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub protocol: ::data_views::Field<&'a str>,
            pub source: ::data_views::Field<&'a str>,
            pub destination: ::data_views::Field<&'a str>,
            pub hop_limit: ::data_views::Field<i64>,
            pub hop_limit_match: ::data_views::Field<&'a str>,
            pub dscp_mask: ::data_views::Field<&'a str>,
            pub sequence: ::data_views::Field<i64>,
            pub remark: ::data_views::Field<&'a str>,
            pub action: ::data_views::Field<&'a str>,
            pub source_ports_match: ::data_views::Field<&'a str>,
            pub source_ports: ::data_views::Field<item::SourcePorts<'a, Mode>>,
            pub destination_ports_match: ::data_views::Field<&'a str>,
            pub destination_ports: ::data_views::Field<item::DestinationPorts<'a, Mode>>,
            pub tcp_flags: ::data_views::Field<item::TcpFlags<'a, Mode>>,
            pub copy_captive_portal: ::data_views::Field<bool>,
            pub log: ::data_views::Field<bool>,
            pub icmp_type: ::data_views::Field<&'a str>,
            pub icmp_code: ::data_views::Field<&'a str>,
            pub nexthop_group: ::data_views::Field<&'a str>,
            pub tracked: ::data_views::Field<bool>,
            pub dscp: ::data_views::Field<&'a str>,
            pub vlan_number: ::data_views::Field<i64>,
            pub vlan_mask: ::data_views::Field<&'a str>,
            pub inner_vlan_number: ::data_views::Field<i64>,
            pub inner_vlan_mask: ::data_views::Field<&'a str>,
        }

        pub mod item {

            #[::data_views::data_view(list)]
            pub struct SourcePorts<'a, Mode> (::data_views::Field<&'a str>);

            #[::data_views::data_view(list)]
            pub struct DestinationPorts<'a, Mode> (::data_views::Field<&'a str>);

            #[::data_views::data_view(list)]
            pub struct TcpFlags<'a, Mode> (::data_views::Field<&'a str>);
        }
    }

    #[::data_views::data_view(indexed_list, primary_key(sequence))]
    pub struct SequenceNumbers<'a, Mode> (::data_views::Field<sequence_numbers::Item<'a, Mode>>);

    pub mod sequence_numbers {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub sequence: ::data_views::Field<i64>,
            pub action: ::data_views::RequiredValue<&'a str, Mode>,
        }
    }
}
