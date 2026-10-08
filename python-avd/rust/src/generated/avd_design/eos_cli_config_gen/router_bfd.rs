// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Multihop {
        scalar interval("interval", 0) -> i64;
        scalar min_rx("min_rx", 1) -> i64;
        scalar multiplier("multiplier", 2) -> i64;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Sbfd {
        model local_interface("local_interface", 0) -> sbfd::LocalInterface<'a>;
        scalar initiator_interval("initiator_interval", 1) -> i64;
        scalar initiator_multiplier("initiator_multiplier", 2) -> i64;
        scalar initiator_measurement_round_trip("initiator_measurement_round_trip", 3) -> bool;
        model reflector("reflector", 4) -> sbfd::Reflector<'a>;
    }
}

pub mod sbfd {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct LocalInterface {
            scalar name("name", 0) -> &'a str;
            model protocols("protocols", 1) -> local_interface::Protocols<'a>;
        }
    }

    pub mod local_interface {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Protocols {
                scalar ipv4("ipv4", 0) -> bool;
                scalar ipv6("ipv6", 1) -> bool;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Reflector {
            scalar min_rx("min_rx", 0) -> i64;
            scalar local_discriminator("local_discriminator", 1) -> &'a str;
        }
    }
}
