// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Exec {
        model console("console", 0) -> exec::Console<'a>;
        model default("default", 1) -> exec::Default<'a>;
    }
}

pub mod exec {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Console {
            scalar field_type("type", 0) -> &'a str;
            model methods("methods", 1) -> console::Methods<'a>;
        }
    }

    pub mod console {

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Methods {
                model item (0) -> methods::Item<'a>;
            }
        }

        pub mod methods {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar method("method", 0) -> &'a str;
                    scalar group("group", 1) -> &'a str;
                }
            }
        }
    }

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Default {
            scalar field_type("type", 0) -> &'a str;
            model methods("methods", 1) -> default::Methods<'a>;
        }
    }

    pub mod default {

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Methods {
                model item (0) -> methods::Item<'a>;
            }
        }

        pub mod methods {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar method("method", 0) -> &'a str;
                    scalar group("group", 1) -> &'a str;
                }
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct System {
        model default("default", 0) -> system::Default<'a>;
    }
}

pub mod system {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Default {
            scalar field_type("type", 0) -> &'a str;
            model methods("methods", 1) -> default::Methods<'a>;
        }
    }

    pub mod default {

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Methods {
                model item (0) -> methods::Item<'a>;
            }
        }

        pub mod methods {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar method("method", 0) -> &'a str;
                    scalar group("group", 1) -> &'a str;
                }
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Dot1x {
        model default("default", 0) -> dot1x::Default<'a>;
    }
}

pub mod dot1x {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Default {
            scalar field_type("type", 0) -> &'a str;
            model methods("methods", 1) -> default::Methods<'a>;
        }
    }

    pub mod default {

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Methods {
                model item (0) -> methods::Item<'a>;
            }
        }

        pub mod methods {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar multicast("multicast", 0) -> bool;
                    scalar method("method", 1) -> &'a str;
                    scalar group("group", 2) -> &'a str;
                }
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Commands {
        model console("console", 0) -> commands::Console<'a>;
        model default("default", 1) -> commands::Default<'a>;
    }
}

pub mod commands {

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Console {
            model item (0) -> console::Item<'a>;
        }
    }

    pub mod console {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar commands("commands", 0) -> &'a str;
                scalar field_type("type", 1) -> &'a str;
                model methods("methods", 2) -> item::Methods<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Methods {
                    model item (0) -> methods::Item<'a>;
                }
            }

            pub mod methods {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar method("method", 0) -> &'a str;
                        scalar group("group", 1) -> &'a str;
                    }
                }
            }
        }
    }

    ::validation::define_archive_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Default {
            model item (0) -> default::Item<'a>;
        }
    }

    pub mod default {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar commands("commands", 0) -> &'a str;
                scalar field_type("type", 1) -> &'a str;
                model methods("methods", 2) -> item::Methods<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Methods {
                    model item (0) -> methods::Item<'a>;
                }
            }

            pub mod methods {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Item {
                        scalar method("method", 0) -> &'a str;
                        scalar group("group", 1) -> &'a str;
                    }
                }
            }
        }
    }
}
