use std::{collections::VecDeque, sync::{Arc, Mutex}, thread};

use crate::{cxon::get_cxon_config, object::{output::ObjectCollection, source::Source}, toolchain::{ToolChain, ToolChainTrait, compiler, gnu::GNU, linker, llvm::LLVM, msvc::MSVC}};

pub mod cli {
    pub mod arg;
}
pub mod object {
    pub mod output;
    pub mod source;
}
pub mod toolchain;
pub mod utils;
pub mod cxon;

fn main() -> () {
    let toolchain = get_cxon_config()
        .read()
        .unwrap()
        .get_toolchain();
    
    match toolchain {
        ToolChain::GNU()  => build_project::<GNU>(),
        ToolChain::LLVM() => build_project::<LLVM>(),
        ToolChain::MSVC() => build_project::<MSVC>(),
    }
}

fn build_project<T: ToolChainTrait>() {
    let cxon = cxon::get_cxon_config();

    let sources = cxon
        .read()
        .unwrap()
        .sources
        .clone()
        .expect("No source files specified in cxon configuration");

    let sources = Arc::new(Mutex::new(VecDeque::from(sources)));

    let objects = ObjectCollection{
        objects: Vec::new(),
    };
    let objects = Arc::new(Mutex::new(objects));

    let thread_count = match cxon.read().unwrap().threads {
        Some(count) => count,
        None => num_cpus::get() as usize - 1,
    };

    let mut compile_threads = Vec::new();

    for _ in 0..thread_count {
        let sources = sources.clone();
        let objects = objects.clone();

        compile_threads.push(thread::spawn(move || {
            while let Some(source) = sources.lock().unwrap().pop_back() {
                let source = Source::new(source.clone().as_path());
                let obj = compiler::compile::<T>(source);
                objects.lock().unwrap().objects.push(obj);
            }
        }));
    }

    for thread in compile_threads {
        thread.join().unwrap();
    }

    linker::link::<T>(objects.lock().unwrap().clone(), 
        get_cxon_config()
        .read()
        .unwrap()
        .get_target_type()
    );
}
