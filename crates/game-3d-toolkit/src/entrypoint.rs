use gdnative::prelude::*;

use argparse_gd::command::CommandParser;
use model_batch_processor::commands::transform::TransformCommand;

// TODO : faire en sorte que entrypoint soit ajouté en singleton, par un editor-like nod,e pour ne pas avoir besoin de déclarer de scene

#[derive(gdnative::derive::NativeClass)]
#[inherit(Node)]
pub struct Entrypoint;

#[gdnative::derive::methods]
impl Entrypoint {
    fn new(_owner: TRef<Node>) -> Self {
        Entrypoint
    }

    #[gdnative::derive::method]
    fn _enter_tree(&self, #[base] owner: TRef<Node>) {
        godot_print!("entrypoint");

        // Create a new command parser
        let mut parser = CommandParser::new();

        // Register some commands
        parser.register(Box::new(TransformCommand {}));

        // Parse and execute a command
        let result = parser.parse_and_execute();

        // Exit
        let tree = owner.get_tree().expect("Couldn't find scene tree!");
        let tree = unsafe { tree.assume_safe() };
        match result {
            Ok(_) => {
                tree.quit(0);
            }
            Err(err) => {
                godot_print!("{}", err);
                tree.quit(1);
            }
        }
    }
}
