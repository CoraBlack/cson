//! Compile stage implementation.
//!
//! This module takes resolved source files and produces object files.

use std::{cmp, path::PathBuf, time::SystemTime};

use crate::{
    compile_commands_json::{add_compile_command, CompileCommand},
    cxon::CxonConfig,
    object::{
        output::{self, Object},
        source::Source,
    },
    toolchain::ToolChainTrait,
    utils::{self, get_object_target_path},
};

#[derive(Clone)]
pub struct CompilerPair {
    pub cc: String,
    pub cxx: String,
}

struct CompileFuncArgs {
    pub src_path: PathBuf,
    pub obj_path: PathBuf,
    pub compiler: String,
    pub flags: Vec<String>,
    pub defines: Vec<String>,
    pub includes: Vec<String>,
    pub project_dir: PathBuf,
}

/// Compile one source file into one object file.
///
/// The project context is provided explicitly through `cxon` so this function
/// can be reused for module-aware builds.
pub fn compile<T: ToolChainTrait>(src: Source, cxon: &CxonConfig) -> Object {
    let obj_path = get_object_target_path::<T>(&src, &cxon.project_dir, &cxon.build_dir)
        .expect("Failed to get the target path of object file");

    if !need_recompile(&src, &obj_path) {
        return Object {
            path: obj_path.clone(),
            modified: Some(obj_path.metadata().unwrap().modified().unwrap()),
        };
    }

    let is_c_file = src.get_path().extension().unwrap() == "c";

    // get compiler flags
    let mut flags = if is_c_file {
        cxon.get_cflags()
    } else {
        cxon.get_cxxflags()
    };

    // get debug flag
    if cxon.get_debug_flag() {
        flags.push(T::DEBUG_FLAG.to_string());
    }

    compile_handler::<T>(CompileFuncArgs {
        src_path: src.get_path().to_path_buf(),
        obj_path: obj_path.clone(),
        compiler: if is_c_file {
            T::CC.to_string()
        } else {
            T::CXX.to_string()
        },
        flags: flags,
        defines: cxon.get_define_args::<T>(),
        includes: cxon.get_include_dir_args::<T>(),
        project_dir: cxon.project_dir.clone(),
    })
}

fn compile_handler<T: ToolChainTrait>(args: CompileFuncArgs) -> Object {
    let mut cmd = std::process::Command::new(args.compiler);
    let cmd = cmd
        .arg(T::ONLY_COMPILE_FLAG)
        .arg(args.src_path.to_str().unwrap())
        .arg(T::EXECUTABLE_OUTPUT_FLAG)
        .arg(args.obj_path.to_str().unwrap())
        .args(args.includes)
        .args(args.defines)
        .args(args.flags);

    let status = cmd
        .spawn()
        .expect(format!("Failed to compile {}", args.src_path.to_str().unwrap()).as_str());

    // Record compile command for optional compile_commands.json export.
    let mut compile_command = CompileCommand::from_source(
        Source::new(&args.src_path, &args.project_dir),
        &args.project_dir,
    );
    compile_command.command = utils::get_command_string(&cmd);

    add_compile_command(compile_command);

    let output = status.wait_with_output().expect(
        format!(
            "Failed to wait for the compilation process of {}",
            args.src_path.to_str().unwrap()
        )
        .as_str(),
    );

    if output.status.success() {
        println!(
            "Compiled {} to {}",
            args.src_path.to_str().unwrap(),
            args.obj_path.to_str().unwrap()
        );
    } else {
        panic!("Failed to compile {}", args.src_path.to_str().unwrap());
    }

    output::Object {
        path: args.obj_path,
        modified: Some(SystemTime::now()),
    }
}

fn need_recompile(src: &Source, obj_path: &PathBuf) -> bool {
    // Reuse existing object when source timestamp is older than object.
    if obj_path.exists() {
        let metadata = obj_path.metadata().unwrap();
        let Ok(modified) = metadata.modified() else {
            return true;
        };

        if src.modified.cmp(&Some(modified)) == cmp::Ordering::Less {
            return false;
        }
    }

    true
}
