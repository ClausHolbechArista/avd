// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Vrfs {
        model item (0) -> vrfs::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod vrfs {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            model destinations("destinations", 1) -> item::Destinations<'a>;
            scalar source("source", 2) -> &'a str;
            scalar source_interface("source_interface", 3) -> &'a str;
        }
    }

    pub mod item {

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Destinations {
                model item (0) -> destinations::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod destinations {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar destination("destination", 0) -> &'a str;
                    scalar port("port", 1) -> i64;
                }
            }
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Destinations {
        model item (0) -> destinations::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod destinations {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar destination("destination", 0) -> &'a str;
            scalar port("port", 1) -> i64;
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Extensions {
        model item (0) -> extensions::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod extensions {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar enabled("enabled", 1) -> bool;
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Interface {
        model disable("disable", 0) -> interface::Disable<'a>;
        model egress("egress", 1) -> interface::Egress<'a>;
    }
}

pub mod interface {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Disable {
            scalar default("default", 0) -> bool;
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Egress {
            scalar enable_default("enable_default", 0) -> bool;
            scalar unmodified("unmodified", 1) -> bool;
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct HardwareAcceleration {
        scalar enabled("enabled", 0) -> bool;
        scalar sample("sample", 1) -> i64;
        model modules("modules", 2) -> hardware_acceleration::Modules<'a>;
    }
}

pub mod hardware_acceleration {

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Modules {
            model item (0) -> modules::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod modules {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                scalar enabled("enabled", 1) -> bool;
            }
        }
    }
}
