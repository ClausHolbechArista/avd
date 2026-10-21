// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(python(name = "EOSConfig.HardwareCounters", schema = Dict(398)))]
pub struct HardwareCounters<'a, Mode> {
    #[data_view(schema = List(150))]
    pub features: ::data_views::Field<Features<'a, Mode>>,
}

#[::data_views::data_view(list, python(name = "EOSConfig.HardwareCounters.Features", schema = List(150)))]
pub struct Features<'a, Mode> (::data_views::Field<features::Item<'a, Mode>>);

pub mod features {

    #[::data_views::data_view(python(name = "EOSConfig.HardwareCounters.FeaturesItem", schema = Dict(397)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(346))]
        pub name: ::data_views::RequiredValue<&'a str, Mode>,
        #[data_view(schema = Str(347))]
        pub direction: ::data_views::Field<&'a str>,
        #[data_view(schema = Bool(37))]
        pub enabled: ::data_views::Field<bool>,
        #[data_view(schema = Str(348))]
        pub address_type: ::data_views::Field<&'a str>,
        #[data_view(schema = Bool(198))]
        pub layer3: ::data_views::Field<bool>,
        #[data_view(schema = Str(349))]
        pub vrf: ::data_views::Field<&'a str>,
        #[data_view(schema = Str(350))]
        pub prefix: ::data_views::Field<&'a str>,
        #[data_view(schema = Bool(7))]
        pub units_packets: ::data_views::Field<bool>,
    }
}
