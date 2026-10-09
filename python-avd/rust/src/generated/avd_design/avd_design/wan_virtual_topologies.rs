// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Vrfs<'a, Mode> (::validated_data::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub policy: ::validated_data::Field<&'a str>,
        pub wan_vni: ::validated_data::RequiredValue<i64, Mode>,
    }
}

#[::validated_data::data_view]
pub struct ControlPlaneVirtualTopology<'a, Mode> {
    pub name: ::validated_data::Field<&'a str>,
    pub application_profile: ::validated_data::Field<&'a str>,
    pub traffic_class: ::validated_data::Field<i64>,
    pub dscp: ::validated_data::Field<i64>,
    pub lowest_hop_count: ::validated_data::Field<bool>,
    pub constraints: ::validated_data::Field<control_plane_virtual_topology::Constraints<'a, Mode>>,
    pub outlier_elimination: ::validated_data::Field<super::super::eos_cli_config_gen::router_adaptive_virtual_topology::profiles::item::OutlierElimination<'a, Mode>>,
    pub metric_order: ::validated_data::Field<super::super::eos_cli_config_gen::router_adaptive_virtual_topology::profiles::item::MetricOrder<'a, Mode>>,
    pub path_groups: ::validated_data::Field<control_plane_virtual_topology::PathGroups<'a, Mode>>,
    pub internet_exit: ::validated_data::Field<control_plane_virtual_topology::InternetExit<'a, Mode>>,
}

pub mod control_plane_virtual_topology {

    #[::validated_data::data_view]
    pub struct Constraints<'a, Mode> {
        pub jitter: ::validated_data::Field<i64>,
        pub latency: ::validated_data::Field<i64>,
        pub loss_rate: ::validated_data::Field<&'a str>,
    }

    #[::validated_data::data_view(list)]
    pub struct PathGroups<'a, Mode> (::validated_data::Field<path_groups::Item<'a, Mode>>);

    pub mod path_groups {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub names: ::validated_data::RequiredValue<item::Names<'a, Mode>, Mode>,
            pub preference: ::validated_data::Field<&'a str>,
        }

        pub mod item {

            #[::validated_data::data_view(list)]
            pub struct Names<'a, Mode> (::validated_data::Field<&'a str>);
        }
    }

    #[::validated_data::data_view]
    pub struct InternetExit<'a, Mode> {
        pub policy: ::validated_data::Field<&'a str>,
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Policies<'a, Mode> (::validated_data::Field<policies::Item<'a, Mode>>);

pub mod policies {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub application_virtual_topologies: ::validated_data::Field<item::ApplicationVirtualTopologies<'a, Mode>>,
        pub default_virtual_topology: ::validated_data::RequiredValue<item::DefaultVirtualTopology<'a, Mode>, Mode>,
    }

    pub mod item {

        #[::validated_data::data_view(indexed_list, primary_key(application_profile))]
        pub struct ApplicationVirtualTopologies<'a, Mode> (::validated_data::Field<application_virtual_topologies::Item<'a, Mode>>);

        pub mod application_virtual_topologies {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub application_profile: ::validated_data::Field<&'a str>,
                pub name: ::validated_data::Field<&'a str>,
                pub id: ::validated_data::Field<i64>,
                pub traffic_class: ::validated_data::Field<i64>,
                pub dscp: ::validated_data::Field<i64>,
                pub lowest_hop_count: ::validated_data::Field<bool>,
                pub constraints: ::validated_data::Field<item::Constraints<'a, Mode>>,
                pub outlier_elimination: ::validated_data::Field<super::super::super::super::super::eos_cli_config_gen::router_adaptive_virtual_topology::profiles::item::OutlierElimination<'a, Mode>>,
                pub metric_order: ::validated_data::Field<super::super::super::super::super::eos_cli_config_gen::router_adaptive_virtual_topology::profiles::item::MetricOrder<'a, Mode>>,
                pub path_groups: ::validated_data::Field<item::PathGroups<'a, Mode>>,
                pub internet_exit: ::validated_data::Field<item::InternetExit<'a, Mode>>,
            }

            pub mod item {

                #[::validated_data::data_view]
                pub struct Constraints<'a, Mode> {
                    pub jitter: ::validated_data::Field<i64>,
                    pub latency: ::validated_data::Field<i64>,
                    pub loss_rate: ::validated_data::Field<&'a str>,
                }

                #[::validated_data::data_view(list)]
                pub struct PathGroups<'a, Mode> (::validated_data::Field<path_groups::Item<'a, Mode>>);

                pub mod path_groups {

                    #[::validated_data::data_view]
                    pub struct Item<'a, Mode> {
                        pub names: ::validated_data::RequiredValue<item::Names<'a, Mode>, Mode>,
                        pub preference: ::validated_data::Field<&'a str>,
                    }

                    pub mod item {

                        #[::validated_data::data_view(list)]
                        pub struct Names<'a, Mode> (::validated_data::Field<&'a str>);
                    }
                }

                #[::validated_data::data_view]
                pub struct InternetExit<'a, Mode> {
                    pub policy: ::validated_data::Field<&'a str>,
                }
            }
        }

        #[::validated_data::data_view]
        pub struct DefaultVirtualTopology<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub drop_unmatched: ::validated_data::Field<bool>,
            pub traffic_class: ::validated_data::Field<i64>,
            pub dscp: ::validated_data::Field<i64>,
            pub lowest_hop_count: ::validated_data::Field<bool>,
            pub constraints: ::validated_data::Field<default_virtual_topology::Constraints<'a, Mode>>,
            pub outlier_elimination: ::validated_data::Field<super::super::super::super::eos_cli_config_gen::router_adaptive_virtual_topology::profiles::item::OutlierElimination<'a, Mode>>,
            pub metric_order: ::validated_data::Field<super::super::super::super::eos_cli_config_gen::router_adaptive_virtual_topology::profiles::item::MetricOrder<'a, Mode>>,
            pub path_groups: ::validated_data::Field<default_virtual_topology::PathGroups<'a, Mode>>,
            pub internet_exit: ::validated_data::Field<default_virtual_topology::InternetExit<'a, Mode>>,
        }

        pub mod default_virtual_topology {

            #[::validated_data::data_view]
            pub struct Constraints<'a, Mode> {
                pub jitter: ::validated_data::Field<i64>,
                pub latency: ::validated_data::Field<i64>,
                pub loss_rate: ::validated_data::Field<&'a str>,
            }

            #[::validated_data::data_view(list)]
            pub struct PathGroups<'a, Mode> (::validated_data::Field<path_groups::Item<'a, Mode>>);

            pub mod path_groups {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub names: ::validated_data::RequiredValue<item::Names<'a, Mode>, Mode>,
                    pub preference: ::validated_data::Field<&'a str>,
                }

                pub mod item {

                    #[::validated_data::data_view(list)]
                    pub struct Names<'a, Mode> (::validated_data::Field<&'a str>);
                }
            }

            #[::validated_data::data_view]
            pub struct InternetExit<'a, Mode> {
                pub policy: ::validated_data::Field<&'a str>,
            }
        }
    }
}
