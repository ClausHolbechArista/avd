// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct ManagementApiGnmi<'a, Mode> {
    pub provider: ::data_views::Field<&'a str>,
    pub transport: ::data_views::Field<Transport<'a, Mode>>,
}

#[::data_views::data_view]
pub struct Transport<'a, Mode> {
    pub grpc: ::data_views::Field<transport::Grpc<'a, Mode>>,
    pub grpc_tunnels: ::data_views::Field<transport::GrpcTunnels<'a, Mode>>,
}

pub mod transport {

    #[::data_views::data_view(indexed_list, primary_key(name))]
    pub struct Grpc<'a, Mode> (::data_views::Field<grpc::Item<'a, Mode>>);

    pub mod grpc {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::data_views::Field<&'a str>,
            pub ssl_profile: ::data_views::Field<&'a str>,
            pub vrf: ::data_views::Field<&'a str>,
            pub notification_timestamp: ::data_views::Field<&'a str>,
            pub ip_access_group: ::data_views::Field<&'a str>,
            pub port: ::data_views::Field<i64>,
            pub authorization_requests: ::data_views::Field<bool>,
        }
    }

    #[::data_views::data_view(indexed_list, primary_key(name))]
    pub struct GrpcTunnels<'a, Mode> (::data_views::Field<grpc_tunnels::Item<'a, Mode>>);

    pub mod grpc_tunnels {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::data_views::Field<&'a str>,
            pub shutdown: ::data_views::Field<bool>,
            pub tunnel_ssl_profile: ::data_views::Field<&'a str>,
            pub gnmi_ssl_profile: ::data_views::Field<&'a str>,
            pub vrf: ::data_views::Field<&'a str>,
            pub destination: ::data_views::Field<item::Destination<'a, Mode>>,
            pub local_interface: ::data_views::Field<item::LocalInterface<'a, Mode>>,
            pub target: ::data_views::Field<item::Target<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view]
            pub struct Destination<'a, Mode> {
                pub address: ::data_views::RequiredValue<&'a str, Mode>,
                pub port: ::data_views::RequiredValue<i64, Mode>,
            }

            #[::data_views::data_view]
            pub struct LocalInterface<'a, Mode> {
                pub name: ::data_views::RequiredValue<&'a str, Mode>,
                pub port: ::data_views::RequiredValue<i64, Mode>,
            }

            #[::data_views::data_view]
            pub struct Target<'a, Mode> {
                pub use_serial_number: ::data_views::Field<bool>,
                pub target_ids: ::data_views::Field<target::TargetIds<'a, Mode>>,
            }

            pub mod target {

                #[::data_views::data_view(list)]
                pub struct TargetIds<'a, Mode> (::data_views::Field<&'a str>);
            }
        }
    }
}
