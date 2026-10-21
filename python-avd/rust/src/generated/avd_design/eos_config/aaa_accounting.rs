// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::validated_data::data_view]
pub struct AaaAccounting<'a, Mode> {
    pub exec: ::validated_data::Field<Exec<'a, Mode>>,
    pub system: ::validated_data::Field<System<'a, Mode>>,
    pub dot1x: ::validated_data::Field<Dot1x<'a, Mode>>,
    pub commands: ::validated_data::Field<Commands<'a, Mode>>,
}

#[::validated_data::data_view(python(name = "EOSConfig.AaaAccounting.Exec", schema = Dict(32)))]
pub struct Exec<'a, Mode> {
    #[data_view(schema = Dict(31))]
    pub console: ::validated_data::Field<exec::Console<'a, Mode>>,
    #[data_view(schema = Dict(31))]
    pub default: ::validated_data::Field<exec::Default<'a, Mode>>,
}

pub mod exec {

    #[::validated_data::data_view(python(name = "EOSConfig.AaaAccounting.Exec.Console", schema = Dict(31)))]
    pub struct Console<'a, Mode> {
        #[data_view(schema = Str(7), rename = "type")]
        pub field_type: ::validated_data::RequiredValue<&'a str, Mode>,
        #[data_view(schema = List(23))]
        pub methods: ::validated_data::Field<console::Methods<'a, Mode>>,
    }

    pub mod console {

        #[::validated_data::data_view(list, python(name = "EOSConfig.AaaAccounting.Exec.Console.Methods", schema = List(23)))]
        pub struct Methods<'a, Mode> (::validated_data::Field<methods::Item<'a, Mode>>);

        pub mod methods {

            #[::validated_data::data_view(python(name = "EOSConfig.AaaAccounting.Exec.Console.MethodsItem", schema = Dict(30)))]
            pub struct Item<'a, Mode> {
                #[data_view(schema = Str(8))]
                pub method: ::validated_data::RequiredValue<&'a str, Mode>,
                #[data_view(schema = Str(9))]
                pub group: ::validated_data::Field<&'a str>,
            }
        }
    }

    #[::validated_data::data_view(python(name = "EOSConfig.AaaAccounting.Exec.Default", schema = Dict(31)))]
    pub struct Default<'a, Mode> {
        #[data_view(schema = Str(7), rename = "type")]
        pub field_type: ::validated_data::RequiredValue<&'a str, Mode>,
        #[data_view(schema = List(23))]
        pub methods: ::validated_data::Field<default::Methods<'a, Mode>>,
    }

    pub mod default {

        #[::validated_data::data_view(list, python(name = "EOSConfig.AaaAccounting.Exec.Default.Methods", schema = List(23)))]
        pub struct Methods<'a, Mode> (::validated_data::Field<methods::Item<'a, Mode>>);

        pub mod methods {

            #[::validated_data::data_view(python(name = "EOSConfig.AaaAccounting.Exec.Default.MethodsItem", schema = Dict(30)))]
            pub struct Item<'a, Mode> {
                #[data_view(schema = Str(8))]
                pub method: ::validated_data::RequiredValue<&'a str, Mode>,
                #[data_view(schema = Str(9))]
                pub group: ::validated_data::Field<&'a str>,
            }
        }
    }
}

#[::validated_data::data_view(python(name = "EOSConfig.AaaAccounting.System", schema = Dict(34)))]
pub struct System<'a, Mode> {
    #[data_view(schema = Dict(33))]
    pub default: ::validated_data::Field<system::Default<'a, Mode>>,
}

pub mod system {

    #[::validated_data::data_view(python(name = "EOSConfig.AaaAccounting.System.Default", schema = Dict(33)))]
    pub struct Default<'a, Mode> {
        #[data_view(schema = Str(7), rename = "type")]
        pub field_type: ::validated_data::RequiredValue<&'a str, Mode>,
        #[data_view(schema = List(23))]
        pub methods: ::validated_data::Field<default::Methods<'a, Mode>>,
    }

    pub mod default {

        #[::validated_data::data_view(list, python(name = "EOSConfig.AaaAccounting.System.Default.Methods", schema = List(23)))]
        pub struct Methods<'a, Mode> (::validated_data::Field<methods::Item<'a, Mode>>);

        pub mod methods {

            #[::validated_data::data_view(python(name = "EOSConfig.AaaAccounting.System.Default.MethodsItem", schema = Dict(30)))]
            pub struct Item<'a, Mode> {
                #[data_view(schema = Str(8))]
                pub method: ::validated_data::RequiredValue<&'a str, Mode>,
                #[data_view(schema = Str(9))]
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

#[::validated_data::data_view(python(name = "EOSConfig.AaaAccounting.Commands", schema = Dict(39)))]
pub struct Commands<'a, Mode> {
    #[data_view(schema = List(25))]
    pub console: ::validated_data::Field<commands::Console<'a, Mode>>,
    #[data_view(schema = List(25))]
    pub default: ::validated_data::Field<commands::Default<'a, Mode>>,
}

pub mod commands {

    #[::validated_data::data_view(list, python(name = "EOSConfig.AaaAccounting.Commands.Console", schema = List(25)))]
    pub struct Console<'a, Mode> (::validated_data::Field<console::Item<'a, Mode>>);

    pub mod console {

        #[::validated_data::data_view(python(name = "EOSConfig.AaaAccounting.Commands.ConsoleItem", schema = Dict(38)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(12))]
            pub commands: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(7), rename = "type")]
            pub field_type: ::validated_data::RequiredValue<&'a str, Mode>,
            #[data_view(schema = List(23))]
            pub methods: ::validated_data::Field<item::Methods<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view(list, python(name = "EOSConfig.AaaAccounting.Commands.ConsoleItem.Methods", schema = List(23)))]
            pub struct Methods<'a, Mode> (::validated_data::Field<methods::Item<'a, Mode>>);

            pub mod methods {

                #[::validated_data::data_view(python(name = "EOSConfig.AaaAccounting.Commands.ConsoleItem.MethodsItem", schema = Dict(30)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Str(8))]
                    pub method: ::validated_data::RequiredValue<&'a str, Mode>,
                    #[data_view(schema = Str(9))]
                    pub group: ::validated_data::Field<&'a str>,
                }
            }
        }
    }

    #[::validated_data::data_view(list, python(name = "EOSConfig.AaaAccounting.Commands.Default", schema = List(25)))]
    pub struct Default<'a, Mode> (::validated_data::Field<default::Item<'a, Mode>>);

    pub mod default {

        #[::validated_data::data_view(python(name = "EOSConfig.AaaAccounting.Commands.DefaultItem", schema = Dict(38)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(12))]
            pub commands: ::validated_data::Field<&'a str>,
            #[data_view(schema = Str(7), rename = "type")]
            pub field_type: ::validated_data::RequiredValue<&'a str, Mode>,
            #[data_view(schema = List(23))]
            pub methods: ::validated_data::Field<item::Methods<'a, Mode>>,
        }

        pub mod item {

            #[::validated_data::data_view(list, python(name = "EOSConfig.AaaAccounting.Commands.DefaultItem.Methods", schema = List(23)))]
            pub struct Methods<'a, Mode> (::validated_data::Field<methods::Item<'a, Mode>>);

            pub mod methods {

                #[::validated_data::data_view(python(name = "EOSConfig.AaaAccounting.Commands.DefaultItem.MethodsItem", schema = Dict(30)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Str(8))]
                    pub method: ::validated_data::RequiredValue<&'a str, Mode>,
                    #[data_view(schema = Str(9))]
                    pub group: ::validated_data::Field<&'a str>,
                }
            }
        }
    }
}
