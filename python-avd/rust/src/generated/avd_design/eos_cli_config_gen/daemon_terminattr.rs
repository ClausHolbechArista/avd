// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Cvaddrs {
        scalar item (0) -> &'a str;
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Clusters {
        model item (0) -> clusters::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod clusters {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            model cvaddrs("cvaddrs", 1) -> item::Cvaddrs<'a>;
            model cvauth("cvauth", 2) -> item::Cvauth<'a>;
            scalar cvobscurekeyfile("cvobscurekeyfile", 3) -> bool;
            scalar cvproxy("cvproxy", 4) -> &'a str;
            scalar cvsourceip("cvsourceip", 5) -> &'a str;
            scalar cvsourceintf("cvsourceintf", 6) -> &'a str;
            scalar cvvrf("cvvrf", 7) -> &'a str;
        }
    }

    pub mod item {

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Cvaddrs {
                scalar item (0) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Cvauth {
                scalar method("method", 0) -> &'a str;
                scalar key("key", 1) -> &'a str;
                scalar token_file("token_file", 2) -> &'a str;
                scalar cert_file("cert_file", 3) -> &'a str;
                scalar ca_file("ca_file", 4) -> &'a str;
                scalar key_file("key_file", 5) -> &'a str;
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Cvauth {
        scalar method("method", 0) -> &'a str;
        scalar key("key", 1) -> &'a str;
        scalar token_file("token_file", 2) -> &'a str;
        scalar cert_file("cert_file", 3) -> &'a str;
        scalar ca_file("ca_file", 4) -> &'a str;
        scalar key_file("key_file", 5) -> &'a str;
    }
}

::validation::define_archive_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Cvtargetconfigs {
        scalar item (0) -> &'a str;
    }
}

::validation::define_archive_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct CustomCvOptions {
        model item (0) -> custom_cv_options::Item<'a>;
    }
}

pub mod custom_cv_options {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar flag("flag", 0) -> &'a str;
            scalar value("value", 1) -> &'a str;
        }
    }
}
