// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct L2Protocol<'a, Mode> {
    pub forwarding_profiles: ::data_views::Field<ForwardingProfiles<'a, Mode>>,
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct ForwardingProfiles<'a, Mode> (::data_views::Field<forwarding_profiles::Item<'a, Mode>>);

pub mod forwarding_profiles {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub protocols: ::data_views::Field<item::Protocols<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view(indexed_list, primary_key(name))]
        pub struct Protocols<'a, Mode> (::data_views::Field<protocols::Item<'a, Mode>>);

        pub mod protocols {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::data_views::Field<&'a str>,
                pub forward: ::data_views::Field<bool>,
                pub tagged_forward: ::data_views::Field<bool>,
                pub untagged_forward: ::data_views::Field<bool>,
            }
        }
    }
}
