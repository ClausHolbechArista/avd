// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Transport {
        model grpc("grpc", 0) -> transport::Grpc<'a>;
        model grpc_tunnels("grpc_tunnels", 1) -> transport::GrpcTunnels<'a>;
    }
}

pub mod transport {

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Grpc {
            model item (0) -> grpc::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod grpc {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                scalar ssl_profile("ssl_profile", 1) -> &'a str;
                scalar vrf("vrf", 2) -> &'a str;
                scalar notification_timestamp("notification_timestamp", 3) -> &'a str;
                scalar ip_access_group("ip_access_group", 4) -> &'a str;
                scalar port("port", 5) -> i64;
                scalar authorization_requests("authorization_requests", 6) -> bool;
            }
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct GrpcTunnels {
            model item (0) -> grpc_tunnels::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod grpc_tunnels {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                scalar shutdown("shutdown", 1) -> bool;
                scalar tunnel_ssl_profile("tunnel_ssl_profile", 2) -> &'a str;
                scalar gnmi_ssl_profile("gnmi_ssl_profile", 3) -> &'a str;
                scalar vrf("vrf", 4) -> &'a str;
                model destination("destination", 5) -> item::Destination<'a>;
                model local_interface("local_interface", 6) -> item::LocalInterface<'a>;
                model target("target", 7) -> item::Target<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Destination {
                    scalar address("address", 0) -> &'a str;
                    scalar port("port", 1) -> i64;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct LocalInterface {
                    scalar name("name", 0) -> &'a str;
                    scalar port("port", 1) -> i64;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Target {
                    scalar use_serial_number("use_serial_number", 0) -> bool;
                    model target_ids("target_ids", 1) -> target::TargetIds<'a>;
                }
            }

            pub mod target {

                ::validation::define_archive_list_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct TargetIds {
                        scalar item (0) -> &'a str;
                    }
                }
            }
        }
    }
}
