use std::collections::HashMap;

use crate::argument::{Action, ArgParser, Argument, Subparser};

// The Command trait represents a command that can be
// executed with a set of arguments.
pub trait Command {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn arguments(&self) -> Vec<Argument>;
    fn execute(&self, args: HashMap<&'static str, String>) -> Result<(), String>;
}

// The CommandParser struct is used to parse and execute commands.
pub struct CommandParser {
    pub commands: HashMap<String, Box<dyn Command>>,
    pub argparser: ArgParser,
}

impl Default for CommandParser {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandParser {
    pub fn new() -> CommandParser {
        CommandParser {
            commands: HashMap::new(),
            argparser: ArgParser {
                subcommands: HashMap::new(),
                arguments: HashMap::from([
                    (
                        "help",
                        Argument {
                            name: "help",
                            default: Some("false".to_string()),
                            required: false,
                            action: Action::StoreTrue,
                            validate: |_| Ok(()),
                        },
                    ),
                    (
                        "version",
                        Argument {
                            name: "version",
                            default: Some("false".to_string()),
                            required: false,
                            action: Action::StoreTrue,
                            validate: |_| Ok(()),
                        },
                    ),
                ]),
            },
        }
    }

    // The register function registers a command with the parser.
    pub fn register(&mut self, command: Box<dyn Command>) {
        let mut sub_argparser = Subparser {
            name: command.name(),
            arguments: HashMap::new(),
        };

        for argument in command.arguments() {
            sub_argparser.arguments.insert(argument.name, argument);
        }

        self.argparser.add_subparser(sub_argparser);
        self.commands.insert(command.name().to_string(), command);
    }

    // The parse_and_execute function parses and executes a command with the given arguments.
    pub fn parse_and_execute(&self) -> Result<(), String> {
        let args = self.argparser.parse_args()?;

        match self.commands.get(&args.0) {
            Some(command) => command.execute(args.1),
            None => Err(format!("Unknown command: '{}'", args.0)),
        }
    }
}
