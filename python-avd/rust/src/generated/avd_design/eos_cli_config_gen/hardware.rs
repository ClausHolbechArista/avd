// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct AccessList {
        scalar mechanism("mechanism", 0) -> &'a str;
        scalar update_default_result_permit("update_default_result_permit", 1) -> bool;
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct SpeedGroups {
        model item (0) -> speed_groups::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod speed_groups {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar speed_group("speed_group", 0) -> &'a str;
            scalar serdes("serdes", 1) -> &'a str;
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct PortGroups {
        model item (0) -> port_groups::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod port_groups {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar port_group("port_group", 0) -> &'a str;
            scalar select("select", 1) -> &'a str;
        }
    }
}
