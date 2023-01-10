use gdnative::prelude::*;

use std::collections::HashMap;
use std::path::Path;

use argparse_gd::{
    argument::{Action, Argument},
    command::Command,
};

pub struct OptimizeCommand;

impl Command for OptimizeCommand {
    fn name(&self) -> &'static str {
        "optimize"
    }

    fn description(&self) -> &'static str {
        "This operation might be used to remove any unnecessary geometry or vertices from the 3D models in the batch, in order to reduce their file size or simplify the models."
    }

    fn arguments(&self) -> Vec<Argument> {
        vec![Argument {
            name: "directory",
            default: None,
            required: true,
            action: Action::Store,
            validate: |value| {
                if Path::new(value.unwrap()).exists() {
                    Ok(())
                } else {
                    Err("Path doesn't exists".to_string())
                }
            },
        }]
    }

    fn execute(&self, _args: HashMap<&'static str, String>) -> Result<(), String> {
        godot_print!("apply optimize");
        Ok(())
    }
}
