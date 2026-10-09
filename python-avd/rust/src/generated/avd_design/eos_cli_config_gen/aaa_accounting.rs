// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.


#[::validated_data::data_view]
pub struct Exec<'a, Mode> {
    pub console: ::validated_data::Field<exec::Console<'a, Mode>>,
    pub default: ::validated_data::Field<exec::Default<'a, Mode>>,
}

pub mod exec {

    #[::validated_data::data_view]
    pub struct Console<'a, Mode> {
        #[data_view(rename = "type")]
        pub field_type: ::validated_data::RequiredValue<&'a str, Mode>,
        pub methods: ::validated_data::Field<console::Methods<'a, Mode>>,
    }

    pub mod console {

        #[::validated_data::data_view(list)]
        pub struct Methods<'a, Mode> (::validated_data::Field<methods::Item<'a, Mode>>);

        pub mod methods {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub method: ::validated_data::RequiredValue<&'a str, Mode>,
                pub group: ::validated_data::Field<&'a str>,
            }
        }
    }

    #[::validated_data::data_view]
    pub struct Default<'a, Mode> {
        #[data_view(rename = "type")]
        pub field_type: ::validated_data::RequiredValue<&'a str, Mode>,
        pub methods: ::validated_data::Field<default::Methods<'a, Mode>>,
    }

    pub mod default {

        #[::validated_data::data_view(list)]
        pub struct Methods<'a, Mode> (::validated_data::Field<methods::Item<'a, Mode>>);

        pub mod methods {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub method: ::validated_data::RequiredValue<&'a str, Mode>,
                pub group: ::validated_data::Field<&'a str>,
            }
        }
    }
}

#[::validated_data::data_view]
pub struct System<'a, Mode> {
    pub default: ::validated_data::Field<system::Default<'a, Mode>>,
}

pub mod system {

    #[::validated_data::data_view]
    pub struct Default<'a, Mode> {
        #[data_view(rename = "type")]
        pub field_type: ::validated_data::RequiredValue<&'a str, Mode>,
        pub methods: ::validated_data::Field<default::Methods<'a, Mode>>,
    }

    pub mod default {

        #[::validated_data::data_view(list)]
        pub struct Methods<'a, Mode> (::validated_data::Field<methods::Item<'a, Mode>>);

        pub mod methods {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub method: ::validated_data::RequiredValue<&'a str, Mode>,
                pub group: ::validated_data::Field<&'a str>,
            }
        }
    }
}

#[::validated_data::data_view]
pub struct Dot1x<'a, Mode> {
    pub default: ::validated_data::Field<dot1x::Default<'a, Mode>>,
}

pub mod dot1x {

    #[::validated_data::data_view]
    pub struct Default<'a, Mode> {
        #[data_view(rename = "type")]
        pub field_type: ::validated_data::RequiredValue<&'a str, Mode>,
        pub methods: ::validated_data::Field<default::Methods<'a, Mode>>,
    }

    pub mod default {

        #[::validated_data::data_view(list)]
        pub struct Methods<'a, Mode> (::validated_data::Field<methods::Item<'a, Mode>>);

        pub mod methods {

            #[::validated_data::data_view]
            pub struct Item<'a, Mode> {
                pub multicast: ::validated_data::Field<bool>,
                pub method: ::validated_data::RequiredValue<&'a str, Mode>,
                pub group: ::validated_data::Field<&'a str>,
            }
        }
    }
}

#[::validated_data::data_view]
pub struct Commands<'a, Mode> {
    pub console: ::validated_data::Field<commands::Console<'a, Mode>>,
    pub default: ::validated_data::Field<commands::Default<'a, Mode>>,
}

pub mod commands {

    #[::validated_data::data_view(list)]
    pub struct Console<'a, Mode> (::validated_data::Field<console::Item<'a, Mode>>);

    pub mod console {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub commands: ::validated_data::Field<&'a str>,
            #[data_view(rename = "type")]
            pub field_type: ::validated_data::RequiredValue<&'a str, Mode>,
            pub methods: ::validated_data::Field<item::Methods<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view(list)]
            pub struct Methods<'a, Mode> (::validated_data::Field<methods::Item<'a, Mode>>);

            pub mod methods {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub method: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub group: ::validated_data::Field<&'a str>,
                }
            }
        }
    }

    #[::validated_data::data_view(list)]
    pub struct Default<'a, Mode> (::validated_data::Field<default::Item<'a, Mode>>);

    pub mod default {

        #[::validated_data::data_view]
        pub struct Item<'a, Mode> {
            pub commands: ::validated_data::Field<&'a str>,
            #[data_view(rename = "type")]
            pub field_type: ::validated_data::RequiredValue<&'a str, Mode>,
            pub methods: ::validated_data::Field<item::Methods<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view(list)]
            pub struct Methods<'a, Mode> (::validated_data::Field<methods::Item<'a, Mode>>);

            pub mod methods {

                #[::validated_data::data_view]
                pub struct Item<'a, Mode> {
                    pub method: ::validated_data::RequiredValue<&'a str, Mode>,
                    pub group: ::validated_data::Field<&'a str>,
                }
            }
        }
    }
}
