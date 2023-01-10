extern crate askama;
extern crate regex;

use askama::Template;
use regex::Regex;

use std::fs;
use std::io::{prelude::*, BufReader, Write};

#[derive(Template)]
#[template(path = "native.jinja2")]
struct NativeTemplate<'a> {
    name: &'a str,
}

#[derive(Template)]
#[template(path = "scene.jinja2")]
struct SceneTemplate<'a> {
    name: &'a str,
    parent_class: &'a str,
}

fn main() {
    let regex = Regex::new(r"handle\.add_.*class::<(?:(?:.+)::)*(\w+)(?:>\(\))?;").unwrap();

    let file = fs::File::open("./src/lib.rs").unwrap();
    let reader = BufReader::new(file);

    // Create a Godot NativeScript for each class
    for line in reader.lines() {
        let line = line.unwrap();
        if let Some(captures) = regex.captures(line.as_str()) {
            let struct_name = &captures[1];
            let native = NativeTemplate { name: struct_name };

            fs::create_dir_all("../../native").unwrap();

            let file_name = format!("../../native/{}.gdns", struct_name);
            let file_content = native.render().unwrap();
            let file_content = file_content.as_bytes();

            let mut file = fs::File::create(file_name).unwrap();
            file.write_all(file_content).unwrap();
            file.write_all(b"\n").unwrap();
        }
    }

    // Create the main Scene
    // let struct_name = "Entrypoint";
    // let scene = SceneTemplate { name: "Entrypoint", parent_class: "Node" };

    // fs::create_dir_all("../../scenes").unwrap();

    // let file_name = format!("../../scenes/{}.tscn", struct_name);
    // let file_content = scene.render().unwrap();
    // let file_content = file_content.as_bytes();

    // let mut file = fs::File::create(file_name).unwrap();
    // file.write(file_content).unwrap();
}
