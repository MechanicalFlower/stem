// use gdnative::api::{EditorPlugin, Script, Texture};
// use gdnative::prelude::*;

// #[derive(NativeClass)]
// #[inherit(EditorPlugin)]
// pub struct RegisterEntrypoint;

// #[methods]
// impl RegisterEntrypoint {
//     fn new(_owner: TRef<EditorPlugin>) -> Self {
//         RegisterEntrypoint
//     }

//     #[method]
//     fn _enter_tree(&self, #[base] owner: TRef<EditorPlugin>) {
//         godot_print!("entrypoint");
//         owner.add_autoload_singleton("Entrypoint", "res://native/Entrypoint.gdns");
//     }

//     #[method]
//     fn _exit_tree(&self, #[base] owner: TRef<EditorPlugin>) {
//         owner.remove_autoload_singleton("Entrypoint");
//     }
// }
