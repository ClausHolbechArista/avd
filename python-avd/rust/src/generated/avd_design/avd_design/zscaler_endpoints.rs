// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Primary {
        scalar ip_address("ip_address", 0) -> &'a str;
        scalar datacenter("datacenter", 1) -> &'a str;
        scalar city("city", 2) -> &'a str;
        scalar country("country", 3) -> &'a str;
        scalar region("region", 4) -> &'a str;
        scalar latitude("latitude", 5) -> &'a str;
        scalar longitude("longitude", 6) -> &'a str;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Secondary {
        scalar ip_address("ip_address", 0) -> &'a str;
        scalar datacenter("datacenter", 1) -> &'a str;
        scalar city("city", 2) -> &'a str;
        scalar country("country", 3) -> &'a str;
        scalar region("region", 4) -> &'a str;
        scalar latitude("latitude", 5) -> &'a str;
        scalar longitude("longitude", 6) -> &'a str;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Tertiary {
        scalar ip_address("ip_address", 0) -> &'a str;
        scalar datacenter("datacenter", 1) -> &'a str;
        scalar city("city", 2) -> &'a str;
        scalar country("country", 3) -> &'a str;
        scalar region("region", 4) -> &'a str;
        scalar latitude("latitude", 5) -> &'a str;
        scalar longitude("longitude", 6) -> &'a str;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct DeviceLocation {
        scalar city("city", 0) -> &'a str;
        scalar country("country", 1) -> &'a str;
    }
}
