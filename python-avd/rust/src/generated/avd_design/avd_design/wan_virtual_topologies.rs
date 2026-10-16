// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.WanVirtualTopologies.Vrfs", schema = List(794), item_name = "AVDDesign.WanVirtualTopologies.VrfsItem"))]
pub struct Vrfs<'a, Mode> (::validated_data::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::validated_data::data_view(python(name = "AVDDesign.WanVirtualTopologies.VrfsItemBase", schema = Dict(1789)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(25))]
        pub name: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(1689))]
        pub policy: ::validated_data::Field<&'a str>,
        #[data_view(schema = Int(645))]
        pub wan_vni: ::validated_data::RequiredValue<i64, Mode>,
    }
}

#[::validated_data::data_view(python(name = "AVDDesign.WanVirtualTopologies.ControlPlaneVirtualTopology", schema = Dict(1793)))]
pub struct ControlPlaneVirtualTopology<'a, Mode> {
    #[data_view(schema = Str(1690))]
    pub name: ::validated_data::Field<&'a str>,
    #[data_view(schema = Str(1691))]
    pub application_profile: ::validated_data::Field<&'a str>,
    #[data_view(schema = Int(359))]
    pub traffic_class: ::validated_data::Field<i64>,
    #[data_view(schema = Int(358))]
    pub dscp: ::validated_data::Field<i64>,
    #[data_view(schema = Bool(682))]
    pub lowest_hop_count: ::validated_data::Field<bool>,
    #[data_view(schema = Dict(1790))]
    pub constraints: ::validated_data::Field<control_plane_virtual_topology::Constraints<'a, Mode>>,
    #[data_view(schema = Dict(878))]
    pub outlier_elimination: ::validated_data::Field<super::super::eos_config::router_adaptive_virtual_topology::profiles::item::OutlierElimination<'a, Mode>>,
    #[data_view(schema = Dict(876))]
    pub metric_order: ::validated_data::Field<super::super::eos_config::router_adaptive_virtual_topology::profiles::item::MetricOrder<'a, Mode>>,
    #[data_view(schema = List(796))]
    pub path_groups: ::validated_data::Field<control_plane_virtual_topology::PathGroups<'a, Mode>>,
    #[data_view(schema = Dict(1792))]
    pub internet_exit: ::validated_data::Field<control_plane_virtual_topology::InternetExit<'a, Mode>>,
}

pub mod control_plane_virtual_topology {

    #[::validated_data::data_view(python(name = "AVDDesign.WanVirtualTopologies.ControlPlaneVirtualTopology.Constraints", schema = Dict(1790)))]
    pub struct Constraints<'a, Mode> {
        #[data_view(schema = Int(444))]
        pub jitter: ::validated_data::Field<i64>,
        #[data_view(schema = Int(445))]
        pub latency: ::validated_data::Field<i64>,
        #[data_view(schema = Str(891))]
        pub loss_rate: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view(list, python(name = "AVDDesign.WanVirtualTopologies.ControlPlaneVirtualTopology.PathGroups", schema = List(796)))]
    pub struct PathGroups<'a, Mode> (::validated_data::Field<path_groups::Item<'a, Mode>>);

    pub mod path_groups {

        #[::validated_data::data_view(python(name = "AVDDesign.WanVirtualTopologies.ControlPlaneVirtualTopology.PathGroupsItem", schema = Dict(1791)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = List(795))]
            pub names: ::validated_data::RequiredValue<item::Names<'a, Mode>, Mode>,
            #[data_view(schema = Str(1692))]
            pub preference: ::validated_data::Field<&'a str>,
        }

        pub mod item {

            #[::validated_data::data_view(list, python(name = "AVDDesign.WanVirtualTopologies.ControlPlaneVirtualTopology.PathGroupsItem.Names", schema = List(795)))]
            pub struct Names<'a, Mode> (::validated_data::Field<&'a str>);
        }
    }

    #[::validated_data::data_view(python(name = "AVDDesign.WanVirtualTopologies.ControlPlaneVirtualTopology.InternetExit", schema = Dict(1792)))]
    pub struct InternetExit<'a, Mode> {
        #[data_view(schema = Str(1693))]
        pub policy: ::validated_data::Field<&'a str>,
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.WanVirtualTopologies.Policies", schema = List(798), item_name = "AVDDesign.WanVirtualTopologies.PoliciesItem"))]
pub struct Policies<'a, Mode> (::validated_data::Field<policies::Item<'a, Mode>>);

pub mod policies {

    #[::validated_data::data_view(python(name = "AVDDesign.WanVirtualTopologies.PoliciesItemBase", schema = Dict(1796)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(1694))]
        pub name: ::validated_data::Field<&'a str>,
        #[data_view(schema = List(797))]
        pub application_virtual_topologies: ::validated_data::Field<item::ApplicationVirtualTopologies<'a, Mode>>,
        #[data_view(schema = Dict(1795))]
        pub default_virtual_topology: ::validated_data::RequiredValue<item::DefaultVirtualTopology<'a, Mode>, Mode>,
    }

    pub mod item {

        #[::validated_data::data_view(indexed_list, primary_key(application_profile), python(name = "AVDDesign.WanVirtualTopologies.PoliciesItemBase.ApplicationVirtualTopologies", schema = List(797), item_name = "AVDDesign.WanVirtualTopologies.PoliciesItemBase.ApplicationVirtualTopologiesItem"))]
        pub struct ApplicationVirtualTopologies<'a, Mode> (::validated_data::Field<application_virtual_topologies::Item<'a, Mode>>);

        pub mod application_virtual_topologies {

            #[::validated_data::data_view(python(name = "AVDDesign.WanVirtualTopologies.PoliciesItemBase.ApplicationVirtualTopologiesItemBase", schema = Dict(1794)))]
            pub struct Item<'a, Mode> {
                #[data_view(schema = Str(1695))]
                pub application_profile: ::validated_data::Field<&'a str>,
                #[data_view(schema = Str(1696))]
                pub name: ::validated_data::Field<&'a str>,
                #[data_view(schema = Int(646))]
                pub id: ::validated_data::Field<i64>,
                #[data_view(schema = Int(359))]
                pub traffic_class: ::validated_data::Field<i64>,
                #[data_view(schema = Int(358))]
                pub dscp: ::validated_data::Field<i64>,
                #[data_view(schema = Bool(682))]
                pub lowest_hop_count: ::validated_data::Field<bool>,
                #[data_view(schema = Dict(1790))]
                pub constraints: ::validated_data::Field<item::Constraints<'a, Mode>>,
                #[data_view(schema = Dict(878))]
                pub outlier_elimination: ::validated_data::Field<super::super::super::super::super::eos_config::router_adaptive_virtual_topology::profiles::item::OutlierElimination<'a, Mode>>,
                #[data_view(schema = Dict(876))]
                pub metric_order: ::validated_data::Field<super::super::super::super::super::eos_config::router_adaptive_virtual_topology::profiles::item::MetricOrder<'a, Mode>>,
                #[data_view(schema = List(796))]
                pub path_groups: ::validated_data::Field<item::PathGroups<'a, Mode>>,
                #[data_view(schema = Dict(1792))]
                pub internet_exit: ::validated_data::Field<item::InternetExit<'a, Mode>>,
            }

            pub mod item {

                #[::validated_data::data_view(python(name = "AVDDesign.WanVirtualTopologies.PoliciesItemBase.ApplicationVirtualTopologiesItemBase.Constraints", schema = Dict(1790)))]
                pub struct Constraints<'a, Mode> {
                    #[data_view(schema = Int(444))]
                    pub jitter: ::validated_data::Field<i64>,
                    #[data_view(schema = Int(445))]
                    pub latency: ::validated_data::Field<i64>,
                    #[data_view(schema = Str(891))]
                    pub loss_rate: ::validated_data::Field<&'a str>,
                }

                #[::validated_data::data_view(list, python(name = "AVDDesign.WanVirtualTopologies.PoliciesItemBase.ApplicationVirtualTopologiesItemBase.PathGroups", schema = List(796)))]
                pub struct PathGroups<'a, Mode> (::validated_data::Field<path_groups::Item<'a, Mode>>);

                pub mod path_groups {

                    #[::validated_data::data_view(python(name = "AVDDesign.WanVirtualTopologies.PoliciesItemBase.ApplicationVirtualTopologiesItemBase.PathGroupsItem", schema = Dict(1791)))]
                    pub struct Item<'a, Mode> {
                        #[data_view(schema = List(795))]
                        pub names: ::validated_data::RequiredValue<item::Names<'a, Mode>, Mode>,
                        #[data_view(schema = Str(1692))]
                        pub preference: ::validated_data::Field<&'a str>,
                    }

                    pub mod item {

                        #[::validated_data::data_view(list, python(name = "AVDDesign.WanVirtualTopologies.PoliciesItemBase.ApplicationVirtualTopologiesItemBase.PathGroupsItem.Names", schema = List(795)))]
                        pub struct Names<'a, Mode> (::validated_data::Field<&'a str>);
                    }
                }

                #[::validated_data::data_view(python(name = "AVDDesign.WanVirtualTopologies.PoliciesItemBase.ApplicationVirtualTopologiesItemBase.InternetExit", schema = Dict(1792)))]
                pub struct InternetExit<'a, Mode> {
                    #[data_view(schema = Str(1693))]
                    pub policy: ::validated_data::Field<&'a str>,
                }
            }
        }

        #[::validated_data::data_view(python(name = "AVDDesign.WanVirtualTopologies.PoliciesItemBase.DefaultVirtualTopology", schema = Dict(1795)))]
        pub struct DefaultVirtualTopology<'a, Mode> {
            #[data_view(schema = Str(1697))]
            pub name: ::validated_data::Field<&'a str>,
            #[data_view(schema = Bool(683))]
            pub drop_unmatched: ::validated_data::Field<bool>,
            #[data_view(schema = Int(359))]
            pub traffic_class: ::validated_data::Field<i64>,
            #[data_view(schema = Int(358))]
            pub dscp: ::validated_data::Field<i64>,
            #[data_view(schema = Bool(682))]
            pub lowest_hop_count: ::validated_data::Field<bool>,
            #[data_view(schema = Dict(1790))]
            pub constraints: ::validated_data::Field<default_virtual_topology::Constraints<'a, Mode>>,
            #[data_view(schema = Dict(878))]
            pub outlier_elimination: ::validated_data::Field<super::super::super::super::eos_config::router_adaptive_virtual_topology::profiles::item::OutlierElimination<'a, Mode>>,
            #[data_view(schema = Dict(876))]
            pub metric_order: ::validated_data::Field<super::super::super::super::eos_config::router_adaptive_virtual_topology::profiles::item::MetricOrder<'a, Mode>>,
            #[data_view(schema = List(796))]
            pub path_groups: ::validated_data::Field<default_virtual_topology::PathGroups<'a, Mode>>,
            #[data_view(schema = Dict(1792))]
            pub internet_exit: ::validated_data::Field<default_virtual_topology::InternetExit<'a, Mode>>,
        }

        pub mod default_virtual_topology {

            #[::validated_data::data_view(python(name = "AVDDesign.WanVirtualTopologies.PoliciesItemBase.DefaultVirtualTopology.Constraints", schema = Dict(1790)))]
            pub struct Constraints<'a, Mode> {
                #[data_view(schema = Int(444))]
                pub jitter: ::validated_data::Field<i64>,
                #[data_view(schema = Int(445))]
                pub latency: ::validated_data::Field<i64>,
                #[data_view(schema = Str(891))]
                pub loss_rate: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view(list, python(name = "AVDDesign.WanVirtualTopologies.PoliciesItemBase.DefaultVirtualTopology.PathGroups", schema = List(796)))]
            pub struct PathGroups<'a, Mode> (::validated_data::Field<path_groups::Item<'a, Mode>>);

            pub mod path_groups {

                #[::validated_data::data_view(python(name = "AVDDesign.WanVirtualTopologies.PoliciesItemBase.DefaultVirtualTopology.PathGroupsItem", schema = Dict(1791)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = List(795))]
                    pub names: ::validated_data::RequiredValue<item::Names<'a, Mode>, Mode>,
                    #[data_view(schema = Str(1692))]
                    pub preference: ::validated_data::Field<&'a str>,
                }

                pub mod item {

                    #[::validated_data::data_view(list, python(name = "AVDDesign.WanVirtualTopologies.PoliciesItemBase.DefaultVirtualTopology.PathGroupsItem.Names", schema = List(795)))]
                    pub struct Names<'a, Mode> (::validated_data::Field<&'a str>);
                }
            }

            #[::validated_data::data_view(python(name = "AVDDesign.WanVirtualTopologies.PoliciesItemBase.DefaultVirtualTopology.InternetExit", schema = Dict(1792)))]
            pub struct InternetExit<'a, Mode> {
                #[data_view(schema = Str(1693))]
                pub policy: ::validated_data::Field<&'a str>,
            }
        }
    }
}
