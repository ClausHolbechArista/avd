// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view]
pub struct Ptp<'a, Mode> {
    pub mode: ::validated_data::Field<&'a str>,
    pub profile: ::validated_data::Field<&'a str>,
    pub mode_one_step: ::validated_data::Field<bool>,
    pub forward_unicast: ::validated_data::Field<bool>,
    pub clock_identity: ::validated_data::Field<&'a str>,
    pub source: ::validated_data::Field<Source<'a, Mode>>,
    pub priority1: ::validated_data::Field<i64>,
    pub priority2: ::validated_data::Field<i64>,
    pub ttl: ::validated_data::Field<i64>,
    pub domain: ::validated_data::Field<i64>,
    pub hold_ptp_time: ::validated_data::Field<i64>,
    pub message_type: ::validated_data::Field<MessageType<'a, Mode>>,
    pub monitor: ::validated_data::Field<Monitor<'a, Mode>>,
    pub free_running: ::validated_data::Field<FreeRunning<'a, Mode>>,
    pub forward_v1: ::validated_data::Field<bool>,
}

#[::validated_data::data_view]
pub struct Source<'a, Mode> {
    pub ip: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view]
pub struct MessageType<'a, Mode> {
    pub general: ::validated_data::Field<message_type::General<'a, Mode>>,
    pub event: ::validated_data::Field<message_type::Event<'a, Mode>>,
}

pub mod message_type {

    #[::validated_data::data_view]
    pub struct General<'a, Mode> {
        pub dscp: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct Event<'a, Mode> {
        pub dscp: ::validated_data::Field<i64>,
    }
}

#[::validated_data::data_view]
pub struct Monitor<'a, Mode> {
    pub enabled: ::validated_data::Field<bool>,
    pub threshold: ::validated_data::Field<monitor::Threshold<'a, Mode>>,
    pub missing_message: ::validated_data::Field<monitor::MissingMessage<'a, Mode>>,
}

pub mod monitor {

    #[::validated_data::data_view]
    pub struct Threshold<'a, Mode> {
        pub offset_from_master: ::validated_data::Field<i64>,
        pub mean_path_delay: ::validated_data::Field<i64>,
        pub drop: ::validated_data::Field<threshold::Drop<'a, Mode>>,
    }

    pub mod threshold {

        #[::validated_data::data_view]
        pub struct Drop<'a, Mode> {
            pub offset_from_master: ::validated_data::Field<i64>,
            pub mean_path_delay: ::validated_data::Field<i64>,
        }
    }

    #[::validated_data::data_view]
    pub struct MissingMessage<'a, Mode> {
        pub intervals: ::validated_data::Field<missing_message::Intervals<'a, Mode>>,
        pub sequence_ids: ::validated_data::Field<missing_message::SequenceIds<'a, Mode>>,
    }

    pub mod missing_message {

        #[::validated_data::data_view]
        pub struct Intervals<'a, Mode> {
            pub announce: ::validated_data::Field<i64>,
            pub follow_up: ::validated_data::Field<i64>,
            pub sync: ::validated_data::Field<i64>,
        }

        #[::validated_data::data_view]
        pub struct SequenceIds<'a, Mode> {
            pub enabled: ::validated_data::Field<bool>,
            pub announce: ::validated_data::Field<i64>,
            pub delay_resp: ::validated_data::Field<i64>,
            pub follow_up: ::validated_data::Field<i64>,
            pub sync: ::validated_data::Field<i64>,
        }
    }
}

#[::validated_data::data_view(python(name = "EOSConfig.Ptp.FreeRunning", schema = Dict(818)))]
pub struct FreeRunning<'a, Mode> {
    #[data_view(schema = Bool(292))]
    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
    #[data_view(schema = Bool(293))]
    pub source_clock_hardware: ::validated_data::Field<bool>,
}
