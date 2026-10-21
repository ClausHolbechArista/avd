// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct RouterAdaptiveVirtualTopology<'a, Mode> {
    pub topology_role: ::data_views::Field<&'a str>,
    pub gateway_vxlan: ::data_views::Field<bool>,
    pub region: ::data_views::Field<Region<'a, Mode>>,
    pub zone: ::data_views::Field<Zone<'a, Mode>>,
    pub site: ::data_views::Field<Site<'a, Mode>>,
    pub profiles: ::data_views::Field<Profiles<'a, Mode>>,
    pub policies: ::data_views::Field<Policies<'a, Mode>>,
    pub vrfs: ::data_views::Field<Vrfs<'a, Mode>>,
}

#[::data_views::data_view]
pub struct Region<'a, Mode> {
    pub name: ::data_views::RequiredValue<&'a str, Mode>,
    pub id: ::data_views::RequiredValue<i64, Mode>,
}

#[::data_views::data_view]
pub struct Zone<'a, Mode> {
    pub name: ::data_views::RequiredValue<&'a str, Mode>,
    pub id: ::data_views::RequiredValue<i64, Mode>,
}

#[::data_views::data_view]
pub struct Site<'a, Mode> {
    pub name: ::data_views::RequiredValue<&'a str, Mode>,
    pub id: ::data_views::RequiredValue<i64, Mode>,
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Profiles<'a, Mode> (::data_views::Field<profiles::Item<'a, Mode>>);

pub mod profiles {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub load_balance_policy: ::data_views::Field<&'a str>,
        pub internet_exit_policy: ::data_views::Field<&'a str>,
        pub metric_order: ::data_views::Field<item::MetricOrder<'a, Mode>>,
        pub outlier_elimination: ::data_views::Field<item::OutlierElimination<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view(python(name = "EOSConfig.RouterAdaptiveVirtualTopology.ProfilesItemBase.MetricOrder", schema = Dict(876)))]
        pub struct MetricOrder<'a, Mode> {
            #[data_view(schema = Str(749))]
            pub preferred_metric: ::data_views::RequiredValue<&'a str, Mode>,
        }

        #[::data_views::data_view(python(name = "EOSConfig.RouterAdaptiveVirtualTopology.ProfilesItemBase.OutlierElimination", schema = Dict(878)))]
        pub struct OutlierElimination<'a, Mode> {
            #[data_view(schema = Bool(309))]
            pub disabled: ::data_views::Field<bool>,
            #[data_view(schema = Dict(877))]
            pub threshold: ::data_views::Field<outlier_elimination::Threshold<'a, Mode>>,
        }

        pub mod outlier_elimination {

            #[::data_views::data_view(python(name = "EOSConfig.RouterAdaptiveVirtualTopology.ProfilesItemBase.OutlierElimination.Threshold", schema = Dict(877)))]
            pub struct Threshold<'a, Mode> {
                #[data_view(schema = Int(356))]
                pub jitter: ::data_views::Field<i64>,
                #[data_view(schema = Int(357))]
                pub latency: ::data_views::Field<i64>,
                #[data_view(schema = Str(750))]
                pub load: ::data_views::Field<&'a str>,
                #[data_view(schema = Str(751))]
                pub loss_rate: ::data_views::Field<&'a str>,
            }
        }
    }
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Policies<'a, Mode> (::data_views::Field<policies::Item<'a, Mode>>);

pub mod policies {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub matches: ::data_views::Field<item::Matches<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view(list)]
        pub struct Matches<'a, Mode> (::data_views::Field<matches::Item<'a, Mode>>);

        pub mod matches {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub application_profile: ::data_views::Field<&'a str>,
                pub avt_profile: ::data_views::Field<&'a str>,
                pub dscp: ::data_views::Field<i64>,
                pub traffic_class: ::data_views::Field<i64>,
            }
        }
    }
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Vrfs<'a, Mode> (::data_views::Field<vrfs::Item<'a, Mode>>);

pub mod vrfs {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub policy: ::data_views::Field<&'a str>,
        pub profiles: ::data_views::Field<item::Profiles<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view(indexed_list, primary_key(name))]
        pub struct Profiles<'a, Mode> (::data_views::Field<profiles::Item<'a, Mode>>);

        pub mod profiles {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::data_views::Field<&'a str>,
                pub id: ::data_views::RequiredValue<i64, Mode>,
            }
        }
    }
}
