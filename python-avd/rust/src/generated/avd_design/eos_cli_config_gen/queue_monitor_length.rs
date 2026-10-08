// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct DefaultThresholds {
        scalar high("high", 0) -> i64;
        scalar low("low", 1) -> i64;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Cpu {
        model thresholds("thresholds", 0) -> cpu::Thresholds<'a>;
    }
}

pub mod cpu {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Thresholds {
            scalar high("high", 0) -> i64;
            scalar low("low", 1) -> i64;
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Mirror {
        scalar enabled("enabled", 0) -> bool;
        model destination("destination", 1) -> mirror::Destination<'a>;
    }
}

pub mod mirror {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Destination {
            scalar cpu("cpu", 0) -> bool;
            model ethernet_interfaces("ethernet_interfaces", 1) -> destination::EthernetInterfaces<'a>;
            model tunnel_mode_gre("tunnel_mode_gre", 2) -> destination::TunnelModeGre<'a>;
        }
    }

    pub mod destination {

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct EthernetInterfaces {
                scalar item (0) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct TunnelModeGre {
                scalar source("source", 0) -> &'a str;
                scalar destination("destination", 1) -> &'a str;
                scalar dscp("dscp", 2) -> i64;
                scalar ttl("ttl", 3) -> i64;
                scalar protocol("protocol", 4) -> &'a str;
                scalar vrf("vrf", 5) -> &'a str;
            }
        }
    }
}
