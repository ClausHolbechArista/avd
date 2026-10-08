// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Servers {
        scalar item (0) -> &'a str;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct ClientRequests {
        model flooding_suppression_vlans("flooding_suppression_vlans", 0) -> client_requests::FloodingSuppressionVlans<'a>;
    }
}

pub mod client_requests {

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct FloodingSuppressionVlans {
            scalar item (0) -> &'a str;
        }
    }
}
