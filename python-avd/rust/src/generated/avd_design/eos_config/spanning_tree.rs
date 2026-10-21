// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view]
pub struct SpanningTree<'a, Mode> {
    pub root_super: ::validated_data::Field<bool>,
    pub edge_port: ::validated_data::Field<EdgePort<'a, Mode>>,
    pub mode: ::validated_data::Field<&'a str>,
    pub bpduguard_rate_limit: ::validated_data::Field<BpduguardRateLimit<'a, Mode>>,
    pub rstp_priority: ::validated_data::Field<i64>,
    pub mst: ::validated_data::Field<Mst<'a, Mode>>,
    pub mst_instances: ::validated_data::Field<MstInstances<'a, Mode>>,
    pub no_spanning_tree_vlan: ::validated_data::Field<&'a str>,
    pub rapid_pvst_instances: ::validated_data::Field<RapidPvstInstances<'a, Mode>>,
    pub port_id_allocation_port_channel_range: ::validated_data::Field<PortIdAllocationPortChannelRange<'a, Mode>>,
    pub loop_guard_default: ::validated_data::Field<bool>,
}

#[::validated_data::data_view]
pub struct EdgePort<'a, Mode> {
    pub bpdufilter_default: ::validated_data::Field<bool>,
    pub bpduguard_default: ::validated_data::Field<bool>,
}

#[::validated_data::data_view]
pub struct BpduguardRateLimit<'a, Mode> {
    pub default: ::validated_data::Field<bool>,
    pub count: ::validated_data::Field<i64>,
}

#[::validated_data::data_view]
pub struct Mst<'a, Mode> {
    pub pvst_border: ::validated_data::Field<bool>,
    pub configuration: ::validated_data::Field<mst::Configuration<'a, Mode>>,
}

pub mod mst {

    #[::validated_data::data_view]
    pub struct Configuration<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub revision: ::validated_data::Field<i64>,
        pub instances: ::validated_data::Field<configuration::Instances<'a, Mode>>,
    }

    pub mod configuration {

        #[::validated_data::data_view(indexed_list, primary_key(id))]
        pub struct Instances<'a, Mode> (::validated_data::Field<instances::Item<'a, Mode>>);

        pub mod instances {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub id: ::validated_data::Field<i64>,
                pub vlans: ::validated_data::Field<&'a str>,
            }
        }
    }
}

#[::validated_data::data_view(indexed_list, primary_key(id))]
pub struct MstInstances<'a, Mode> (::validated_data::Field<mst_instances::Item<'a, Mode>>);

pub mod mst_instances {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub id: ::validated_data::Field<&'a str>,
        pub priority: ::validated_data::Field<i64>,
    }
}

#[::validated_data::data_view(indexed_list, primary_key(id))]
pub struct RapidPvstInstances<'a, Mode> (::validated_data::Field<rapid_pvst_instances::Item<'a, Mode>>);

pub mod rapid_pvst_instances {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub id: ::validated_data::Field<&'a str>,
        pub priority: ::validated_data::Field<i64>,
    }
}

#[::validated_data::data_view(python(name = "EOSConfig.SpanningTree.PortIdAllocationPortChannelRange", schema = Dict(1253)))]
pub struct PortIdAllocationPortChannelRange<'a, Mode> {
    #[data_view(schema = Int(467))]
    pub minimum: ::validated_data::RequiredValue<i64, Mode>,
    #[data_view(schema = Int(468))]
    pub maximum: ::validated_data::RequiredValue<i64, Mode>,
}
