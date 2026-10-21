// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct Ptp<'a, Mode> {
    pub mode: ::data_views::Field<&'a str>,
    pub profile: ::data_views::Field<&'a str>,
    pub mode_one_step: ::data_views::Field<bool>,
    pub forward_unicast: ::data_views::Field<bool>,
    pub clock_identity: ::data_views::Field<&'a str>,
    pub source: ::data_views::Field<Source<'a, Mode>>,
    pub priority1: ::data_views::Field<i64>,
    pub priority2: ::data_views::Field<i64>,
    pub ttl: ::data_views::Field<i64>,
    pub domain: ::data_views::Field<i64>,
    pub hold_ptp_time: ::data_views::Field<i64>,
    pub message_type: ::data_views::Field<MessageType<'a, Mode>>,
    pub monitor: ::data_views::Field<Monitor<'a, Mode>>,
    pub free_running: ::data_views::Field<FreeRunning<'a, Mode>>,
    pub forward_v1: ::data_views::Field<bool>,
}

#[::data_views::data_view]
pub struct Source<'a, Mode> {
    pub ip: ::data_views::Field<&'a str>,
}

#[::data_views::data_view]
pub struct MessageType<'a, Mode> {
    pub general: ::data_views::Field<message_type::General<'a, Mode>>,
    pub event: ::data_views::Field<message_type::Event<'a, Mode>>,
}

pub mod message_type {

    #[::data_views::data_view]
    pub struct General<'a, Mode> {
        pub dscp: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct Event<'a, Mode> {
        pub dscp: ::data_views::Field<i64>,
    }
}

#[::data_views::data_view]
pub struct Monitor<'a, Mode> {
    pub enabled: ::data_views::Field<bool>,
    pub threshold: ::data_views::Field<monitor::Threshold<'a, Mode>>,
    pub missing_message: ::data_views::Field<monitor::MissingMessage<'a, Mode>>,
}

pub mod monitor {

    #[::data_views::data_view]
    pub struct Threshold<'a, Mode> {
        pub offset_from_master: ::data_views::Field<i64>,
        pub mean_path_delay: ::data_views::Field<i64>,
        pub drop: ::data_views::Field<threshold::Drop<'a, Mode>>,
    }

    pub mod threshold {

        #[::data_views::data_view]
        pub struct Drop<'a, Mode> {
            pub offset_from_master: ::data_views::Field<i64>,
            pub mean_path_delay: ::data_views::Field<i64>,
        }
    }

    #[::data_views::data_view]
    pub struct MissingMessage<'a, Mode> {
        pub intervals: ::data_views::Field<missing_message::Intervals<'a, Mode>>,
        pub sequence_ids: ::data_views::Field<missing_message::SequenceIds<'a, Mode>>,
    }

    pub mod missing_message {

        #[::data_views::data_view]
        pub struct Intervals<'a, Mode> {
            pub announce: ::data_views::Field<i64>,
            pub follow_up: ::data_views::Field<i64>,
            pub sync: ::data_views::Field<i64>,
        }

        #[::data_views::data_view]
        pub struct SequenceIds<'a, Mode> {
            pub enabled: ::data_views::Field<bool>,
            pub announce: ::data_views::Field<i64>,
            pub delay_resp: ::data_views::Field<i64>,
            pub follow_up: ::data_views::Field<i64>,
            pub sync: ::data_views::Field<i64>,
        }
    }
}

#[::data_views::data_view(python(name = "EOSConfig.Ptp.FreeRunning", schema = Dict(818)))]
pub struct FreeRunning<'a, Mode> {
    #[data_view(schema = Bool(292))]
    pub enabled: ::data_views::RequiredValue<bool, Mode>,
    #[data_view(schema = Bool(293))]
    pub source_clock_hardware: ::data_views::Field<bool>,
}
