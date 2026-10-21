// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "AVDDesign.FlowTrackingSettings", schema = Dict(1581)))]
pub struct FlowTrackingSettings<'a, Mode> {
    #[data_view(schema = Dict(1573))]
    pub sampled: ::data_views::Field<Sampled<'a, Mode>>,
    #[data_view(schema = Dict(1574))]
    pub hardware: ::data_views::Field<Hardware<'a, Mode>>,
    #[data_view(schema = Dict(1575))]
    pub cloudvision_exporter: ::data_views::Field<CloudvisionExporter<'a, Mode>>,
    #[data_view(schema = List(653))]
    pub trackers: ::data_views::Field<Trackers<'a, Mode>>,
}

#[::data_views::data_view(python(name = "AVDDesign.FlowTrackingSettings.Sampled", schema = Dict(1573)))]
pub struct Sampled<'a, Mode> {
    #[data_view(schema = Dict(374))]
    pub encapsulation: ::data_views::Field<sampled::Encapsulation<'a, Mode>>,
    #[data_view(schema = Int(583))]
    pub sample: ::data_views::Field<i64>,
    #[data_view(schema = Dict(375))]
    pub hardware_offload: ::data_views::Field<sampled::HardwareOffload<'a, Mode>>,
}

pub mod sampled {

    #[::data_views::data_view(python(name = "AVDDesign.FlowTrackingSettings.Sampled.Encapsulation", schema = Dict(374)))]
    pub struct Encapsulation<'a, Mode> {
        #[data_view(schema = Bool(7))]
        pub ipv4_ipv6: ::data_views::Field<bool>,
        #[data_view(schema = Bool(7))]
        pub mpls: ::data_views::Field<bool>,
    }

    #[::data_views::data_view(python(name = "AVDDesign.FlowTrackingSettings.Sampled.HardwareOffload", schema = Dict(375)))]
    pub struct HardwareOffload<'a, Mode> {
        #[data_view(schema = Bool(190))]
        pub ipv4: ::data_views::Field<bool>,
        #[data_view(schema = Bool(191))]
        pub ipv6: ::data_views::Field<bool>,
        #[data_view(schema = Int(166))]
        pub threshold_minimum: ::data_views::Field<i64>,
    }
}

#[::data_views::data_view(python(name = "AVDDesign.FlowTrackingSettings.Hardware", schema = Dict(1574)))]
pub struct Hardware<'a, Mode> {
    #[data_view(schema = Dict(383))]
    pub record: ::data_views::Field<hardware::Record<'a, Mode>>,
}

pub mod hardware {

    #[::data_views::data_view(python(name = "AVDDesign.FlowTrackingSettings.Hardware.Record", schema = Dict(383)))]
    pub struct Record<'a, Mode> {
        #[data_view(schema = Bool(193))]
        pub format_ipfix_standard_timestamps_counters: ::data_views::Field<bool>,
    }
}

#[::data_views::data_view(python(name = "AVDDesign.FlowTrackingSettings.CloudvisionExporter", schema = Dict(1575)))]
pub struct CloudvisionExporter<'a, Mode> {
    #[data_view(schema = Str(1407))]
    pub name: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1408))]
    pub vrf: ::data_views::Field<&'a str>,
    #[data_view(schema = Str(1409))]
    pub source_interface: ::data_views::Field<&'a str>,
}

#[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.FlowTrackingSettings.Trackers", schema = List(653), item_name = "AVDDesign.FlowTrackingSettings.TrackersItem"))]
pub struct Trackers<'a, Mode> (::data_views::Field<trackers::Item<'a, Mode>>);

pub mod trackers {

    #[::data_views::data_view(python(name = "AVDDesign.FlowTrackingSettings.TrackersItemBase", schema = Dict(1580)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(1410))]
        pub name: ::data_views::Field<&'a str>,
        #[data_view(schema = Dict(1577))]
        pub sampled: ::data_views::Field<item::Sampled<'a, Mode>>,
        #[data_view(schema = Dict(1578))]
        pub record_export: ::data_views::Field<item::RecordExport<'a, Mode>>,
        #[data_view(schema = Bool(533))]
        pub export_to_cloudvision: ::data_views::Field<bool>,
        #[data_view(schema = List(652))]
        pub exporters: ::data_views::Field<item::Exporters<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view(python(name = "AVDDesign.FlowTrackingSettings.TrackersItemBase.Sampled", schema = Dict(1577)))]
        pub struct Sampled<'a, Mode> {
            #[data_view(schema = Int(167))]
            pub table_size: ::data_views::Field<i64>,
            #[data_view(schema = Dict(1576))]
            pub record_export: ::data_views::Field<sampled::RecordExport<'a, Mode>>,
        }

        pub mod sampled {

            #[::data_views::data_view(python(name = "AVDDesign.FlowTrackingSettings.TrackersItemBase.Sampled.RecordExport", schema = Dict(1576)))]
            pub struct RecordExport<'a, Mode> {
                #[data_view(schema = Bool(192))]
                pub mpls: ::data_views::Field<bool>,
            }
        }

        #[::data_views::data_view(python(name = "AVDDesign.FlowTrackingSettings.TrackersItemBase.RecordExport", schema = Dict(1578)))]
        pub struct RecordExport<'a, Mode> {
            #[data_view(schema = Int(584))]
            pub on_inactive_timeout: ::data_views::Field<i64>,
            #[data_view(schema = Int(585))]
            pub on_interval: ::data_views::Field<i64>,
        }

        #[::data_views::data_view(indexed_list, primary_key(name), python(name = "AVDDesign.FlowTrackingSettings.TrackersItemBase.Exporters", schema = List(652), item_name = "AVDDesign.FlowTrackingSettings.TrackersItemBase.ExportersItem"))]
        pub struct Exporters<'a, Mode> (::data_views::Field<exporters::Item<'a, Mode>>);

        pub mod exporters {

            #[::data_views::data_view(python(name = "AVDDesign.FlowTrackingSettings.TrackersItemBase.ExportersItemBase", schema = Dict(1579)))]
            pub struct Item<'a, Mode> {
                #[data_view(schema = Str(1411))]
                pub name: ::data_views::Field<&'a str>,
                #[data_view(schema = List(141))]
                pub collectors: ::data_views::Field<item::Collectors<'a, Mode>>,
                #[data_view(schema = Dict(378))]
                pub format: ::data_views::Field<item::Format<'a, Mode>>,
                #[data_view(schema = Str(1412))]
                pub local_interface: ::data_views::Field<&'a str>,
                #[data_view(schema = Int(586))]
                pub template_interval: ::data_views::Field<i64>,
            }

            pub mod item {

                #[::data_views::data_view(indexed_list, primary_key(host), python(name = "AVDDesign.FlowTrackingSettings.TrackersItemBase.ExportersItemBase.Collectors", schema = List(141), item_name = "AVDDesign.FlowTrackingSettings.TrackersItemBase.ExportersItemBase.CollectorsItem"))]
                pub struct Collectors<'a, Mode> (::data_views::Field<collectors::Item<'a, Mode>>);

                pub mod collectors {

                    #[::data_views::data_view(python(name = "AVDDesign.FlowTrackingSettings.TrackersItemBase.ExportersItemBase.CollectorsItemBase", schema = Dict(377)))]
                    pub struct Item<'a, Mode> {
                        #[data_view(schema = Str(339))]
                        pub host: ::data_views::Field<&'a str>,
                        #[data_view(schema = Int(170))]
                        pub port: ::data_views::Field<i64>,
                    }
                }

                #[::data_views::data_view(python(name = "AVDDesign.FlowTrackingSettings.TrackersItemBase.ExportersItemBase.Format", schema = Dict(378)))]
                pub struct Format<'a, Mode> {
                    #[data_view(schema = Int(77))]
                    pub ipfix_version: ::data_views::Field<i64>,
                }
            }
        }
    }
}
