// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Item<'a, Mode> {
    pub id: ::validated_data::Field<i64>,
    pub name: ::validated_data::Field<&'a str>,
    pub state: ::validated_data::Field<&'a str>,
    pub address_locking: ::validated_data::Field<item::AddressLocking<'a, Mode>>,
    pub trunk_groups: ::validated_data::Field<item::TrunkGroups<'a, Mode>>,
    pub e_tree: ::validated_data::Field<item::ETree<'a, Mode>>,
    pub private_vlan: ::validated_data::Field<item::PrivateVlan<'a, Mode>>,
    pub metadata: ::validated_data::Field<item::Metadata<'a, Mode>>,
}

pub mod item {

    #[::validated_data::data_view]
    pub struct AddressLocking<'a, Mode> {
        pub address_family: ::validated_data::Field<address_locking::AddressFamily<'a, Mode>>,
        pub ipv4_enforcement_disabled: ::validated_data::Field<bool>,
    }

    pub mod address_locking {

        #[::validated_data::data_view]
        pub struct AddressFamily<'a, Mode> {
            pub ipv4: ::validated_data::Field<bool>,
            pub ipv6: ::validated_data::Field<bool>,
        }
    }

    #[::validated_data::data_view(list)]
    pub struct TrunkGroups<'a, Mode> (::validated_data::Field<&'a str>);

    #[::validated_data::data_view]
    pub struct ETree<'a, Mode> {
        pub leaf_role: ::validated_data::Field<bool>,
        pub remote_leaf_host_drop: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view]
    pub struct PrivateVlan<'a, Mode> {
        #[data_view(rename = "type")]
        pub field_type: ::validated_data::Field<&'a str>,
        pub primary_vlan: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct Metadata<'a, Mode> {
        pub tenants: ::validated_data::Field<metadata::Tenants<'a, Mode>>,
    }

    pub mod metadata {

        #[::validated_data::data_view(list)]
        pub struct Tenants<'a, Mode> (::validated_data::Field<&'a str>);
    }
}
