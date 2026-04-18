//! CLI entrypoint for cxon.
//!
//! The runtime flow is:
//! 1. Parse CLI args and locate root `cxon.json`.
//! 2. Load the module graph recursively.
//! 3. Build the root project with the selected toolchain.

use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    thread,
};

use crate::{
    compile_commands_json::generate_compile_commands_json,
    cxon::CxonConfig,
    object::{output::ObjectCollection, source::Source},
    project_graph::load_module_graph,
    toolchain::{compiler, gnu::GNU, linker, llvm::LLVM, msvc::MSVC, ToolChain, ToolChainTrait},
};

pub mod cli {
    pub mod arg;
}
pub mod object {
    pub mod output;
    pub mod source;
}
pub mod compile_commands_json;
pub mod cxon;
pub mod project_graph;
pub mod toolchain;
pub mod utils;

fn main() -> () {
    // CLI accepts either a project directory or a direct `cxon.json` path.
    let args = cli::arg::get_args();
    let module_graph = load_module_graph(args.project_dir.as_path());
    let cxon = module_graph.config.clone();

    let toolchain = cxon.get_toolchain();

    match toolchain {
        ToolChain::GNU() => build_project::<GNU>(cxon),
        ToolChain::LLVM() => build_project::<LLVM>(cxon),
        ToolChain::MSVC() => build_project::<MSVC>(cxon),
    }
}

fn build_project<T: ToolChainTrait>(cxon: CxonConfig) {
    // Sources are canonicalized during config loading.
    // Empty is allowed for module-only projects.
    let sources = cxon.sources.clone().unwrap_or_default();

    let sources = Arc::new(Mutex::new(VecDeque::from(sources)));

    let objects = ObjectCollection {
        objects: Vec::new(),
    };
    let objects = Arc::new(Mutex::new(objects));

    let thread_count = match cxon.threads {
        Some(count) => count,
        None => std::cmp::max(1, num_cpus::get().saturating_sub(1)),
    };

    let mut compile_threads = Vec::new();

    for _ in 0..thread_count {
        let sources = sources.clone();
        let objects = objects.clone();
        let cxon = cxon.clone();

        compile_threads.push(thread::spawn(move || {
            while let Some(source) = sources.lock().unwrap().pop_back() {
                // Source path handling must use current project context.
                let source = Source::new(source.clone().as_path(), &cxon.project_dir);
                let obj = compiler::compile::<T>(source, &cxon);
                objects.lock().unwrap().objects.push(obj);
            }
        }));
    }

    for thread in compile_threads {
        thread.join().unwrap();
    }

    linker::link::<T>(
        objects.lock().unwrap().clone(),
        cxon.get_target_type(),
        &cxon,
    );

    // compile_commands.json is generated only when explicitly enabled.
    if cxon.export_compile_commands {
        generate_compile_commands_json(&cxon).expect("Failed to export compile_commands.json")
    }
}
