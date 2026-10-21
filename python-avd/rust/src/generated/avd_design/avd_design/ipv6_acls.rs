// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.Ipv6Acls", schema = List(663), item_name = "AVDDesign.Ipv6AclsItem"))]
pub struct Ipv6Acls<'a, Mode> (::validated_data::Field<Item<'a, Mode>>);

#[::validated_data::data_view(python(name = "AVDDesign.Ipv6AclsItemBase", schema = Dict(1601)))]
pub struct Item<'a, Mode> {
    #[data_view(schema = Str(1418))]
    pub name: ::validated_data::Field<&'a str>,
    #[data_view(schema = List(662))]
    pub entries: ::validated_data::RequiredValue<item::Entries<'a, Mode>, Mode>,
    #[data_view(schema = Bool(7))]
    pub counters_per_entry: ::validated_data::Field<bool>,
    #[data_view(schema = List(201))]
    pub sequence_numbers: ::validated_data::Field<item::SequenceNumbers<'a, Mode>>,
}

pub mod item {

    #[::validated_data::data_view(list, python(name = "AVDDesign.Ipv6AclsItemBase.Entries", schema = List(662)))]
    pub struct Entries<'a, Mode> (::validated_data::Field<entries::Item<'a, Mode>>);

    pub mod entries {

        #[::validated_data::data_view(python(name = "AVDDesign.Ipv6AclsItemBase.EntriesItem", schema = Dict(1600)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(1422))]
            pub source: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(1422))]
            pub destination: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(422))]
            pub protocol: ::validated_data::Field<&'a str>,
            #[data_view(schema = Int(200))]
            pub hop_limit: ::validated_data::Field<i64>,
            #[data_view(schema = Str(361))]
            pub hop_limit_match: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(424))]
            pub dscp_mask: ::validated_data::Field<&'a str>,
            #[data_view(schema = Int(174))]
            pub sequence: ::validated_data::Field<i64>,
            #[data_view(schema = Str(357))]
            pub remark: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(358))]
            pub action: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(362))]
            pub source_ports_match: ::validated_data::Field<&'a str>,
            #[data_view(schema = List(157))]
            pub source_ports: ::validated_data::Field<item::SourcePorts<'a, Mode>>,
            #[data_view(schema = Str(362))]
            pub destination_ports_match: ::validated_data::Field<&'a str>,
            #[data_view(schema = List(158))]
            pub destination_ports: ::validated_data::Field<item::DestinationPorts<'a, Mode>>,
            #[data_view(schema = List(159))]
            pub tcp_flags: ::validated_data::Field<item::TcpFlags<'a, Mode>>,
            #[data_view(schema = Bool(201))]
            pub copy_captive_portal: ::validated_data::Field<bool>,
            #[data_view(schema = Bool(202))]
            pub log: ::validated_data::Field<bool>,
            #[data_view(schema = Str(366))]
            pub icmp_type: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(367))]
            pub icmp_code: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(368))]
            pub nexthop_group: ::validated_data::Field<&'a str>,
            #[data_view(schema = Bool(203))]
            pub tracked: ::validated_data::Field<bool>,
            #[data_view(schema = Str(369))]
            pub dscp: ::validated_data::Field<&'a str>,
            #[data_view(schema = Int(77))]
            pub vlan_number: ::validated_data::Field<i64>,
            #[data_view(schema = Str(370))]
            pub vlan_mask: ::validated_data::Field<&'a str>,
            #[data_view(schema = Int(77))]
            pub inner_vlan_number: ::validated_data::Field<i64>,
            #[data_view(schema = Str(371))]
            pub inner_vlan_mask: ::validated_data::Field<&'a str>,
        }

        pub mod item {

            #[::validated_data::data_view(list, python(name = "AVDDesign.Ipv6AclsItemBase.EntriesItem.SourcePorts", schema = List(157)))]
            pub struct SourcePorts<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(list, python(name = "AVDDesign.Ipv6AclsItemBase.EntriesItem.DestinationPorts", schema = List(158)))]
            pub struct DestinationPorts<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view(list, python(name = "AVDDesign.Ipv6AclsItemBase.EntriesItem.TcpFlags", schema = List(159)))]
            pub struct TcpFlags<'a, Mode> (::validated_data::Field<&'a str>);
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(sequence), python(name = "AVDDesign.Ipv6AclsItemBase.SequenceNumbers", schema = List(201), item_name = "AVDDesign.Ipv6AclsItemBase.SequenceNumbersItem"))]
    pub struct SequenceNumbers<'a, Mode> (::validated_data::Field<sequence_numbers::Item<'a, Mode>>);

    pub mod sequence_numbers {

        #[::validated_data::data_view(python(name = "AVDDesign.Ipv6AclsItemBase.SequenceNumbersItemBase", schema = Dict(464)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Int(5))]
            pub sequence: ::validated_data::Field<i64>,
            #[data_view(schema = Str(425))]
            pub action: ::validated_data::RequiredValue<&'a str, Mode>,
        }
    }
}
