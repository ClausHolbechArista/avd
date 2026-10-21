// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view(python(name = "EOSConfig.HardwareCounters", schema = Dict(398)))]
pub struct HardwareCounters<'a, Mode> {
    #[data_view(schema = List(150))]
    pub features: ::validated_data::Field<Features<'a, Mode>>,
}

#[::validated_data::data_view(list, python(name = "EOSConfig.HardwareCounters.Features", schema = List(150)))]
pub struct Features<'a, Mode> (::validated_data::Field<features::Item<'a, Mode>>);

pub mod features {

    #[::validated_data::data_view(python(name = "EOSConfig.HardwareCounters.FeaturesItem", schema = Dict(397)))]
    pub struct Item<'a, Mode> {
        #[data_view(schema = Str(346))]
        pub name: ::validated_data::RequiredValue<&'a str, Mode>,
        #[data_view(schema = Str(347))]
        pub direction: ::validated_data::Field<&'a str>,
        #[data_view(schema = Bool(37))]
        pub enabled: ::validated_data::Field<bool>,
        #[data_view(schema = Str(348))]
        pub address_type: ::validated_data::Field<&'a str>,
        #[data_view(schema = Bool(198))]
        pub layer3: ::validated_data::Field<bool>,
        #[data_view(schema = Str(349))]
        pub vrf: ::validated_data::Field<&'a str>,
        #[data_view(schema = Str(350))]
        pub prefix: ::validated_data::Field<&'a str>,
        #[data_view(schema = Bool(7))]
        pub units_packets: ::validated_data::Field<bool>,
    }
}
