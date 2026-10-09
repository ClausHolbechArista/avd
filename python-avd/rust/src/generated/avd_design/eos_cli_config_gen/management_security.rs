// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct AutoCertificate<'a, Mode> {
    pub profiles: ::validated_data::Field<auto_certificate::Profiles<'a, Mode>>,
    pub protocols: ::validated_data::Field<auto_certificate::Protocols<'a, Mode>>,
}

pub mod auto_certificate {

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct Profiles<'a, Mode> (::validated_data::Field<profiles::Item<'a, Mode>>);

    pub mod profiles {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub digest: ::validated_data::Field<&'a str>,
            pub key: ::validated_data::Field<&'a str>,
            pub protocol_instance_name: ::validated_data::Field<&'a str>,
            pub renewal: ::validated_data::Field<i64>,
            pub parameters: ::validated_data::Field<item::Parameters<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view]
            pub struct Parameters<'a, Mode> {
                pub distinguished_name: ::validated_data::Field<parameters::DistinguishedName<'a, Mode>>,
                pub subject_alternative_name: ::validated_data::Field<parameters::SubjectAlternativeName<'a, Mode>>,
            }

            pub mod parameters {

                #[::validated_data::data_view]
                pub struct DistinguishedName<'a, Mode> {
                    pub common_name: ::validated_data::Field<&'a str>,
                    pub country: ::validated_data::Field<&'a str>,
                    pub email: ::validated_data::Field<&'a str>,
                    pub locality: ::validated_data::Field<&'a str>,
                    pub organization: ::validated_data::Field<&'a str>,
                    pub organization_unit: ::validated_data::Field<&'a str>,
                    pub serial_number: ::validated_data::Field<&'a str>,
                    pub state: ::validated_data::Field<&'a str>,
                }

                #[::validated_data::data_view]
                pub struct SubjectAlternativeName<'a, Mode> {
                    pub dns: ::validated_data::Field<subject_alternative_name::Dns<'a, Mode>>,
                    pub email: ::validated_data::Field<subject_alternative_name::Email<'a, Mode>>,
                    pub ip: ::validated_data::Field<subject_alternative_name::Ip<'a, Mode>>,
                    pub uri: ::validated_data::Field<subject_alternative_name::Uri<'a, Mode>>,
                }

                pub mod subject_alternative_name {

                    #[::validated_data::data_view(list)]
                    pub struct Dns<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(list)]
                    pub struct Email<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(list)]
                    pub struct Ip<'a, Mode> (::validated_data::Field<&'a str>);

                    #[::validated_data::data_view(list)]
                    pub struct Uri<'a, Mode> (::validated_data::Field<&'a str>);
                }
            }
        }
    }

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct Protocols<'a, Mode> (::validated_data::Field<protocols::Item<'a, Mode>>);

    pub mod protocols {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub protocol: ::validated_data::RequiredValue<&'a str, Mode>,
            pub disabled: ::validated_data::Field<bool>,
            pub connection_retry: ::validated_data::Field<item::ConnectionRetry<'a, Mode>>,
            pub credentials: ::validated_data::Field<item::Credentials<'a, Mode>>,
            pub server: ::validated_data::Field<item::Server<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view]
            pub struct ConnectionRetry<'a, Mode> {
                pub count: ::validated_data::Field<i64>,
                pub interval: ::validated_data::Field<i64>,
                pub exponential_backoff: ::validated_data::Field<bool>,
            }

            #[::validated_data::data_view]
            pub struct Credentials<'a, Mode> {
                pub enroll: ::validated_data::Field<credentials::Enroll<'a, Mode>>,
                pub re_enroll: ::validated_data::Field<credentials::ReEnroll<'a, Mode>>,
            }

            pub mod credentials {

                #[::validated_data::data_view]
                pub struct Enroll<'a, Mode> {
                    pub token: ::validated_data::Field<&'a str>,
                    pub token_type: ::validated_data::Field<&'a str>,
                    pub username: ::validated_data::Field<&'a str>,
                    pub secret: ::validated_data::Field<&'a str>,
                    pub secret_type: ::validated_data::Field<&'a str>,
                }

                #[::validated_data::data_view]
                pub struct ReEnroll<'a, Mode> {
                    pub token: ::validated_data::Field<&'a str>,
                    pub token_type: ::validated_data::Field<&'a str>,
                    pub username: ::validated_data::Field<&'a str>,
                    pub secret: ::validated_data::Field<&'a str>,
                    pub secret_type: ::validated_data::Field<&'a str>,
                }
            }

            #[::validated_data::data_view]
            pub struct Server<'a, Mode> {
                pub ssl_profile: ::validated_data::Field<&'a str>,
                pub url: ::validated_data::Field<&'a str>,
                pub vrf: ::validated_data::Field<&'a str>,
            }
        }
    }
}

#[::validated_data::data_view]
pub struct EntropySources<'a, Mode> {
    pub hardware: ::validated_data::Field<bool>,
    pub haveged: ::validated_data::Field<bool>,
    pub cpu_jitter: ::validated_data::Field<bool>,
    pub hardware_exclusive: ::validated_data::Field<bool>,
}

#[::validated_data::data_view]
pub struct SignatureVerification<'a, Mode> {
    pub enabled: ::validated_data::RequiredValue<bool, Mode>,
    pub ssl_profile: ::validated_data::Field<&'a str>,
}

#[::validated_data::data_view]
pub struct Password<'a, Mode> {
    pub minimum_length: ::validated_data::Field<i64>,
    pub encryption_key_common: ::validated_data::Field<bool>,
    pub encryption_reversible: ::validated_data::Field<&'a str>,
    pub policies: ::validated_data::Field<password::Policies<'a, Mode>>,
}

pub mod password {

    #[::validated_data::data_view(indexed_list, primary_key(name))]
    pub struct Policies<'a, Mode> (::validated_data::Field<policies::Item<'a, Mode>>);

    pub mod policies {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub name: ::validated_data::Field<&'a str>,
            pub minimum: ::validated_data::Field<item::Minimum<'a, Mode>>,
            pub maximum: ::validated_data::Field<item::Maximum<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view]
            pub struct Minimum<'a, Mode> {
                pub digits: ::validated_data::Field<i64>,
                pub length: ::validated_data::Field<i64>,
                pub lower: ::validated_data::Field<i64>,
                pub special: ::validated_data::Field<i64>,
                pub upper: ::validated_data::Field<i64>,
            }

            #[::validated_data::data_view]
            pub struct Maximum<'a, Mode> {
                pub repetitive: ::validated_data::Field<i64>,
                pub sequential: ::validated_data::Field<i64>,
            }
        }
    }
}

#[::validated_data::data_view(indexed_list, primary_key(name))]
pub struct SslProfiles<'a, Mode> (::validated_data::Field<ssl_profiles::Item<'a, Mode>>);

pub mod ssl_profiles {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub name: ::validated_data::Field<&'a str>,
        pub fips_restrictions: ::validated_data::Field<bool>,
        pub tls_versions: ::validated_data::Field<&'a str>,
        pub cipher_list: ::validated_data::Field<&'a str>,
        pub ciphers: ::validated_data::Field<item::Ciphers<'a, Mode>>,
        pub trust_certificate: ::validated_data::Field<item::TrustCertificate<'a, Mode>>,
        pub chain_certificate: ::validated_data::Field<item::ChainCertificate<'a, Mode>>,
        pub certificate: ::validated_data::Field<item::Certificate<'a, Mode>>,
        pub certificate_revocation_lists: ::validated_data::Field<item::CertificateRevocationLists<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view]
        pub struct Ciphers<'a, Mode> {
            pub v1_0: ::validated_data::Field<&'a str>,
            pub v1_3: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view]
        pub struct TrustCertificate<'a, Mode> {
            pub certificates: ::validated_data::Field<trust_certificate::Certificates<'a, Mode>>,
            pub requirement: ::validated_data::Field<trust_certificate::Requirement<'a, Mode>>,
            pub policy_expiry_date_ignore: ::validated_data::Field<bool>,
            pub system: ::validated_data::Field<bool>,
        }

        pub mod trust_certificate {

            #[::validated_data::data_view(list)]
            pub struct Certificates<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view]
            pub struct Requirement<'a, Mode> {
                pub basic_constraint_ca: ::validated_data::Field<bool>,
                pub hostname_fqdn: ::validated_data::Field<bool>,
            }
        }

        #[::validated_data::data_view]
        pub struct ChainCertificate<'a, Mode> {
            pub certificates: ::validated_data::Field<chain_certificate::Certificates<'a, Mode>>,
            pub requirement: ::validated_data::Field<chain_certificate::Requirement<'a, Mode>>,
        }

        pub mod chain_certificate {

            #[::validated_data::data_view(list)]
            pub struct Certificates<'a, Mode> (::validated_data::Field<&'a str>);

            #[::validated_data::data_view]
            pub struct Requirement<'a, Mode> {
                pub basic_constraint_ca: ::validated_data::Field<bool>,
                pub include_root_ca: ::validated_data::Field<bool>,
            }
        }

        #[::validated_data::data_view]
        pub struct Certificate<'a, Mode> {
            pub file: ::validated_data::Field<&'a str>,
            pub key: ::validated_data::Field<&'a str>,
            pub auto_certificate: ::validated_data::Field<&'a str>,
        }

        #[::validated_data::data_view(list)]
        pub struct CertificateRevocationLists<'a, Mode> (::validated_data::Field<&'a str>);
    }
}

#[::validated_data::data_view(indexed_list, primary_key(profile))]
pub struct SharedSecretProfiles<'a, Mode> (::validated_data::Field<shared_secret_profiles::Item<'a, Mode>>);

pub mod shared_secret_profiles {

    #[::validated_data::data_view]
    pub struct Item<'a, Mode> {
        pub profile: ::validated_data::Field<&'a str>,
        pub secrets: ::validated_data::Field<item::Secrets<'a, Mode>>,
    }

    pub mod item {

        #[::validated_data::data_view(indexed_list, primary_key(name))]
        pub struct Secrets<'a, Mode> (::validated_data::Field<secrets::Item<'a, Mode>>);

        pub mod secrets {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub name: ::validated_data::Field<&'a str>,
                pub secret: ::validated_data::RequiredValue<&'a str, Mode>,
                pub secret_type: ::validated_data::Field<&'a str>,
                pub receive_lifetime: ::validated_data::RequiredValue<item::ReceiveLifetime<'a, Mode>, Mode>,
                pub transmit_lifetime: ::validated_data::RequiredValue<item::TransmitLifetime<'a, Mode>, Mode>,
                pub local_time: ::validated_data::Field<bool>,
            }

            pub mod item {

                #[::validated_data::data_view]
                pub struct ReceiveLifetime<'a, Mode> {
                    pub infinite: ::validated_data::Field<bool>,
                    pub start_date_time: ::validated_data::Field<&'a str>,
                    pub end_date_time: ::validated_data::Field<&'a str>,
                }

                #[::validated_data::data_view]
                pub struct TransmitLifetime<'a, Mode> {
                    pub infinite: ::validated_data::Field<bool>,
                    pub start_date_time: ::validated_data::Field<&'a str>,
                    pub end_date_time: ::validated_data::Field<&'a str>,
                }
            }
        }
    }
}
