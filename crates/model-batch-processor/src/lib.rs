extern crate argparse_gd;
extern crate gdnative;

// use gdnative::{
//     api::{GLTFState, Node, PackedSceneGLTF},
//     core_types::{GodotError, GodotString},
//     object::{ownership::Shared, AsArg, Ref},
//     prelude::GodotObject,
// };

pub mod commands;

// struct Gltf {
//     pub data_node: Ref<Node, Shared>,
// }

// impl Gltf {
//     pub fn open(path: impl Into<GodotString>) -> Result<Self, GodotError> {
//         let scene = PackedSceneGLTF::new();
//         if let Some(gltf) = scene.import_gltf_scene(path, 0, 1000.0, 2194432, GLTFState::null()) {
//             return Ok(Self { data_node: gltf });
//         }
//         Err(GodotError::Failed)
//     }

//     fn save(node: impl AsArg<Node>, path: impl Into<GodotString>) -> Result<(), GodotError> {
//         let scene = PackedSceneGLTF::new();
//         scene.export_gltf(node, path, 0, 1000.0)
//     }
// }
