// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct SpanningTree<'a, Mode> {
    pub root_super: ::data_views::Field<bool>,
    pub edge_port: ::data_views::Field<EdgePort<'a, Mode>>,
    pub mode: ::data_views::Field<&'a str>,
    pub bpduguard_rate_limit: ::data_views::Field<BpduguardRateLimit<'a, Mode>>,
    pub rstp_priority: ::data_views::Field<i64>,
    pub mst: ::data_views::Field<Mst<'a, Mode>>,
    pub mst_instances: ::data_views::Field<MstInstances<'a, Mode>>,
    pub no_spanning_tree_vlan: ::data_views::Field<&'a str>,
    pub rapid_pvst_instances: ::data_views::Field<RapidPvstInstances<'a, Mode>>,
    pub port_id_allocation_port_channel_range: ::data_views::Field<PortIdAllocationPortChannelRange<'a, Mode>>,
    pub loop_guard_default: ::data_views::Field<bool>,
}

#[::data_views::data_view]
pub struct EdgePort<'a, Mode> {
    pub bpdufilter_default: ::data_views::Field<bool>,
    pub bpduguard_default: ::data_views::Field<bool>,
}

#[::data_views::data_view]
pub struct BpduguardRateLimit<'a, Mode> {
    pub default: ::data_views::Field<bool>,
    pub count: ::data_views::Field<i64>,
}

#[::data_views::data_view]
pub struct Mst<'a, Mode> {
    pub pvst_border: ::data_views::Field<bool>,
    pub configuration: ::data_views::Field<mst::Configuration<'a, Mode>>,
}

pub mod mst {

    #[::data_views::data_view]
    pub struct Configuration<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub revision: ::data_views::Field<i64>,
        pub instances: ::data_views::Field<configuration::Instances<'a, Mode>>,
    }

    pub mod configuration {

        #[::data_views::data_view(indexed_list, primary_key(id))]
        pub struct Instances<'a, Mode> (::data_views::Field<instances::Item<'a, Mode>>);

        pub mod instances {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub id: ::data_views::Field<i64>,
                pub vlans: ::data_views::Field<&'a str>,
            }
        }
    }
}

#[::data_views::data_view(indexed_list, primary_key(id))]
pub struct MstInstances<'a, Mode> (::data_views::Field<mst_instances::Item<'a, Mode>>);

pub mod mst_instances {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub id: ::data_views::Field<&'a str>,
        pub priority: ::data_views::Field<i64>,
    }
}

#[::data_views::data_view(indexed_list, primary_key(id))]
pub struct RapidPvstInstances<'a, Mode> (::data_views::Field<rapid_pvst_instances::Item<'a, Mode>>);

pub mod rapid_pvst_instances {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub id: ::data_views::Field<&'a str>,
        pub priority: ::data_views::Field<i64>,
    }
}

#[::data_views::data_view(python(name = "EOSConfig.SpanningTree.PortIdAllocationPortChannelRange", schema = Dict(1253)))]
pub struct PortIdAllocationPortChannelRange<'a, Mode> {
    #[data_view(schema = Int(467))]
    pub minimum: ::data_views::RequiredValue<i64, Mode>,
    #[data_view(schema = Int(468))]
    pub maximum: ::data_views::RequiredValue<i64, Mode>,
}
