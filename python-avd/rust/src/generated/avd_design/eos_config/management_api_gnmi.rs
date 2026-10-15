// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Transport<'a, Mode> {
    pub grpc: ::validated_data::Field<transport::Grpc<'a, Mode>>,
    pub grpc_tunnels: ::validated_data::Field<transport::GrpcTunnels<'a, Mode>>,
}

pub mod transport {

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct Grpc<'a, Mode> (::validated_data::Field<grpc::Item<'a, Mode>>);

    pub mod grpc {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub ssl_profile: ::validated_data::Field<&'a str>,
            pub vrf: ::validated_data::Field<&'a str>,
            pub notification_timestamp: ::validated_data::Field<&'a str>,
            pub ip_access_group: ::validated_data::Field<&'a str>,
            pub port: ::validated_data::Field<i64>,
            pub authorization_requests: ::validated_data::Field<bool>,
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct GrpcTunnels<'a, Mode> (::validated_data::Field<grpc_tunnels::Item<'a, Mode>>);

    pub mod grpc_tunnels {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub shutdown: ::validated_data::Field<bool>,
            pub tunnel_ssl_profile: ::validated_data::Field<&'a str>,
            pub gnmi_ssl_profile: ::validated_data::Field<&'a str>,
            pub vrf: ::validated_data::Field<&'a str>,
            pub destination: ::validated_data::Field<item::Destination<'a, Mode>>,
            pub local_interface: ::validated_data::Field<item::LocalInterface<'a, Mode>>,
            pub target: ::validated_data::Field<item::Target<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view]
            pub struct Destination<'a, Mode> {
                pub address: ::validated_data::RequiredValue<&'a str, Mode>,
                pub port: ::validated_data::RequiredValue<i64, Mode>,
            }

            #[::validated_data::data_view]
            pub struct LocalInterface<'a, Mode> {
                pub name: ::validated_data::RequiredValue<&'a str, Mode>,
                pub port: ::validated_data::RequiredValue<i64, Mode>,
            }

            #[::validated_data::data_view]
            pub struct Target<'a, Mode> {
                pub use_serial_number: ::validated_data::Field<bool>,
                pub target_ids: ::validated_data::Field<target::TargetIds<'a, Mode>>,
            }

            pub mod target {

                #[::validated_data::data_view(list)]
                pub struct TargetIds<'a, Mode> (::validated_data::Field<&'a str>);
            }
        }
    }
}
