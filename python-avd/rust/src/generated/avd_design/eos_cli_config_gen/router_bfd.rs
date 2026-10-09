// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Multihop<'a, Mode> {
    pub interval: ::validated_data::Field<i64>,
    pub min_rx: ::validated_data::Field<i64>,
    pub multiplier: ::validated_data::Field<i64>,
}

#[::validated_data::data_view]
pub struct Sbfd<'a, Mode> {
    pub local_interface: ::validated_data::Field<sbfd::LocalInterface<'a, Mode>>,
    pub initiator_interval: ::validated_data::Field<i64>,
    pub initiator_multiplier: ::validated_data::Field<i64>,
    pub initiator_measurement_round_trip: ::validated_data::Field<bool>,
    pub reflector: ::validated_data::Field<sbfd::Reflector<'a, Mode>>,
}

pub mod sbfd {

    #[::validated_data::data_view]
    pub struct LocalInterface<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub protocols: ::validated_data::Field<local_interface::Protocols<'a, Mode>>,
    }

    pub mod local_interface {

        #[::validated_data::data_view]
        pub struct Protocols<'a, Mode> {
            pub ipv4: ::validated_data::Field<bool>,
            pub ipv6: ::validated_data::Field<bool>,
        }
    }

    #[::validated_data::data_view]
    pub struct Reflector<'a, Mode> {
        pub min_rx: ::validated_data::Field<i64>,
        pub local_discriminator: ::validated_data::Field<&'a str>,
    }
}
