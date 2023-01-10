extern crate argparse_gd;
extern crate gdnative;
extern crate model_batch_processor;

use gdnative::prelude::{godot_init, InitHandle};

pub mod entrypoint;

/// Registers all exposed classes to Godot.
fn init(handle: InitHandle) {
    handle.add_tool_class::<entrypoint::Entrypoint>();
}

// Macros that create the entry-points of the dynamic library.
godot_init!(init);
