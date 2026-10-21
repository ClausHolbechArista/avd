// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view(indexed_list, primary_key(id))]
pub struct Vlans<'a, Mode> (::data_views::Field<Item<'a, Mode>>);

#[::data_views::data_view]
pub struct Item<'a, Mode> {
    pub id: ::data_views::Field<i64>,
    pub name: ::data_views::Field<&'a str>,
    pub state: ::data_views::Field<&'a str>,
    pub address_locking: ::data_views::Field<item::AddressLocking<'a, Mode>>,
    pub trunk_groups: ::data_views::Field<item::TrunkGroups<'a, Mode>>,
    pub e_tree: ::data_views::Field<item::ETree<'a, Mode>>,
    pub private_vlan: ::data_views::Field<item::PrivateVlan<'a, Mode>>,
    pub metadata: ::data_views::Field<item::Metadata<'a, Mode>>,
}

pub mod item {

    #[::data_views::data_view]
    pub struct AddressLocking<'a, Mode> {
        pub address_family: ::data_views::Field<address_locking::AddressFamily<'a, Mode>>,
        pub ipv4_enforcement_disabled: ::data_views::Field<bool>,
    }

    pub mod address_locking {

        #[::data_views::data_view(python(name = "EOSConfig.VlansItemBase.AddressLocking.AddressFamily", schema = Dict(1340)))]
        pub struct AddressFamily<'a, Mode> {
            #[data_view(schema = Bool(391))]
            pub ipv4: ::data_views::Field<bool>,
            #[data_view(schema = Bool(392))]
            pub ipv6: ::data_views::Field<bool>,
        }
    }

    #[::data_views::data_view(list)]
    pub struct TrunkGroups<'a, Mode> (::data_views::Field<&'a str>);

    #[::data_views::data_view]
    pub struct ETree<'a, Mode> {
        pub leaf_role: ::data_views::Field<bool>,
        pub remote_leaf_host_drop: ::data_views::Field<bool>,
    }

    #[::data_views::data_view]
    pub struct PrivateVlan<'a, Mode> {
        #[data_view(rename = "type")]
        pub field_type: ::data_views::Field<&'a str>,
        pub primary_vlan: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct Metadata<'a, Mode> {
        pub tenants: ::data_views::Field<metadata::Tenants<'a, Mode>>,
    }

    pub mod metadata {

        #[::data_views::data_view(list)]
        pub struct Tenants<'a, Mode> (::data_views::Field<&'a str>);
    }
}
