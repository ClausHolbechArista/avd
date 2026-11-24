// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct RouterBfd<'a, Mode> {
    pub interval: ::data_views::Field<i64>,
    pub local_address: ::data_views::Field<&'a str>,
    pub min_rx: ::data_views::Field<i64>,
    pub multiplier: ::data_views::Field<i64>,
    pub multihop: ::data_views::Field<Multihop<'a, Mode>>,
    pub session_snapshot_interval: ::data_views::Field<i64>,
    pub session_snapshot_interval_dangerous: ::data_views::Field<bool>,
    pub sbfd: ::data_views::Field<Sbfd<'a, Mode>>,
    pub slow_timer: ::data_views::Field<i64>,
}

#[::data_views::data_view]
pub struct Multihop<'a, Mode> {
    pub interval: ::data_views::Field<i64>,
    pub min_rx: ::data_views::Field<i64>,
    pub multiplier: ::data_views::Field<i64>,
}

#[::data_views::data_view]
pub struct Sbfd<'a, Mode> {
    pub local_interface: ::data_views::Field<sbfd::LocalInterface<'a, Mode>>,
    pub initiator_interval: ::data_views::Field<i64>,
    pub initiator_multiplier: ::data_views::Field<i64>,
    pub initiator_measurement_round_trip: ::data_views::Field<bool>,
    pub reflector: ::data_views::Field<sbfd::Reflector<'a, Mode>>,
}

pub mod sbfd {

    #[::data_views::data_view]
    pub struct LocalInterface<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub protocols: ::data_views::Field<local_interface::Protocols<'a, Mode>>,
    }

    pub mod local_interface {

        #[::data_views::data_view]
        pub struct Protocols<'a, Mode> {
            pub ipv4: ::data_views::Field<bool>,
            pub ipv6: ::data_views::Field<bool>,
        }
    }

    #[::data_views::data_view]
    pub struct Reflector<'a, Mode> {
        pub min_rx: ::data_views::Field<i64>,
        pub local_discriminator: ::data_views::Field<&'a str>,
    }
}
