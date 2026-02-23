use std::{cmp, path::PathBuf, time::SystemTime};

use crate::{cxon::get_cxon_config, object::{output::{self, Object}, source::Source}, toolchain::ToolChainTrait, utils::get_object_target_path
};

#[derive(Clone)]
pub struct CompilerPair {
    pub cc:  String,
    pub cxx: String,
}

pub struct CompileFuncArgs {
    pub src_path: PathBuf,
    pub obj_path: PathBuf,
    pub compiler: String,
    pub flags:    Vec<String>,
    pub defines:  Vec<String>,
    pub includes: Vec<String>,
}

pub fn compile<T: ToolChainTrait>(src: Source) -> Object {
    let obj_path = get_object_target_path::<T>(&src).expect("Failed to get the target path of object file");

    if !need_recompile(&src, &obj_path) {
        return Object { 
            path: obj_path.clone(),
            modified: Some(obj_path
                .metadata()
                .unwrap()
                .modified()
                .unwrap()
            )
        };
    }

    let is_c_file = src.get_path().extension().unwrap() == "c";

    let flags = if is_c_file {
        get_cxon_config().read().unwrap().get_cflags()
    } else {
        get_cxon_config().read().unwrap().get_cxxflags()
    };

    compile_handler::<T>(CompileFuncArgs {
        src_path: src.get_path().to_path_buf(),
        obj_path: obj_path.clone(),
        compiler: if is_c_file { T::CC.to_string() } else { T::CXX.to_string() },
        flags: flags,
        defines: get_cxon_config().read().unwrap().get_define_args::<T>(),
        includes: get_cxon_config().read().unwrap().get_include_dir_args::<T>(),
    })
}

fn compile_handler<T: ToolChainTrait>(args: CompileFuncArgs) -> Object {
    let status = std::process::Command::new(args.compiler)
        .arg(T::DEBUG_FLAG)
        .arg(T::ONLY_COMPILE_FLAG)
        .arg(args.src_path.to_str().unwrap())
        .arg(T::EXECUTABLE_OUTPUT_FLAG)
        .arg(args.obj_path.to_str().unwrap())
        .args(args.includes)
        .args(args.defines)
        .args(args.flags)
        .status()
        .expect(format!("Failed to compile {}", args.src_path.to_str().unwrap()).as_str());

    if status.success() {
        println!("Compiled {} to {}", args.src_path.to_str().unwrap(), args.obj_path.to_str().unwrap());
    } else {
        panic!("Failed to compile {}", args.src_path.to_str().unwrap());
    }

    output::Object {
        path: args.obj_path,
        modified: Some(SystemTime::now()),
    }
}

fn need_recompile(src: &Source, obj_path: &PathBuf) -> bool {
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