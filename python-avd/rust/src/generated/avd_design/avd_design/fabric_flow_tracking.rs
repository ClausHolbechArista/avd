// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Uplinks {
        scalar enabled("enabled", 0) -> bool;
        scalar name("name", 1) -> &'a str;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Downlinks {
        scalar enabled("enabled", 0) -> bool;
        scalar name("name", 1) -> &'a str;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Endpoints {
        scalar enabled("enabled", 0) -> bool;
        scalar name("name", 1) -> &'a str;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct L3Edge {
        scalar enabled("enabled", 0) -> bool;
        scalar name("name", 1) -> &'a str;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct CoreInterfaces {
        scalar enabled("enabled", 0) -> bool;
        scalar name("name", 1) -> &'a str;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct MlagInterfaces {
        scalar enabled("enabled", 0) -> bool;
        scalar name("name", 1) -> &'a str;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct L3Interfaces {
        scalar enabled("enabled", 0) -> bool;
        scalar name("name", 1) -> &'a str;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct L3PortChannels {
        scalar enabled("enabled", 0) -> bool;
        scalar name("name", 1) -> &'a str;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct DpsInterfaces {
        scalar enabled("enabled", 0) -> bool;
        scalar name("name", 1) -> &'a str;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct DirectWanHaLinks {
        scalar enabled("enabled", 0) -> bool;
        scalar name("name", 1) -> &'a str;
    }
}
