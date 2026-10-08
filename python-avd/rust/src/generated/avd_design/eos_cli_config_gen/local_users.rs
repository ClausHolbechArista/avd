// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        scalar name("name", 0) -> &'a str;
        scalar disabled("disabled", 1) -> bool;
        scalar privilege("privilege", 2) -> i64;
        scalar role("role", 3) -> &'a str;
        scalar sha512_password("sha512_password", 4) -> &'a str;
        scalar no_password("no_password", 5) -> bool;
        scalar ssh_key("ssh_key", 6) -> &'a str;
        scalar secondary_ssh_key("secondary_ssh_key", 7) -> &'a str;
        scalar shell("shell", 8) -> &'a str;
    }
}
