// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Login {
        scalar default("default", 0) -> &'a str;
        scalar command_api("command_api", 1) -> &'a str;
        scalar console("console", 2) -> &'a str;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Enable {
        scalar default("default", 0) -> &'a str;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Dot1x {
        scalar default("default", 0) -> &'a str;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Policies {
        scalar on_failure_log("on_failure_log", 0) -> bool;
        scalar on_success_log("on_success_log", 1) -> bool;
        model local("local", 2) -> policies::Local<'a>;
        model lockout("lockout", 3) -> policies::Lockout<'a>;
    }
}

pub mod policies {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Local {
            scalar allow_nopassword("allow_nopassword", 0) -> bool;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Lockout {
            scalar failure("failure", 0) -> i64;
            scalar duration("duration", 1) -> i64;
            scalar window("window", 2) -> i64;
        }
    }
}
