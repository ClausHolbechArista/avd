// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct ManagementSecurity<'a, Mode> {
    pub auto_certificate: ::data_views::Field<AutoCertificate<'a, Mode>>,
    pub entropy_sources: ::data_views::Field<EntropySources<'a, Mode>>,
    pub signature_verification: ::data_views::Field<SignatureVerification<'a, Mode>>,
    pub password: ::data_views::Field<Password<'a, Mode>>,
    pub ssl_profiles: ::data_views::Field<SslProfiles<'a, Mode>>,
    pub shared_secret_profiles: ::data_views::Field<SharedSecretProfiles<'a, Mode>>,
}

#[::data_views::data_view]
pub struct AutoCertificate<'a, Mode> {
    pub profiles: ::data_views::Field<auto_certificate::Profiles<'a, Mode>>,
    pub protocols: ::data_views::Field<auto_certificate::Protocols<'a, Mode>>,
}

pub mod auto_certificate {

    #[::data_views::data_view(indexed_list, primary_key(name))]
    pub struct Profiles<'a, Mode> (::data_views::Field<profiles::Item<'a, Mode>>);

    pub mod profiles {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::data_views::Field<&'a str>,
            pub digest: ::data_views::Field<&'a str>,
            pub key: ::data_views::Field<&'a str>,
            pub protocol_instance_name: ::data_views::Field<&'a str>,
            pub renewal: ::data_views::Field<i64>,
            pub parameters: ::data_views::Field<item::Parameters<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view]
            pub struct Parameters<'a, Mode> {
                pub distinguished_name: ::data_views::Field<parameters::DistinguishedName<'a, Mode>>,
                pub subject_alternative_name: ::data_views::Field<parameters::SubjectAlternativeName<'a, Mode>>,
            }

            pub mod parameters {

                #[::data_views::data_view]
                pub struct DistinguishedName<'a, Mode> {
                    pub common_name: ::data_views::Field<&'a str>,
                    pub country: ::data_views::Field<&'a str>,
                    pub email: ::data_views::Field<&'a str>,
                    pub locality: ::data_views::Field<&'a str>,
                    pub organization: ::data_views::Field<&'a str>,
                    pub organization_unit: ::data_views::Field<&'a str>,
                    pub serial_number: ::data_views::Field<&'a str>,
                    pub state: ::data_views::Field<&'a str>,
                }

                #[::data_views::data_view]
                pub struct SubjectAlternativeName<'a, Mode> {
                    pub dns: ::data_views::Field<subject_alternative_name::Dns<'a, Mode>>,
                    pub email: ::data_views::Field<subject_alternative_name::Email<'a, Mode>>,
                    pub ip: ::data_views::Field<subject_alternative_name::Ip<'a, Mode>>,
                    pub uri: ::data_views::Field<subject_alternative_name::Uri<'a, Mode>>,
                }

                pub mod subject_alternative_name {

                    #[::data_views::data_view(list)]
                    pub struct Dns<'a, Mode> (::data_views::Field<&'a str>);

                    #[::data_views::data_view(list)]
                    pub struct Email<'a, Mode> (::data_views::Field<&'a str>);

                    #[::data_views::data_view(list)]
                    pub struct Ip<'a, Mode> (::data_views::Field<&'a str>);

                    #[::data_views::data_view(list)]
                    pub struct Uri<'a, Mode> (::data_views::Field<&'a str>);
                }
            }
        }
    }

    #[::data_views::data_view(indexed_list, primary_key(name))]
    pub struct Protocols<'a, Mode> (::data_views::Field<protocols::Item<'a, Mode>>);

    pub mod protocols {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::data_views::Field<&'a str>,
            pub protocol: ::data_views::RequiredValue<&'a str, Mode>,
            pub disabled: ::data_views::Field<bool>,
            pub connection_retry: ::data_views::Field<item::ConnectionRetry<'a, Mode>>,
            pub credentials: ::data_views::Field<item::Credentials<'a, Mode>>,
            pub server: ::data_views::Field<item::Server<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view]
            pub struct ConnectionRetry<'a, Mode> {
                pub count: ::data_views::Field<i64>,
                pub interval: ::data_views::Field<i64>,
                pub exponential_backoff: ::data_views::Field<bool>,
            }

            #[::data_views::data_view]
            pub struct Credentials<'a, Mode> {
                pub enroll: ::data_views::Field<credentials::Enroll<'a, Mode>>,
                pub re_enroll: ::data_views::Field<credentials::ReEnroll<'a, Mode>>,
            }

            pub mod credentials {

                #[::data_views::data_view]
                pub struct Enroll<'a, Mode> {
                    pub token: ::data_views::Field<&'a str>,
                    pub token_type: ::data_views::Field<&'a str>,
                    pub username: ::data_views::Field<&'a str>,
                    pub secret: ::data_views::Field<&'a str>,
                    pub secret_type: ::data_views::Field<&'a str>,
                }

                #[::data_views::data_view]
                pub struct ReEnroll<'a, Mode> {
                    pub token: ::data_views::Field<&'a str>,
                    pub token_type: ::data_views::Field<&'a str>,
                    pub username: ::data_views::Field<&'a str>,
                    pub secret: ::data_views::Field<&'a str>,
                    pub secret_type: ::data_views::Field<&'a str>,
                }
            }

            #[::data_views::data_view]
            pub struct Server<'a, Mode> {
                pub ssl_profile: ::data_views::Field<&'a str>,
                pub url: ::data_views::Field<&'a str>,
                pub vrf: ::data_views::Field<&'a str>,
            }
        }
    }
}

#[::data_views::data_view]
pub struct EntropySources<'a, Mode> {
    pub hardware: ::data_views::Field<bool>,
    pub haveged: ::data_views::Field<bool>,
    pub cpu_jitter: ::data_views::Field<bool>,
    pub hardware_exclusive: ::data_views::Field<bool>,
}

#[::data_views::data_view]
pub struct SignatureVerification<'a, Mode> {
    pub enabled: ::data_views::RequiredValue<bool, Mode>,
    pub ssl_profile: ::data_views::Field<&'a str>,
}

#[::data_views::data_view]
pub struct Password<'a, Mode> {
    pub minimum_length: ::data_views::Field<i64>,
    pub encryption_key_common: ::data_views::Field<bool>,
    pub encryption_reversible: ::data_views::Field<&'a str>,
    pub policies: ::data_views::Field<password::Policies<'a, Mode>>,
}

pub mod password {

    #[::data_views::data_view(indexed_list, primary_key(name))]
    pub struct Policies<'a, Mode> (::data_views::Field<policies::Item<'a, Mode>>);

    pub mod policies {

        #[::data_views::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::data_views::Field<&'a str>,
            pub minimum: ::data_views::Field<item::Minimum<'a, Mode>>,
            pub maximum: ::data_views::Field<item::Maximum<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view]
            pub struct Minimum<'a, Mode> {
                pub digits: ::data_views::Field<i64>,
                pub length: ::data_views::Field<i64>,
                pub lower: ::data_views::Field<i64>,
                pub special: ::data_views::Field<i64>,
                pub upper: ::data_views::Field<i64>,
            }

            #[::data_views::data_view]
            pub struct Maximum<'a, Mode> {
                pub repetitive: ::data_views::Field<i64>,
                pub sequential: ::data_views::Field<i64>,
            }
        }
    }
}

#[::data_views::data_view(indexed_list, primary_key(name))]
pub struct SslProfiles<'a, Mode> (::data_views::Field<ssl_profiles::Item<'a, Mode>>);

pub mod ssl_profiles {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::data_views::Field<&'a str>,
        pub fips_restrictions: ::data_views::Field<bool>,
        pub tls_versions: ::data_views::Field<&'a str>,
        pub cipher_list: ::data_views::Field<&'a str>,
        pub ciphers: ::data_views::Field<item::Ciphers<'a, Mode>>,
        pub trust_certificate: ::data_views::Field<item::TrustCertificate<'a, Mode>>,
        pub chain_certificate: ::data_views::Field<item::ChainCertificate<'a, Mode>>,
        pub certificate: ::data_views::Field<item::Certificate<'a, Mode>>,
        pub certificate_revocation_lists: ::data_views::Field<item::CertificateRevocationLists<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view]
        pub struct Ciphers<'a, Mode> {
            pub v1_0: ::data_views::Field<&'a str>,
            pub v1_3: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view]
        pub struct TrustCertificate<'a, Mode> {
            pub certificates: ::data_views::Field<trust_certificate::Certificates<'a, Mode>>,
            pub requirement: ::data_views::Field<trust_certificate::Requirement<'a, Mode>>,
            pub policy_expiry_date_ignore: ::data_views::Field<bool>,
            pub system: ::data_views::Field<bool>,
        }

        pub mod trust_certificate {

            #[::data_views::data_view(list)]
            pub struct Certificates<'a, Mode> (::data_views::Field<&'a str>);

            #[::data_views::data_view]
            pub struct Requirement<'a, Mode> {
                pub basic_constraint_ca: ::data_views::Field<bool>,
                pub hostname_fqdn: ::data_views::Field<bool>,
            }
        }

        #[::data_views::data_view]
        pub struct ChainCertificate<'a, Mode> {
            pub certificates: ::data_views::Field<chain_certificate::Certificates<'a, Mode>>,
            pub requirement: ::data_views::Field<chain_certificate::Requirement<'a, Mode>>,
        }

        pub mod chain_certificate {

            #[::data_views::data_view(list)]
            pub struct Certificates<'a, Mode> (::data_views::Field<&'a str>);

            #[::data_views::data_view]
            pub struct Requirement<'a, Mode> {
                pub basic_constraint_ca: ::data_views::Field<bool>,
                pub include_root_ca: ::data_views::Field<bool>,
            }
        }

        #[::data_views::data_view]
        pub struct Certificate<'a, Mode> {
            pub file: ::data_views::Field<&'a str>,
            pub key: ::data_views::Field<&'a str>,
            pub auto_certificate: ::data_views::Field<&'a str>,
        }

        #[::data_views::data_view(list)]
        pub struct CertificateRevocationLists<'a, Mode> (::data_views::Field<&'a str>);
    }
}

#[::data_views::data_view(indexed_list, primary_key(profile))]
pub struct SharedSecretProfiles<'a, Mode> (::data_views::Field<shared_secret_profiles::Item<'a, Mode>>);

pub mod shared_secret_profiles {

    #[::data_views::data_view]
    pub struct Item<'a, Mode> {
        pub profile: ::data_views::Field<&'a str>,
        pub secrets: ::data_views::Field<item::Secrets<'a, Mode>>,
    }

    pub mod item {

        #[::data_views::data_view(indexed_list, primary_key(name))]
        pub struct Secrets<'a, Mode> (::data_views::Field<secrets::Item<'a, Mode>>);

        pub mod secrets {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::data_views::Field<&'a str>,
                pub secret: ::data_views::RequiredValue<&'a str, Mode>,
                pub secret_type: ::data_views::Field<&'a str>,
                pub receive_lifetime: ::data_views::RequiredValue<item::ReceiveLifetime<'a, Mode>, Mode>,
                pub transmit_lifetime: ::data_views::RequiredValue<item::TransmitLifetime<'a, Mode>, Mode>,
                pub local_time: ::data_views::Field<bool>,
            }

            pub mod item {

                #[::data_views::data_view]
                pub struct ReceiveLifetime<'a, Mode> {
                    pub infinite: ::data_views::Field<bool>,
                    pub start_date_time: ::data_views::Field<&'a str>,
                    pub end_date_time: ::data_views::Field<&'a str>,
                }

                #[::data_views::data_view]
                pub struct TransmitLifetime<'a, Mode> {
                    pub infinite: ::data_views::Field<bool>,
                    pub start_date_time: ::data_views::Field<&'a str>,
                    pub end_date_time: ::data_views::Field<&'a str>,
                }
            }
        }
    }
}
