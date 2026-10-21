// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "AVDDesign.WanVirtualTopologies", schema = Dict(1797)))]
pub struct WanVirtualTopologies<'a, Mode> {
    #[data_view(schema = List(794))]
    pub vrfs: ::data_views::Field<Vrfs<'a, Mode>>,
    #[data_view(schema = Dict(1793))]
    pub control_plane_virtual_topology: ::data_views::Field<ControlPlaneVirtualTopology<'a, Mode>>,
    #[data_view(schema = List(798))]
    pub policies: ::data_views::Field<Policies<'a, Mode>>,
}

#[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.WanVirtualTopologies.Vrfs", schema = List(794), item_name = "AVDDesign.WanVirtualTopologies.VrfsItem"))]
pub struct Vrfs<'a, Mode> (::data_views::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::data_views::data_view(python(name = "AVDDesign.WanVirtualTopologies.VrfsItemBase", schema = Dict(1789)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(25))]
        pub name: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(1689))]
        pub policy: ::data_views::Field<&'a str>,
        #[data_view(schema = Int(645))]
        pub wan_vni: ::data_views::RequiredValue<i64, Mode>,
    }
}

#[::data_views::data_view(python(name = "AVDDesign.WanVirtualTopologies.ControlPlaneVirtualTopology", schema = Dict(1793)))]
pub struct ControlPlaneVirtualTopology<'a, Mode> {
    #[data_view(schema = Str(1690))]
    pub name: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1691))]
    pub application_profile: ::data_views::Field<&'a str>,
    #[data_view(schema = Int(359))]
    pub traffic_class: ::data_views::Field<i64>,
    #[data_view(schema = Int(358))]
    pub dscp: ::data_views::Field<i64>,
    #[data_view(schema = Bool(682))]
    pub lowest_hop_count: ::data_views::Field<bool>,
    #[data_view(schema = Dict(1790))]
    pub constraints: ::data_views::Field<control_plane_virtual_topology::Constraints<'a, Mode>>,
    #[data_view(schema = Dict(878))]
    pub outlier_elimination: ::data_views::Field<super::super::eos_config::router_adaptive_virtual_topology::profiles::item::OutlierElimination<'a, Mode>>,
    #[data_view(schema = Dict(876))]
    pub metric_order: ::data_views::Field<super::super::eos_config::router_adaptive_virtual_topology::profiles::item::MetricOrder<'a, Mode>>,
    #[data_view(schema = List(796))]
    pub path_groups: ::data_views::Field<control_plane_virtual_topology::PathGroups<'a, Mode>>,
    #[data_view(schema = Dict(1792))]
    pub internet_exit: ::data_views::Field<control_plane_virtual_topology::InternetExit<'a, Mode>>,
}

pub mod control_plane_virtual_topology {

    #[::data_views::data_view(python(name = "AVDDesign.WanVirtualTopologies.ControlPlaneVirtualTopology.Constraints", schema = Dict(1790)))]
    pub struct Constraints<'a, Mode> {
        #[data_view(schema = Int(444))]
        pub jitter: ::data_views::Field<i64>,
        #[data_view(schema = Int(445))]
        pub latency: ::data_views::Field<i64>,
        #[data_view(schema = Str(891))]
        pub loss_rate: ::data_views::Field<&'a str>,
    }

    #[::data_views::data_view(list, python(name = "AVDDesign.WanVirtualTopologies.ControlPlaneVirtualTopology.PathGroups", schema = List(796)))]
    pub struct PathGroups<'a, Mode> (::data_views::Field<path_groups::Item<'a, Mode>>);

    pub mod path_groups {

        #[::data_views::data_view(python(name = "AVDDesign.WanVirtualTopologies.ControlPlaneVirtualTopology.PathGroupsItem", schema = Dict(1791)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = List(795))]
            pub names: ::data_views::RequiredValue<item::Names<'a, Mode>, Mode>,
            #[data_view(schema = Str(1692))]
            pub preference: ::data_views::Field<&'a str>,
        }

        pub mod item {

            #[::data_views::data_view(list, python(name = "AVDDesign.WanVirtualTopologies.ControlPlaneVirtualTopology.PathGroupsItem.Names", schema = List(795)))]
            pub struct Names<'a, Mode> (::data_views::Field<&'a str>);
        }
    }

    #[::data_views::data_view(python(name = "AVDDesign.WanVirtualTopologies.ControlPlaneVirtualTopology.InternetExit", schema = Dict(1792)))]
    pub struct InternetExit<'a, Mode> {
        #[data_view(schema = Str(1693))]
        pub policy: ::data_views::Field<&'a str>,
    }
}

#[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.WanVirtualTopologies.Policies", schema = List(798), item_name = "AVDDesign.WanVirtualTopologies.PoliciesItem"))]
pub struct Policies<'a, Mode> (::data_views::Field<policies::Item<'a, Mode>>);

pub mod policies {

    #[::data_views::data_view(python(name = "AVDDesign.WanVirtualTopologies.PoliciesItemBase", schema = Dict(1796)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(1694))]
        pub name: ::data_views::Field<&'a str>,
        #[data_view(schema = List(797))]
        pub application_virtual_topologies: ::data_views::Field<item::ApplicationVirtualTopologies<'a, Mode>>,
        #[data_view(schema = Dict(1795))]
        pub default_virtual_topology: ::data_views::RequiredValue<item::DefaultVirtualTopology<'a, Mode>, Mode>,
    }

    pub mod item {

        #[::data_views::data_view(indexed_list, primary_key(application_profile), python(name = "AVDDesign.WanVirtualTopologies.PoliciesItemBase.ApplicationVirtualTopologies", schema = List(797), item_name = "AVDDesign.WanVirtualTopologies.PoliciesItemBase.ApplicationVirtualTopologiesItem"))]
        pub struct ApplicationVirtualTopologies<'a, Mode> (::data_views::Field<application_virtual_topologies::Item<'a, Mode>>);

        pub mod application_virtual_topologies {

            #[::data_views::data_view(python(name = "AVDDesign.WanVirtualTopologies.PoliciesItemBase.ApplicationVirtualTopologiesItemBase", schema = Dict(1794)))]
            pub struct Item<'a, Mode> {
                #[data_view(schema = Str(1695))]
                pub application_profile: ::data_views::Field<&'a str>,
                #[data_view(schema = Str(1696))]
                pub name: ::data_views::Field<&'a str>,
                #[data_view(schema = Int(646))]
                pub id: ::data_views::Field<i64>,
                #[data_view(schema = Int(359))]
                pub traffic_class: ::data_views::Field<i64>,
                #[data_view(schema = Int(358))]
                pub dscp: ::data_views::Field<i64>,
                #[data_view(schema = Bool(682))]
                pub lowest_hop_count: ::data_views::Field<bool>,
                #[data_view(schema = Dict(1790))]
                pub constraints: ::data_views::Field<item::Constraints<'a, Mode>>,
                #[data_view(schema = Dict(878))]
                pub outlier_elimination: ::data_views::Field<super::super::super::super::super::eos_config::router_adaptive_virtual_topology::profiles::item::OutlierElimination<'a, Mode>>,
                #[data_view(schema = Dict(876))]
                pub metric_order: ::data_views::Field<super::super::super::super::super::eos_config::router_adaptive_virtual_topology::profiles::item::MetricOrder<'a, Mode>>,
                #[data_view(schema = List(796))]
                pub path_groups: ::data_views::Field<item::PathGroups<'a, Mode>>,
                #[data_view(schema = Dict(1792))]
                pub internet_exit: ::data_views::Field<item::InternetExit<'a, Mode>>,
            }

            pub mod item {

                #[::data_views::data_view(python(name = "AVDDesign.WanVirtualTopologies.PoliciesItemBase.ApplicationVirtualTopologiesItemBase.Constraints", schema = Dict(1790)))]
                pub struct Constraints<'a, Mode> {
                    #[data_view(schema = Int(444))]
                    pub jitter: ::data_views::Field<i64>,
                    #[data_view(schema = Int(445))]
                    pub latency: ::data_views::Field<i64>,
                    #[data_view(schema = Str(891))]
                    pub loss_rate: ::data_views::Field<&'a str>,
                }

                #[::data_views::data_view(list, python(name = "AVDDesign.WanVirtualTopologies.PoliciesItemBase.ApplicationVirtualTopologiesItemBase.PathGroups", schema = List(796)))]
                pub struct PathGroups<'a, Mode> (::data_views::Field<path_groups::Item<'a, Mode>>);

                pub mod path_groups {

                    #[::data_views::data_view(python(name = "AVDDesign.WanVirtualTopologies.PoliciesItemBase.ApplicationVirtualTopologiesItemBase.PathGroupsItem", schema = Dict(1791)))]
                    pub struct Item<'a, Mode> {
                        #[data_view(schema = List(795))]
                        pub names: ::data_views::RequiredValue<item::Names<'a, Mode>, Mode>,
                        #[data_view(schema = Str(1692))]
                        pub preference: ::data_views::Field<&'a str>,
                    }

                    pub mod item {

                        #[::data_views::data_view(list, python(name = "AVDDesign.WanVirtualTopologies.PoliciesItemBase.ApplicationVirtualTopologiesItemBase.PathGroupsItem.Names", schema = List(795)))]
                        pub struct Names<'a, Mode> (::data_views::Field<&'a str>);
                    }
                }

                #[::data_views::data_view(python(name = "AVDDesign.WanVirtualTopologies.PoliciesItemBase.ApplicationVirtualTopologiesItemBase.InternetExit", schema = Dict(1792)))]
                pub struct InternetExit<'a, Mode> {
                    #[data_view(schema = Str(1693))]
                    pub policy: ::data_views::Field<&'a str>,
                }
            }
        }

        #[::data_views::data_view(python(name = "AVDDesign.WanVirtualTopologies.PoliciesItemBase.DefaultVirtualTopology", schema = Dict(1795)))]
        pub struct DefaultVirtualTopology<'a, Mode> {
            #[data_view(schema = Str(1697))]
            pub name: ::data_views::Field<&'a str>,
            #[data_view(schema = Bool(683))]
            pub drop_unmatched: ::data_views::Field<bool>,
            #[data_view(schema = Int(359))]
            pub traffic_class: ::data_views::Field<i64>,
            #[data_view(schema = Int(358))]
            pub dscp: ::data_views::Field<i64>,
            #[data_view(schema = Bool(682))]
            pub lowest_hop_count: ::data_views::Field<bool>,
            #[data_view(schema = Dict(1790))]
            pub constraints: ::data_views::Field<default_virtual_topology::Constraints<'a, Mode>>,
            #[data_view(schema = Dict(878))]
            pub outlier_elimination: ::data_views::Field<super::super::super::super::eos_config::router_adaptive_virtual_topology::profiles::item::OutlierElimination<'a, Mode>>,
            #[data_view(schema = Dict(876))]
            pub metric_order: ::data_views::Field<super::super::super::super::eos_config::router_adaptive_virtual_topology::profiles::item::MetricOrder<'a, Mode>>,
            #[data_view(schema = List(796))]
            pub path_groups: ::data_views::Field<default_virtual_topology::PathGroups<'a, Mode>>,
            #[data_view(schema = Dict(1792))]
            pub internet_exit: ::data_views::Field<default_virtual_topology::InternetExit<'a, Mode>>,
        }

        pub mod default_virtual_topology {

            #[::data_views::data_view(python(name = "AVDDesign.WanVirtualTopologies.PoliciesItemBase.DefaultVirtualTopology.Constraints", schema = Dict(1790)))]
            pub struct Constraints<'a, Mode> {
                #[data_view(schema = Int(444))]
                pub jitter: ::data_views::Field<i64>,
                #[data_view(schema = Int(445))]
                pub latency: ::data_views::Field<i64>,
                #[data_view(schema = Str(891))]
                pub loss_rate: ::data_views::Field<&'a str>,
            }

            #[::data_views::data_view(list, python(name = "AVDDesign.WanVirtualTopologies.PoliciesItemBase.DefaultVirtualTopology.PathGroups", schema = List(796)))]
            pub struct PathGroups<'a, Mode> (::data_views::Field<path_groups::Item<'a, Mode>>);

            pub mod path_groups {

                #[::data_views::data_view(python(name = "AVDDesign.WanVirtualTopologies.PoliciesItemBase.DefaultVirtualTopology.PathGroupsItem", schema = Dict(1791)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = List(795))]
                    pub names: ::data_views::RequiredValue<item::Names<'a, Mode>, Mode>,
                    #[data_view(schema = Str(1692))]
                    pub preference: ::data_views::Field<&'a str>,
                }

                pub mod item {

                    #[::data_views::data_view(list, python(name = "AVDDesign.WanVirtualTopologies.PoliciesItemBase.DefaultVirtualTopology.PathGroupsItem.Names", schema = List(795)))]
                    pub struct Names<'a, Mode> (::data_views::Field<&'a str>);
                }
            }

            #[::data_views::data_view(python(name = "AVDDesign.WanVirtualTopologies.PoliciesItemBase.DefaultVirtualTopology.InternetExit", schema = Dict(1792)))]
            pub struct InternetExit<'a, Mode> {
                #[data_view(schema = Str(1693))]
                pub policy: ::data_views::Field<&'a str>,
            }
        }
    }
}
