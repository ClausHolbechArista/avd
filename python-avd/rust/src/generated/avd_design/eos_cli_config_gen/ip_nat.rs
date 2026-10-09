// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Profiles<'a, Mode> (::validated_data::Field<profiles::Item<'a, Mode>>);

pub mod profiles {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub vrf: ::validated_data::Field<&'a str>,
        pub destination: ::validated_data::Field<item::Destination<'a, Mode>>,
        pub source: ::validated_data::Field<item::Source<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view]
        pub struct Destination<'a, Mode> {
            pub dynamic: ::validated_data::Field<destination::Dynamic<'a, Mode>>,
            #[data_view(rename = "static")]
            pub field_static: ::validated_data::Field<destination::FieldStatic<'a, Mode>>,
        }

        pub mod destination {

            #[::validated_data::data_view(indexed_list, primary_key(access_list))]
            pub struct Dynamic<'a, Mode> (::validated_data::Field<dynamic::Item<'a, Mode>>);

            pub mod dynamic {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub access_list: ::validated_data::Field<&'a str>,
                    pub comment: ::validated_data::Field<&'a str>,
                    pub pool_name: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub priority: ::validated_data::Field<i64>,
                }
            }

            #[::validated_data::data_view(list)]
            pub struct FieldStatic<'a, Mode> (::validated_data::Field<field_static::Item<'a, Mode>>);

            pub mod field_static {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub access_list: ::validated_data::Field<&'a str>,
                    pub comment: ::validated_data::Field<&'a str>,
                    pub direction: ::validated_data::Field<&'a str>,
                    pub group: ::validated_data::Field<i64>,
                    pub original_ip: ::validated_data::Field<&'a str>,
                    pub original_port: ::validated_data::Field<i64>,
                    pub priority: ::validated_data::Field<i64>,
                    pub protocol: ::validated_data::Field<&'a str>,
                    pub translated_ip: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub translated_port: ::validated_data::Field<i64>,
                }
            }
        }

        #[::validated_data::data_view]
        pub struct Source<'a, Mode> {
            pub dynamic: ::validated_data::Field<source::Dynamic<'a, Mode>>,
            #[data_view(rename = "static")]
            pub field_static: ::validated_data::Field<source::FieldStatic<'a, Mode>>,
        }

        pub mod source {

            #[::validated_data::data_view(indexed_list, primary_key(access_list))]
            pub struct Dynamic<'a, Mode> (::validated_data::Field<dynamic::Item<'a, Mode>>);

            pub mod dynamic {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub access_list: ::validated_data::Field<&'a str>,
                    pub comment: ::validated_data::Field<&'a str>,
                    pub nat_type: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub pool_name: ::validated_data::Field<&'a str>,
                    pub priority: ::validated_data::Field<i64>,
                }
            }

            #[::validated_data::data_view(list)]
            pub struct FieldStatic<'a, Mode> (::validated_data::Field<field_static::Item<'a, Mode>>);

            pub mod field_static {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub access_list: ::validated_data::Field<&'a str>,
                    pub comment: ::validated_data::Field<&'a str>,
                    pub direction: ::validated_data::Field<&'a str>,
                    pub group: ::validated_data::Field<i64>,
                    pub original_ip: ::validated_data::Field<&'a str>,
                    pub original_port: ::validated_data::Field<i64>,
                    pub priority: ::validated_data::Field<i64>,
                    pub protocol: ::validated_data::Field<&'a str>,
                    pub translated_ip: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub translated_port: ::validated_data::Field<i64>,
                }
            }
        }
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct Pools<'a, Mode> (::validated_data::Field<pools::Item<'a, Mode>>);

pub mod pools {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        #[data_view(rename = "type")]
        pub field_type: ::validated_data::Field<&'a str>,
        pub prefix_length: ::validated_data::Field<i64>,
        pub ranges: ::validated_data::Field<item::Ranges<'a, Mode>>,
        pub utilization_log_threshold: ::validated_data::Field<i64>,
    }

    pub mod item {

        #[::validated_data::data_view(list)]
        pub struct Ranges<'a, Mode> (::validated_data::Field<ranges::Item<'a, Mode>>);

        pub mod ranges {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub first_ip: ::validated_data::Field<&'a str>,
                pub last_ip: ::validated_data::Field<&'a str>,
                pub first_port: ::validated_data::Field<i64>,
                pub last_port: ::validated_data::Field<i64>,
            }
        }
    }
}

#[::validated_data::data_view]
pub struct Synchronization<'a, Mode> {
    pub description: ::validated_data::Field<&'a str>,
    pub expiry_interval: ::validated_data::Field<i64>,
    pub local_interface: ::validated_data::Field<&'a str>,
    pub peer_address: ::validated_data::Field<&'a str>,
    pub port_range: ::validated_data::Field<synchronization::PortRange<'a, Mode>>,
    pub shutdown: ::validated_data::Field<bool>,
}

pub mod synchronization {

    #[::validated_data::data_view]
    pub struct PortRange<'a, Mode> {
        pub first_port: ::validated_data::Field<i64>,
        pub last_port: ::validated_data::Field<i64>,
        pub split_disabled: ::validated_data::Field<bool>,
    }
}

#[::validated_data::data_view]
pub struct Translation<'a, Mode> {
    pub address_selection: ::validated_data::Field<translation::AddressSelection<'a, Mode>>,
    pub counters: ::validated_data::Field<bool>,
    pub low_mark: ::validated_data::Field<translation::LowMark<'a, Mode>>,
    pub max_entries: ::validated_data::Field<translation::MaxEntries<'a, Mode>>,
    pub timeouts: ::validated_data::Field<translation::Timeouts<'a, Mode>>,
}

pub mod translation {

    #[::validated_data::data_view]
    pub struct AddressSelection<'a, Mode> {
        pub any: ::validated_data::Field<bool>,
        pub hash_field_source_ip: ::validated_data::Field<bool>,
    }

    #[::validated_data::data_view]
    pub struct LowMark<'a, Mode> {
        pub percentage: ::validated_data::Field<i64>,
        pub host_percentage: ::validated_data::Field<i64>,
    }

    #[::validated_data::data_view]
    pub struct MaxEntries<'a, Mode> {
        pub limit: ::validated_data::Field<i64>,
        pub host_limit: ::validated_data::Field<i64>,
        pub ip_limits: ::validated_data::Field<max_entries::IpLimits<'a, Mode>>,
    }

    pub mod max_entries {

        #[::validated_data::data_view(indexed_list, primary_key(ip))]
        pub struct IpLimits<'a, Mode> (::validated_data::Field<ip_limits::Item<'a, Mode>>);

        pub mod ip_limits {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub ip: ::validated_data::Field<&'a str>,
                pub limit: ::validated_data::RequiredValue<i64, Mode>,
            }
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(protocol))]
    pub struct Timeouts<'a, Mode> (::validated_data::Field<timeouts::Item<'a, Mode>>);

    pub mod timeouts {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub protocol: ::validated_data::Field<&'a str>,
            pub timeout: ::validated_data::RequiredValue<i64, Mode>,
        }
    }
}
