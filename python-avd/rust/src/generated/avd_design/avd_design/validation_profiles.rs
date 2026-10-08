// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar name("name", 0) -> &'a str;
        scalar parent_profile("parent_profile", 1) -> &'a str;
        model hardware("hardware", 2) -> item::Hardware<'a>;
        model logging("logging", 3) -> item::Logging<'a>;
        scalar exclude_as_extra_fabric_validation_target("exclude_as_extra_fabric_validation_target", 4) -> bool;
        model interfaces("interfaces", 5) -> super::super::eos_cli_config_gen::metadata::Interfaces<'a>;
        model bgp("bgp", 6) -> super::super::eos_cli_config_gen::metadata::Bgp<'a>;
    }
}

pub mod item {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Hardware {
            scalar min_power_supplies("min_power_supplies", 0) -> i64;
            scalar min_fans("min_fans", 1) -> i64;
            scalar min_supervisors("min_supervisors", 2) -> i64;
            scalar min_line_cards("min_line_cards", 3) -> i64;
            scalar min_fabric_cards("min_fabric_cards", 4) -> i64;
            model transceiver_manufacturers("transceiver_manufacturers", 5) -> hardware::TransceiverManufacturers<'a>;
            scalar ignore_no_transceivers("ignore_no_transceivers", 6) -> bool;
        }
    }

    pub mod hardware {

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct TransceiverManufacturers {
                scalar item (0) -> &'a str;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Logging {
            scalar validate_no_errors_period("validate_no_errors_period", 0) -> i64;
        }
    }
}
