use gdnative::prelude::*;

use std::collections::HashMap;
use std::path::Path;

use argparse_gd::{
    argument::{Action, Argument},
    command::Command,
};

pub struct TransformCommand;

impl Command for TransformCommand {
    fn name(&self) -> &'static str {
        "transform"
    }

    fn description(&self) -> &'static str {
        "Apply various transformations to the 3D models in the batch, such as translation, rotation, or scaling."
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
        godot_print!("apply transform");
        Ok(())
    }
}
