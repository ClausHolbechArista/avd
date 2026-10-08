// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct AutoCertificate {
        model profiles("profiles", 0) -> auto_certificate::Profiles<'a>;
        model protocols("protocols", 1) -> auto_certificate::Protocols<'a>;
    }
}

pub mod auto_certificate {

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Profiles {
            model item (0) -> profiles::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod profiles {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                scalar digest("digest", 1) -> &'a str;
                scalar key("key", 2) -> &'a str;
                scalar protocol_instance_name("protocol_instance_name", 3) -> &'a str;
                scalar renewal("renewal", 4) -> i64;
                model parameters("parameters", 5) -> item::Parameters<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Parameters {
                    model distinguished_name("distinguished_name", 0) -> parameters::DistinguishedName<'a>;
                    model subject_alternative_name("subject_alternative_name", 1) -> parameters::SubjectAlternativeName<'a>;
                }
            }

            pub mod parameters {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct DistinguishedName {
                        scalar common_name("common_name", 0) -> &'a str;
                        scalar country("country", 1) -> &'a str;
                        scalar email("email", 2) -> &'a str;
                        scalar locality("locality", 3) -> &'a str;
                        scalar organization("organization", 4) -> &'a str;
                        scalar organization_unit("organization_unit", 5) -> &'a str;
                        scalar serial_number("serial_number", 6) -> &'a str;
                        scalar state("state", 7) -> &'a str;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct SubjectAlternativeName {
                        model dns("dns", 0) -> subject_alternative_name::Dns<'a>;
                        model email("email", 1) -> subject_alternative_name::Email<'a>;
                        model ip("ip", 2) -> subject_alternative_name::Ip<'a>;
                        model uri("uri", 3) -> subject_alternative_name::Uri<'a>;
                    }
                }

                pub mod subject_alternative_name {

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Dns {
                            scalar item (0) -> &'a str;
                        }
                    }

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Email {
                            scalar item (0) -> &'a str;
                        }
                    }

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Ip {
                            scalar item (0) -> &'a str;
                        }
                    }

                    ::validation::define_archive_list_view! {
                        #[derive(Clone, Copy, Debug)]
                        pub struct Uri {
                            scalar item (0) -> &'a str;
                        }
                    }
                }
            }
        }
    }

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Protocols {
            model item (0) -> protocols::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod protocols {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                scalar protocol("protocol", 1) -> &'a str;
                scalar disabled("disabled", 2) -> bool;
                model connection_retry("connection_retry", 3) -> item::ConnectionRetry<'a>;
                model credentials("credentials", 4) -> item::Credentials<'a>;
                model server("server", 5) -> item::Server<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct ConnectionRetry {
                    scalar count("count", 0) -> i64;
                    scalar interval("interval", 1) -> i64;
                    scalar exponential_backoff("exponential_backoff", 2) -> bool;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Credentials {
                    model enroll("enroll", 0) -> credentials::Enroll<'a>;
                    model re_enroll("re_enroll", 1) -> credentials::ReEnroll<'a>;
                }
            }

            pub mod credentials {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct Enroll {
                        scalar token("token", 0) -> &'a str;
                        scalar token_type("token_type", 1) -> &'a str;
                        scalar username("username", 2) -> &'a str;
                        scalar secret("secret", 3) -> &'a str;
                        scalar secret_type("secret_type", 4) -> &'a str;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct ReEnroll {
                        scalar token("token", 0) -> &'a str;
                        scalar token_type("token_type", 1) -> &'a str;
                        scalar username("username", 2) -> &'a str;
                        scalar secret("secret", 3) -> &'a str;
                        scalar secret_type("secret_type", 4) -> &'a str;
                    }
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Server {
                    scalar ssl_profile("ssl_profile", 0) -> &'a str;
                    scalar url("url", 1) -> &'a str;
                    scalar vrf("vrf", 2) -> &'a str;
                }
            }
        }
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct EntropySources {
        scalar hardware("hardware", 0) -> bool;
        scalar haveged("haveged", 1) -> bool;
        scalar cpu_jitter("cpu_jitter", 2) -> bool;
        scalar hardware_exclusive("hardware_exclusive", 3) -> bool;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct SignatureVerification {
        scalar enabled("enabled", 0) -> bool;
        scalar ssl_profile("ssl_profile", 1) -> &'a str;
    }
}

::validation::define_archive_dict_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct Password {
        scalar minimum_length("minimum_length", 0) -> i64;
        scalar encryption_key_common("encryption_key_common", 1) -> bool;
        scalar encryption_reversible("encryption_reversible", 2) -> &'a str;
        model policies("policies", 3) -> password::Policies<'a>;
    }
}

pub mod password {

    ::validation::define_archive_indexed_list_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Policies {
            model item (0) -> policies::Item<'a>;
            primary_key_fields: [0];
        }
    }

    pub mod policies {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Item {
                scalar name("name", 0) -> &'a str;
                model minimum("minimum", 1) -> item::Minimum<'a>;
                model maximum("maximum", 2) -> item::Maximum<'a>;
            }
        }

        pub mod item {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Minimum {
                    scalar digits("digits", 0) -> i64;
                    scalar length("length", 1) -> i64;
                    scalar lower("lower", 2) -> i64;
                    scalar special("special", 3) -> i64;
                    scalar upper("upper", 4) -> i64;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Maximum {
                    scalar repetitive("repetitive", 0) -> i64;
                    scalar sequential("sequential", 1) -> i64;
                }
            }
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct SslProfiles {
        model item (0) -> ssl_profiles::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod ssl_profiles {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar name("name", 0) -> &'a str;
            scalar fips_restrictions("fips_restrictions", 1) -> bool;
            scalar tls_versions("tls_versions", 2) -> &'a str;
            scalar cipher_list("cipher_list", 3) -> &'a str;
            model ciphers("ciphers", 4) -> item::Ciphers<'a>;
            model trust_certificate("trust_certificate", 5) -> item::TrustCertificate<'a>;
            model chain_certificate("chain_certificate", 6) -> item::ChainCertificate<'a>;
            model certificate("certificate", 7) -> item::Certificate<'a>;
            model certificate_revocation_lists("certificate_revocation_lists", 8) -> item::CertificateRevocationLists<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Ciphers {
                scalar v1_0("v1_0", 0) -> &'a str;
                scalar v1_3("v1_3", 1) -> &'a str;
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct TrustCertificate {
                model certificates("certificates", 0) -> trust_certificate::Certificates<'a>;
                model requirement("requirement", 1) -> trust_certificate::Requirement<'a>;
                scalar policy_expiry_date_ignore("policy_expiry_date_ignore", 2) -> bool;
                scalar system("system", 3) -> bool;
            }
        }

        pub mod trust_certificate {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Certificates {
                    scalar item (0) -> &'a str;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Requirement {
                    scalar basic_constraint_ca("basic_constraint_ca", 0) -> bool;
                    scalar hostname_fqdn("hostname_fqdn", 1) -> bool;
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct ChainCertificate {
                model certificates("certificates", 0) -> chain_certificate::Certificates<'a>;
                model requirement("requirement", 1) -> chain_certificate::Requirement<'a>;
            }
        }

        pub mod chain_certificate {

            ::validation::define_archive_list_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Certificates {
                    scalar item (0) -> &'a str;
                }
            }

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Requirement {
                    scalar basic_constraint_ca("basic_constraint_ca", 0) -> bool;
                    scalar include_root_ca("include_root_ca", 1) -> bool;
                }
            }
        }

        ::validation::define_archive_dict_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Certificate {
                scalar file("file", 0) -> &'a str;
                scalar key("key", 1) -> &'a str;
                scalar auto_certificate("auto_certificate", 2) -> &'a str;
            }
        }

        ::validation::define_archive_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct CertificateRevocationLists {
                scalar item (0) -> &'a str;
            }
        }
    }
}

::validation::define_archive_indexed_list_view! {
    #[derive(Clone, Copy, Debug)]
    pub struct SharedSecretProfiles {
        model item (0) -> shared_secret_profiles::Item<'a>;
        primary_key_fields: [0];
    }
}

pub mod shared_secret_profiles {

    ::validation::define_archive_dict_view! {
        #[derive(Clone, Copy, Debug)]
        pub struct Item {
            scalar profile("profile", 0) -> &'a str;
            model secrets("secrets", 1) -> item::Secrets<'a>;
        }
    }

    pub mod item {

        ::validation::define_archive_indexed_list_view! {
            #[derive(Clone, Copy, Debug)]
            pub struct Secrets {
                model item (0) -> secrets::Item<'a>;
                primary_key_fields: [0];
            }
        }

        pub mod secrets {

            ::validation::define_archive_dict_view! {
                #[derive(Clone, Copy, Debug)]
                pub struct Item {
                    scalar name("name", 0) -> &'a str;
                    scalar secret("secret", 1) -> &'a str;
                    scalar secret_type("secret_type", 2) -> &'a str;
                    model receive_lifetime("receive_lifetime", 3) -> item::ReceiveLifetime<'a>;
                    model transmit_lifetime("transmit_lifetime", 4) -> item::TransmitLifetime<'a>;
                    scalar local_time("local_time", 5) -> bool;
                }
            }

            pub mod item {

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct ReceiveLifetime {
                        scalar infinite("infinite", 0) -> bool;
                        scalar start_date_time("start_date_time", 1) -> &'a str;
                        scalar end_date_time("end_date_time", 2) -> &'a str;
                    }
                }

                ::validation::define_archive_dict_view! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct TransmitLifetime {
                        scalar infinite("infinite", 0) -> bool;
                        scalar start_date_time("start_date_time", 1) -> &'a str;
                        scalar end_date_time("end_date_time", 2) -> &'a str;
                    }
                }
            }
        }
    }
}
