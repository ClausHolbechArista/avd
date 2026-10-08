// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct HttpClient {
        scalar mgmt_interface("mgmt_interface", 0) -> bool;
        scalar inband_mgmt_interface("inband_mgmt_interface", 1) -> bool;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct SshClient {
        scalar mgmt_interface("mgmt_interface", 0) -> bool;
        scalar inband_mgmt_interface("inband_mgmt_interface", 1) -> bool;
    }
}
