// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct ArpProxy {
        scalar prefix_list("prefix_list", 0) -> &'a str;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct NdProxy {
        scalar prefix_list("prefix_list", 0) -> &'a str;
    }
}
