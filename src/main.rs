use std::{env::current_dir, error::Error, fs, path::PathBuf, process::Stdio};

use crate::{cli::arg, object::{output::ObjectCollection, source::Source}, toolchain::gnu::GNU};

pub mod cli {
    // pub mod app;
    pub mod arg;
}

pub mod compile {

}

pub mod object {
    pub mod output;
    pub mod source;
}
pub mod toolchain;

pub mod utils;

pub mod cson;

fn main() -> () {
    let cson = cson::get_cson_config();

    let source_paths = cson.read().unwrap().sources.clone().expect("No source files specified in cson configuration");

    let mut objects = ObjectCollection{
        objects: Vec::new(),
    };
    for path in source_paths {
        let source = Source::new(&path);
        let obj = toolchain::compiler::compile::<GNU>(source);
        objects.objects.push(obj);
    }
    toolchain::linker::link_to_execuable::<GNU>(objects);
}
