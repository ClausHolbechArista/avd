// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct ForwardingProfiles<'a, Mode> (::validated_data::Field<forwarding_profiles::Item<'a, Mode>>);

pub mod forwarding_profiles {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub protocols: ::validated_data::Field<item::Protocols<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view(indexed_list, primary_key(name))]
        pub struct Protocols<'a, Mode> (::validated_data::Field<protocols::Item<'a, Mode>>);

        pub mod protocols {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::validated_data::Field<&'a str>,
                pub forward: ::validated_data::Field<bool>,
                pub tagged_forward: ::validated_data::Field<bool>,
                pub untagged_forward: ::validated_data::Field<bool>,
            }
        }
    }
}
