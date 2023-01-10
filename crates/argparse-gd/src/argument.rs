use std::collections::HashMap;

use gdnative::api::OS;
use gdnative::prelude::*;

#[derive(PartialEq, Eq)]
pub enum Action {
    Store,
    StoreTrue,
    StoreFalse,
}

pub struct Argument {
    pub name: &'static str,
    pub default: Option<String>,
    pub required: bool,
    pub action: Action,
    pub validate: fn(Option<&str>) -> Result<(), String>,
}

pub struct Subparser {
    pub name: &'static str,
    pub arguments: HashMap<&'static str, Argument>,
}

pub struct ArgParser {
    pub subcommands: HashMap<&'static str, Subparser>,
    pub arguments: HashMap<&'static str, Argument>,
}

impl Default for ArgParser {
    fn default() -> Self {
        Self::new()
    }
}

impl ArgParser {
    pub fn new() -> Self {
        Self {
            subcommands: HashMap::new(),
            arguments: HashMap::new(),
        }
    }

    pub fn add_argument(&mut self, argument: Argument) {
        assert_eq!(
            argument.required,
            argument.default.is_some(),
            "May not set both `required` and `default`"
        );
        assert_eq!(
            argument.action == Action::StoreTrue,
            argument.default == Some("true".to_string())
        );
        assert_eq!(
            argument.action == Action::StoreFalse,
            argument.default == Some("false".to_string())
        );

        self.arguments.insert(argument.name, argument);
    }

    pub fn add_subparser(&mut self, subcommand: Subparser) {
        self.subcommands.insert(subcommand.name, subcommand);
    }

    pub fn parse_args(&self) -> Result<(String, HashMap<&'static str, String>), String> {
        let mut subcommand = String::new();
        let mut args = HashMap::new();
        let cmd_args = OS::godot_singleton().get_cmdline_args();

        for i in 0..cmd_args.len() {
            let mut arg_name = cmd_args.get(i).to_string();

            if arg_name.starts_with("--") {
                arg_name = arg_name[2..].to_string();
                let mut arg_value = None;

                let next_arg_name = cmd_args.get(i + 1).to_string();
                if (i + 1) < cmd_args.len() && !next_arg_name.starts_with("--") {
                    arg_value = Some(next_arg_name);
                }

                if let Some(argument) = self.arguments.get(&*arg_name) {
                    let is_valid = (argument.validate)(arg_value.as_deref());

                    match is_valid {
                        Ok(_) => match argument.action {
                            Action::Store => {
                                if argument.required && arg_value.is_none() {
                                    return Err("Missing required argument value".into());
                                }

                                args.insert(
                                    argument.name,
                                    arg_value.unwrap_or({
                                        if let Some(def) = &argument.default {
                                            def.to_string()
                                        } else {
                                            "".to_string()
                                        }
                                    }),
                                );
                            }
                            Action::StoreTrue => {
                                args.insert(
                                    argument.name,
                                    arg_value
                                        .map(|_| "true".to_string())
                                        .or_else(|| Some("false".to_string()))
                                        .unwrap(),
                                );
                            }
                            Action::StoreFalse => {
                                args.insert(
                                    argument.name,
                                    arg_value
                                        .map(|_| "false".to_string())
                                        .or_else(|| Some("true".to_string()))
                                        .unwrap(),
                                );
                            }
                        },
                        Err(err) => {
                            return Err(format!("Invalid argument value: {}", err));
                        }
                    }
                } else {
                    return Err(format!("Invalid argument: {}", arg_name));
                }
            } else if let Some(subparser) = self.subcommands.get(&*arg_name) {
                subcommand = arg_name;
                args = self.parse_subcommand_args(subparser, i + 1, cmd_args)?;
                break;
            } else {
                return Err(format!("Invalid argument: {}", arg_name));
            }
        }

        for argument in self.arguments.values() {
            if argument.required && !args.contains_key(argument.name) {
                return Err(format!("Missing required argument: {}", argument.name));
            }
        }

        Ok((subcommand, args))
    }

    fn parse_subcommand_args(
        &self,
        subcommand: &Subparser,
        start_index: i32,
        cmd_args: PoolArray<GodotString>,
    ) -> Result<HashMap<&'static str, String>, String> {
        let mut args = HashMap::new();

        for i in start_index..cmd_args.len() {
            let mut arg_name = cmd_args.get(i).to_string();

            if arg_name.starts_with("--") {
                arg_name = arg_name[2..].to_string();
                let mut arg_value = None;

                let next_arg_name = cmd_args.get(i + 1).to_string();
                if (i + 1) < cmd_args.len() && !next_arg_name.starts_with("--") {
                    arg_value = Some(next_arg_name);
                }

                if let Some(argument) = subcommand.arguments.get(&*arg_name) {
                    let is_valid = (argument.validate)(arg_value.as_deref());

                    match is_valid {
                        Ok(_) => match argument.action {
                            Action::Store => {
                                if argument.required && arg_value.is_none() {
                                    return Err("Missing required argument value".into());
                                }

                                args.insert(
                                    argument.name,
                                    arg_value.unwrap_or({
                                        if let Some(def) = &argument.default {
                                            def.to_string()
                                        } else {
                                            "".to_string()
                                        }
                                    }),
                                );
                            }
                            Action::StoreTrue => {
                                args.insert(
                                    argument.name,
                                    arg_value
                                        .map(|_| "true".to_string())
                                        .or_else(|| Some("false".to_string()))
                                        .unwrap(),
                                );
                            }
                            Action::StoreFalse => {
                                args.insert(
                                    argument.name,
                                    arg_value
                                        .map(|_| "false".to_string())
                                        .or_else(|| Some("true".to_string()))
                                        .unwrap(),
                                );
                            }
                        },
                        Err(err) => {
                            return Err(format!("Invalid argument value: {}", err));
                        }
                    }
                } else {
                    return Err(format!("Invalid argument: {}", arg_name));
                }
            }
        }

        for argument in subcommand.arguments.values() {
            if argument.required && !args.contains_key(argument.name) {
                return Err(format!("Missing required argument: {}", argument.name));
            }
        }

        Ok(args)
    }
}

// #[cfg(test)]
// mod tests {
//     extern crate mockall;

//     use self::mockall::predicate::*;
//     use self::mockall::*;
//     use super::*;

//     #[automock]
//     trait OS {
//         fn get_cmdline_args(&self) -> Vec<String>;
//     }

//     #[test]
//     fn test_parse_args() {
//         let mut mock = MockOS::new();

//         mock.expect_get_cmdline_args().returning(|| {
//             vec![
//                 "--foo".to_string(),
//                 "bar".to_string(),
//                 "--baz".to_string(),
//                 "--qux".to_string(),
//                 "quux".to_string(),
//             ]
//         });

//         let mut parser = ArgParser::new();
//         parser.add_argument(Argument {
//             name: "foo",
//             default: None,
//             action: Action::Store,
//             required: false,
//             validate: |_| Ok(()),
//         });
//         parser.add_argument(Argument {
//             name: "baz",
//             default: None,
//             action: Action::Store,
//             required: true,
//             validate: |_| Ok(()),
//         });
//         parser.add_argument(Argument {
//             name: "qux",
//             default: None,
//             action: Action::Store,
//             required: false,
//             validate: |_| Ok(()),
//         });

//         let (subcommand, args) = parser.parse_args().unwrap();
//         assert_eq!(subcommand, "", "{:?}", subcommand);
//         assert_eq!(args.get("foo").unwrap(), "bar");
//         assert_eq!(args.get("baz").unwrap(), "");
//         assert_eq!(args.get("qux").unwrap(), "quux");
//     }

//     #[test]
//     fn test_parse_args_with_subcommand() {
//         let mut mock = MockOS::new();

//         mock.expect_get_cmdline_args().returning(|| {
//             vec![
//                 "subcommand".to_string(),
//                 "--foo".to_string(),
//                 "bar".to_string(),
//                 "--baz".to_string(),
//                 "--qux".to_string(),
//                 "quux".to_string(),
//             ]
//         });

//         let mut sub_argparser = Subparser {
//             name: "subcommand",
//             arguments: HashMap::new(),
//         };

//         let arguments = vec![
//             Argument {
//                 name: "foo",
//                 default: None,
//                 action: Action::Store,
//                 required: false,
//                 validate: |_| Ok(()),
//             },
//             Argument {
//                 name: "baz",
//                 default: None,
//                 action: Action::Store,
//                 required: true,
//                 validate: |_| Ok(()),
//             },
//             Argument {
//                 name: "qux",
//                 default: None,
//                 action: Action::Store,
//                 required: false,
//                 validate: |_| Ok(()),
//             },
//         ];

//         for argument in arguments {
//             sub_argparser.arguments.insert(argument.name, argument);
//         }

//         let mut parser = ArgParser::new();
//         parser.add_subparser(sub_argparser);

//         let (subcommand, args) = parser.parse_args().unwrap();
//         assert_eq!(subcommand, "subcommand");
//         assert_eq!(args.get("foo").unwrap(), "bar");
//         assert_eq!(args.get("baz").unwrap(), "");
//         assert_eq!(args.get("qux").unwrap(), "quux");
//     }
// }
