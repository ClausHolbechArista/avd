// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct IpNat<'a, Mode> {
    pub kernel_buffer_size: ::data_views::Field<i64>,
    pub profiles: ::data_views::Field<Profiles<'a, Mode>>,
    pub pools: ::data_views::Field<Pools<'a, Mode>>,
    pub synchronization: ::data_views::Field<Synchronization<'a, Mode>>,
    pub translation: ::data_views::Field<Translation<'a, Mode>>,
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Profiles<'a, Mode> (::data_views::Field<profiles::Item<'a, Mode>>);

pub mod profiles {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub vrf: ::data_views::Field<&'a str>,
        pub destination: ::data_views::Field<item::Destination<'a, Mode>>,
        pub source: ::data_views::Field<item::Source<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view]
        pub struct Destination<'a, Mode> {
            pub dynamic: ::data_views::Field<destination::Dynamic<'a, Mode>>,
            #[data_view(rename = "static")]
            pub field_static: ::data_views::Field<destination::FieldStatic<'a, Mode>>,
        }

        pub mod destination {

            #[::data_views::data_view(indexed_list, primary_key(access_list))]
            pub struct Dynamic<'a, Mode> (::data_views::Field<dynamic::Item<'a, Mode>>);

            pub mod dynamic {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub access_list: ::data_views::Field<&'a str>,
                    pub comment: ::data_views::Field<&'a str>,
                    pub pool_name: ::data_views::RequiredValue<&'a str, Mode>,
                    pub priority: ::data_views::Field<i64>,
                }
            }

            #[::data_views::data_view(list)]
            pub struct FieldStatic<'a, Mode> (::data_views::Field<field_static::Item<'a, Mode>>);

            pub mod field_static {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub access_list: ::data_views::Field<&'a str>,
                    pub comment: ::data_views::Field<&'a str>,
                    pub direction: ::data_views::Field<&'a str>,
                    pub group: ::data_views::Field<i64>,
                    pub original_ip: ::data_views::Field<&'a str>,
                    pub original_port: ::data_views::Field<i64>,
                    pub priority: ::data_views::Field<i64>,
                    pub protocol: ::data_views::Field<&'a str>,
                    pub translated_ip: ::data_views::RequiredValue<&'a str, Mode>,
                    pub translated_port: ::data_views::Field<i64>,
                }
            }
        }

        #[::data_views::data_view]
        pub struct Source<'a, Mode> {
            pub dynamic: ::data_views::Field<source::Dynamic<'a, Mode>>,
            #[data_view(rename = "static")]
            pub field_static: ::data_views::Field<source::FieldStatic<'a, Mode>>,
        }

        pub mod source {

            #[::data_views::data_view(indexed_list, primary_key(access_list))]
            pub struct Dynamic<'a, Mode> (::data_views::Field<dynamic::Item<'a, Mode>>);

            pub mod dynamic {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub access_list: ::data_views::Field<&'a str>,
                    pub comment: ::data_views::Field<&'a str>,
                    pub nat_type: ::data_views::RequiredValue<&'a str, Mode>,
                    pub pool_name: ::data_views::Field<&'a str>,
                    pub priority: ::data_views::Field<i64>,
                }
            }

            #[::data_views::data_view(list)]
            pub struct FieldStatic<'a, Mode> (::data_views::Field<field_static::Item<'a, Mode>>);

            pub mod field_static {

                #[::data_views::data_view]
                pub struct Item<'a, Mode> {
                    pub access_list: ::data_views::Field<&'a str>,
                    pub comment: ::data_views::Field<&'a str>,
                    pub direction: ::data_views::Field<&'a str>,
                    pub group: ::data_views::Field<i64>,
                    pub original_ip: ::data_views::Field<&'a str>,
                    pub original_port: ::data_views::Field<i64>,
                    pub priority: ::data_views::Field<i64>,
                    pub protocol: ::data_views::Field<&'a str>,
                    pub translated_ip: ::data_views::RequiredValue<&'a str, Mode>,
                    pub translated_port: ::data_views::Field<i64>,
                }
            }
        }
    }
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct Pools<'a, Mode> (::data_views::Field<pools::Item<'a, Mode>>);

pub mod pools {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        #[data_view(rename = "type")]
        pub field_type: ::data_views::Field<&'a str>,
        pub prefix_length: ::data_views::Field<i64>,
        pub ranges: ::data_views::Field<item::Ranges<'a, Mode>>,
        pub utilization_log_threshold: ::data_views::Field<i64>,
    }

    pub mod item {

        #[::data_views::data_view(list)]
        pub struct Ranges<'a, Mode> (::data_views::Field<ranges::Item<'a, Mode>>);

        pub mod ranges {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub first_ip: ::data_views::Field<&'a str>,
                pub last_ip: ::data_views::Field<&'a str>,
                pub first_port: ::data_views::Field<i64>,
                pub last_port: ::data_views::Field<i64>,
            }
        }
    }
}

#[::data_views::data_view]
pub struct Synchronization<'a, Mode> {
    pub description: ::data_views::Field<&'a str>,
    pub expiry_interval: ::data_views::Field<i64>,
    pub local_interface: ::data_views::Field<&'a str>,
    pub peer_address: ::data_views::Field<&'a str>,
    pub port_range: ::data_views::Field<synchronization::PortRange<'a, Mode>>,
    pub shutdown: ::data_views::Field<bool>,
}

pub mod synchronization {

    #[::data_views::data_view]
    pub struct PortRange<'a, Mode> {
        pub first_port: ::data_views::Field<i64>,
        pub last_port: ::data_views::Field<i64>,
        pub split_disabled: ::data_views::Field<bool>,
    }
}

#[::data_views::data_view]
pub struct Translation<'a, Mode> {
    pub address_selection: ::data_views::Field<translation::AddressSelection<'a, Mode>>,
    pub counters: ::data_views::Field<bool>,
    pub low_mark: ::data_views::Field<translation::LowMark<'a, Mode>>,
    pub max_entries: ::data_views::Field<translation::MaxEntries<'a, Mode>>,
    pub timeouts: ::data_views::Field<translation::Timeouts<'a, Mode>>,
}

pub mod translation {

    #[::data_views::data_view]
    pub struct AddressSelection<'a, Mode> {
        pub any: ::data_views::Field<bool>,
        pub hash_field_source_ip: ::data_views::Field<bool>,
    }

    #[::data_views::data_view]
    pub struct LowMark<'a, Mode> {
        pub percentage: ::data_views::Field<i64>,
        pub host_percentage: ::data_views::Field<i64>,
    }

    #[::data_views::data_view]
    pub struct MaxEntries<'a, Mode> {
        pub limit: ::data_views::Field<i64>,
        pub host_limit: ::data_views::Field<i64>,
        pub ip_limits: ::data_views::Field<max_entries::IpLimits<'a, Mode>>,
    }

    pub mod max_entries {

        #[::data_views::data_view(indexed_list, primary_key(ip))]
        pub struct IpLimits<'a, Mode> (::data_views::Field<ip_limits::Item<'a, Mode>>);

        pub mod ip_limits {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub ip: ::data_views::Field<&'a str>,
                pub limit: ::data_views::RequiredValue<i64, Mode>,
            }
        }
    }

    #[::data_views::data_view(indexed_list, primary_key(protocol))]
    pub struct Timeouts<'a, Mode> (::data_views::Field<timeouts::Item<'a, Mode>>);

    pub mod timeouts {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub protocol: ::data_views::Field<&'a str>,
            pub timeout: ::data_views::RequiredValue<i64, Mode>,
        }
    }
}
