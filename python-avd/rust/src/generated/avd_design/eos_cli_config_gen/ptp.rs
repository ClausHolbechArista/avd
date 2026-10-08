// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Source {
        scalar ip("ip", 0) -> &'a str;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct MessageType {
        model general("general", 0) -> message_type::General<'a>;
        model event("event", 1) -> message_type::Event<'a>;
    }
}

pub mod message_type {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct General {
            scalar dscp("dscp", 0) -> i64;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Event {
            scalar dscp("dscp", 0) -> i64;
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Monitor {
        scalar enabled("enabled", 0) -> bool;
        model threshold("threshold", 1) -> monitor::Threshold<'a>;
        model missing_message("missing_message", 2) -> monitor::MissingMessage<'a>;
    }
}

pub mod monitor {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Threshold {
            scalar offset_from_master("offset_from_master", 0) -> i64;
            scalar mean_path_delay("mean_path_delay", 1) -> i64;
            model drop("drop", 2) -> threshold::Drop<'a>;
        }
    }

    pub mod threshold {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Drop {
                scalar offset_from_master("offset_from_master", 0) -> i64;
                scalar mean_path_delay("mean_path_delay", 1) -> i64;
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct MissingMessage {
            model intervals("intervals", 0) -> missing_message::Intervals<'a>;
            model sequence_ids("sequence_ids", 1) -> missing_message::SequenceIds<'a>;
        }
    }

    pub mod missing_message {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Intervals {
                scalar announce("announce", 0) -> i64;
                scalar follow_up("follow_up", 1) -> i64;
                scalar sync("sync", 2) -> i64;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct SequenceIds {
                scalar enabled("enabled", 0) -> bool;
                scalar announce("announce", 1) -> i64;
                scalar delay_resp("delay_resp", 2) -> i64;
                scalar follow_up("follow_up", 3) -> i64;
                scalar sync("sync", 4) -> i64;
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct FreeRunning {
        scalar enabled("enabled", 0) -> bool;
        scalar source_clock_hardware("source_clock_hardware", 1) -> bool;
    }
}
