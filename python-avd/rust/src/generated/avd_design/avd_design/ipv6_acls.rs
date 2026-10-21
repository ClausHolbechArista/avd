// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.Ipv6Acls", schema = List(663), item_name = "AVDDesign.Ipv6AclsItem"))]
pub struct Ipv6Acls<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view(python(name = "AVDDesign.Ipv6AclsItemBase", schema = Dict(1601)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(1418))]
    pub name: ::data_views::Field<&'a str>,
    #[data_view(schema = List(662))]
    pub entries: ::data_views::RequiredValue<item::Entries<'a, Mode>, Mode>,
    #[data_view(schema = Bool(7))]
    pub counters_per_entry: ::data_views::Field<bool>,
    #[data_view(schema = List(201))]
    pub sequence_numbers: ::data_views::Field<item::SequenceNumbers<'a, Mode>>,
}

pub mod item {

    #[::data_views::data_view(list, python(name = "AVDDesign.Ipv6AclsItemBase.Entries", schema = List(662)))]
    pub struct Entries<'a, Mode> (::data_views::Field<entries::Item<'a, Mode>>);

    pub mod entries {

        #[::data_views::data_view(python(name = "AVDDesign.Ipv6AclsItemBase.EntriesItem", schema = Dict(1600)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(1422))]
            pub source: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(1422))]
            pub destination: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(422))]
            pub protocol: ::data_views::Field<&'a str>,
            #[data_view(schema = Int(200))]
            pub hop_limit: ::data_views::Field<i64>,
            #[data_view(schema = Str(361))]
            pub hop_limit_match: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(424))]
            pub dscp_mask: ::data_views::Field<&'a str>,
            #[data_view(schema = Int(174))]
            pub sequence: ::data_views::Field<i64>,
            #[data_view(schema = Str(357))]
            pub remark: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(358))]
            pub action: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(362))]
            pub source_ports_match: ::data_views::Field<&'a str>,
            #[data_view(schema = List(157))]
            pub source_ports: ::data_views::Field<item::SourcePorts<'a, Mode>>,
            #[data_view(schema = Str(362))]
            pub destination_ports_match: ::data_views::Field<&'a str>,
            #[data_view(schema = List(158))]
            pub destination_ports: ::data_views::Field<item::DestinationPorts<'a, Mode>>,
            #[data_view(schema = List(159))]
            pub tcp_flags: ::data_views::Field<item::TcpFlags<'a, Mode>>,
            #[data_view(schema = Bool(201))]
            pub copy_captive_portal: ::data_views::Field<bool>,
            #[data_view(schema = Bool(202))]
            pub log: ::data_views::Field<bool>,
            #[data_view(schema = Str(366))]
            pub icmp_type: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(367))]
            pub icmp_code: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(368))]
            pub nexthop_group: ::data_views::Field<&'a str>,
            #[data_view(schema = Bool(203))]
            pub tracked: ::data_views::Field<bool>,
            #[data_view(schema = Str(369))]
            pub dscp: ::data_views::Field<&'a str>,
            #[data_view(schema = Int(77))]
            pub vlan_number: ::data_views::Field<i64>,
            #[data_view(schema = Str(370))]
            pub vlan_mask: ::data_views::Field<&'a str>,
            #[data_view(schema = Int(77))]
            pub inner_vlan_number: ::data_views::Field<i64>,
            #[data_view(schema = Str(371))]
            pub inner_vlan_mask: ::data_views::Field<&'a str>,
        }

        pub mod item {

            #[::data_views::data_view(list, python(name = "AVDDesign.Ipv6AclsItemBase.EntriesItem.SourcePorts", schema = List(157)))]
            pub struct SourcePorts<'a, Mode> (::data_views::Field<&'a str>);

            #[::data_views::data_view(list, python(name = "AVDDesign.Ipv6AclsItemBase.EntriesItem.DestinationPorts", schema = List(158)))]
            pub struct DestinationPorts<'a, Mode> (::data_views::Field<&'a str>);

            #[::data_views::data_view(list, python(name = "AVDDesign.Ipv6AclsItemBase.EntriesItem.TcpFlags", schema = List(159)))]
            pub struct TcpFlags<'a, Mode> (::data_views::Field<&'a str>);
        }
    }

    #[::data_views::data_view(indexed_list, primary_key(sequence), python(name = "AVDDesign.Ipv6AclsItemBase.SequenceNumbers", schema = List(201), item_name = "AVDDesign.Ipv6AclsItemBase.SequenceNumbersItem"))]
    pub struct SequenceNumbers<'a, Mode> (::data_views::Field<sequence_numbers::Item<'a, Mode>>);

    pub mod sequence_numbers {

        #[::data_views::data_view(python(name = "AVDDesign.Ipv6AclsItemBase.SequenceNumbersItemBase", schema = Dict(464)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Int(5))]
            pub sequence: ::data_views::Field<i64>,
            #[data_view(schema = Str(425))]
            pub action: ::data_views::RequiredValue<&'a str, Mode>,
        }
    }
}
