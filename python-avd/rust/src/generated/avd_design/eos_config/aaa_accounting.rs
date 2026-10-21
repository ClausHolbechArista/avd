// Copyright (c) 2026 Arista Networks, Inc.
// Generated from the AVD schema. Do not edit by hand.

#[::data_views::data_view]
pub struct AaaAccounting<'a, Mode> {
    pub exec: ::data_views::Field<Exec<'a, Mode>>,
    pub system: ::data_views::Field<System<'a, Mode>>,
    pub dot1x: ::data_views::Field<Dot1x<'a, Mode>>,
    pub commands: ::data_views::Field<Commands<'a, Mode>>,
}

#[::data_views::data_view(python(name = "EOSConfig.AaaAccounting.Exec", schema = Dict(32)))]
pub struct Exec<'a, Mode> {
    #[data_view(schema = Dict(31))]
    pub console: ::data_views::Field<exec::Console<'a, Mode>>,
    #[data_view(schema = Dict(31))]
    pub default: ::data_views::Field<exec::Default<'a, Mode>>,
}

pub mod exec {

    #[::data_views::data_view(python(name = "EOSConfig.AaaAccounting.Exec.Console", schema = Dict(31)))]
    pub struct Console<'a, Mode> {
        #[data_view(schema = Str(7), rename = "type")]
        pub field_type: ::data_views::RequiredValue<&'a str, Mode>,
        #[data_view(schema = List(23))]
        pub methods: ::data_views::Field<console::Methods<'a, Mode>>,
    }

    pub mod console {

        #[::data_views::data_view(list, python(name = "EOSConfig.AaaAccounting.Exec.Console.Methods", schema = List(23)))]
        pub struct Methods<'a, Mode> (::data_views::Field<methods::Item<'a, Mode>>);

        pub mod methods {

            #[::data_views::data_view(python(name = "EOSConfig.AaaAccounting.Exec.Console.MethodsItem", schema = Dict(30)))]
            pub struct Item<'a, Mode> {
                #[data_view(schema = Str(8))]
                pub method: ::data_views::RequiredValue<&'a str, Mode>,
                #[data_view(schema = Str(9))]
                pub group: ::data_views::Field<&'a str>,
            }
        }
    }

    #[::data_views::data_view(python(name = "EOSConfig.AaaAccounting.Exec.Default", schema = Dict(31)))]
    pub struct Default<'a, Mode> {
        #[data_view(schema = Str(7), rename = "type")]
        pub field_type: ::data_views::RequiredValue<&'a str, Mode>,
        #[data_view(schema = List(23))]
        pub methods: ::data_views::Field<default::Methods<'a, Mode>>,
    }

    pub mod default {

        #[::data_views::data_view(list, python(name = "EOSConfig.AaaAccounting.Exec.Default.Methods", schema = List(23)))]
        pub struct Methods<'a, Mode> (::data_views::Field<methods::Item<'a, Mode>>);

        pub mod methods {

            #[::data_views::data_view(python(name = "EOSConfig.AaaAccounting.Exec.Default.MethodsItem", schema = Dict(30)))]
            pub struct Item<'a, Mode> {
                #[data_view(schema = Str(8))]
                pub method: ::data_views::RequiredValue<&'a str, Mode>,
                #[data_view(schema = Str(9))]
                pub group: ::data_views::Field<&'a str>,
            }
        }
    }
}

#[::data_views::data_view(python(name = "EOSConfig.AaaAccounting.System", schema = Dict(34)))]
pub struct System<'a, Mode> {
    #[data_view(schema = Dict(33))]
    pub default: ::data_views::Field<system::Default<'a, Mode>>,
}

pub mod system {

    #[::data_views::data_view(python(name = "EOSConfig.AaaAccounting.System.Default", schema = Dict(33)))]
    pub struct Default<'a, Mode> {
        #[data_view(schema = Str(7), rename = "type")]
        pub field_type: ::data_views::RequiredValue<&'a str, Mode>,
        #[data_view(schema = List(23))]
        pub methods: ::data_views::Field<default::Methods<'a, Mode>>,
    }

    pub mod default {

        #[::data_views::data_view(list, python(name = "EOSConfig.AaaAccounting.System.Default.Methods", schema = List(23)))]
        pub struct Methods<'a, Mode> (::data_views::Field<methods::Item<'a, Mode>>);

        pub mod methods {

            #[::data_views::data_view(python(name = "EOSConfig.AaaAccounting.System.Default.MethodsItem", schema = Dict(30)))]
            pub struct Item<'a, Mode> {
                #[data_view(schema = Str(8))]
                pub method: ::data_views::RequiredValue<&'a str, Mode>,
                #[data_view(schema = Str(9))]
                pub group: ::data_views::Field<&'a str>,
            }
        }
    }
}

#[::data_views::data_view]
pub struct Dot1x<'a, Mode> {
    pub default: ::data_views::Field<dot1x::Default<'a, Mode>>,
}

pub mod dot1x {

    #[::data_views::data_view]
    pub struct Default<'a, Mode> {
        #[data_view(rename = "type")]
        pub field_type: ::data_views::RequiredValue<&'a str, Mode>,
        pub methods: ::data_views::Field<default::Methods<'a, Mode>>,
    }

    pub mod default {

        #[::data_views::data_view(list)]
        pub struct Methods<'a, Mode> (::data_views::Field<methods::Item<'a, Mode>>);

        pub mod methods {

            #[::data_views::data_view]
            pub struct Item<'a, Mode> {
                pub multicast: ::data_views::Field<bool>,
                pub method: ::data_views::RequiredValue<&'a str, Mode>,
                pub group: ::data_views::Field<&'a str>,
            }
        }
    }
}

#[::data_views::data_view(python(name = "EOSConfig.AaaAccounting.Commands", schema = Dict(39)))]
pub struct Commands<'a, Mode> {
    #[data_view(schema = List(25))]
    pub console: ::data_views::Field<commands::Console<'a, Mode>>,
    #[data_view(schema = List(25))]
    pub default: ::data_views::Field<commands::Default<'a, Mode>>,
}

pub mod commands {

    #[::data_views::data_view(list, python(name = "EOSConfig.AaaAccounting.Commands.Console", schema = List(25)))]
    pub struct Console<'a, Mode> (::data_views::Field<console::Item<'a, Mode>>);

    pub mod console {

        #[::data_views::data_view(python(name = "EOSConfig.AaaAccounting.Commands.ConsoleItem", schema = Dict(38)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(12))]
            pub commands: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(7), rename = "type")]
            pub field_type: ::data_views::RequiredValue<&'a str, Mode>,
            #[data_view(schema = List(23))]
            pub methods: ::data_views::Field<item::Methods<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view(list, python(name = "EOSConfig.AaaAccounting.Commands.ConsoleItem.Methods", schema = List(23)))]
            pub struct Methods<'a, Mode> (::data_views::Field<methods::Item<'a, Mode>>);

            pub mod methods {

                #[::data_views::data_view(python(name = "EOSConfig.AaaAccounting.Commands.ConsoleItem.MethodsItem", schema = Dict(30)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Str(8))]
                    pub method: ::data_views::RequiredValue<&'a str, Mode>,
                    #[data_view(schema = Str(9))]
                    pub group: ::data_views::Field<&'a str>,
                }
            }
        }
    }

    #[::data_views::data_view(list, python(name = "EOSConfig.AaaAccounting.Commands.Default", schema = List(25)))]
    pub struct Default<'a, Mode> (::data_views::Field<default::Item<'a, Mode>>);

    pub mod default {

        #[::data_views::data_view(python(name = "EOSConfig.AaaAccounting.Commands.DefaultItem", schema = Dict(38)))]
        pub struct Item<'a, Mode> {
            #[data_view(schema = Str(12))]
            pub commands: ::data_views::Field<&'a str>,
            #[data_view(schema = Str(7), rename = "type")]
            pub field_type: ::data_views::RequiredValue<&'a str, Mode>,
            #[data_view(schema = List(23))]
            pub methods: ::data_views::Field<item::Methods<'a, Mode>>,
        }

        pub mod item {

            #[::data_views::data_view(list, python(name = "EOSConfig.AaaAccounting.Commands.DefaultItem.Methods", schema = List(23)))]
            pub struct Methods<'a, Mode> (::data_views::Field<methods::Item<'a, Mode>>);

            pub mod methods {

                #[::data_views::data_view(python(name = "EOSConfig.AaaAccounting.Commands.DefaultItem.MethodsItem", schema = Dict(30)))]
                pub struct Item<'a, Mode> {
                    #[data_view(schema = Str(8))]
                    pub method: ::data_views::RequiredValue<&'a str, Mode>,
                    #[data_view(schema = Str(9))]
                    pub group: ::data_views::Field<&'a str>,
                }
            }
        }
    }
}
