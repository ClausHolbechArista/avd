// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct MessageDigestKeys {
        model item (0) -> message_digest_keys::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod message_digest_keys {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar id("id", 0) -> i64;
            scalar hash_algorithm("hash_algorithm", 1) -> &'a str;
            scalar cleartext_key("cleartext_key", 2) -> &'a str;
        }
    }
}
