use std::cmp;
use std::path::PathBuf;
use std::time::SystemTime;

use which::which;

use crate::cli::arg;
use crate::cson::get_cson_config;
use crate::object::output::{self, Object, ObjectCollection, SharedLib, StaticLib};
use crate::object::source::Source;
use crate::toolchain::compiler::{Compiler, CompilerPair};
use crate::toolchain::linker::Linker;

pub struct GNU{
    #[allow(dead_code)] data: ()
}

impl Compiler for GNU {
    const DEFINE_FLAG_PREFIX:   &'static str = "-D";
    const INCLUDE_FLAG_PREFIX:  &'static str = "-I";
    const LINK_DIR_FLAG_PREFIX: &'static str = "-L";
    const LINK_LIB_FLAG_PREFIX: &'static str = "-l";

    fn get_compiler() -> CompilerPair {
        const GNU_CC:  &str = "gcc";
        const GNU_CXX: &str = "g++";

        let cc = which(GNU_CC)
            .expect("Failed to find gcc")
            .to_str().unwrap().to_string();

        let cxx = which(GNU_CXX)
            .expect("Failed to find g++")
            .to_str().unwrap().to_string();
        

        CompilerPair {
            cc: cc,
            cxx: cxx,
        }
    }

    fn compile(src: Source) -> Object {
        let cc_pair = Self::get_compiler();

        let cc = cc_pair.cc.clone();
        let cxx = cc_pair.cxx.clone();

        let src_path = src.get_path();

        let obj_sub_path = pathdiff::diff_paths(&src_path, arg::get_args().project_dir);

        let obj_path = get_cson_config()
            .read()
            .unwrap()
            .build_dir
            .join(obj_sub_path.unwrap())
            .with_extension("o");

        if !obj_path.parent().expect("Failed to get the folder of object file").exists() {
            std::fs::create_dir_all(obj_path.parent().unwrap()).expect("Failed to create object file directory");
        }

        if obj_path.exists() {
            let metadata = obj_path.metadata().unwrap();
            let modified = match metadata.modified() {
                Ok(modified) => Some(modified),
                Err(_) => None,
            };

            if src.modified.cmp(&modified) == cmp::Ordering::Less {
                return output::Object {
                    path: obj_path,
                    modified: modified,
                };
            }  
        }

        let compiler = match src_path.extension().unwrap().to_str().unwrap() {
            "c" => &cc,
            "cpp" | "cxx" | "cc" => &cxx,
             ext => panic!("Unsupported source file extension: {}", ext),
        };

        let status = std::process::Command::new(compiler)
            .arg("-g")
            .arg("-c")
            .arg(src_path.to_str().unwrap())
            .arg("-o")
            .arg(obj_path.to_str().unwrap())
            .args(get_cson_config().read().unwrap().get_define_args::<Self>())
            .args(get_cson_config().read().unwrap().get_include_dir_args::<Self>())
            .status()
            .expect(format!("Failed to compile {}", src_path.to_str().unwrap()).as_str());

        if status.success() {
            println!("Compiled {} to {}", src_path.to_str().unwrap(), obj_path.to_str().unwrap());
        } else {
            panic!("Failed to compile {}", src_path.to_str().unwrap());
        }
    
        output::Object {
            path: obj_path,
            modified: Some(SystemTime::now()),
        }
        
    }
}


impl Linker for GNU {
    fn get_linker() -> Option<String> {
        const GNU_LD: &str = "g++";
        return match which(GNU_LD) {
            Ok(ld_path) => {
                Some(ld_path.to_str().unwrap().to_string())
            },
            Err(_) => { None }
        };
    }

    fn link_to_object(input: ObjectCollection) -> Option<Object> {
        return None;
    }

    fn link_to_execuable(input: ObjectCollection) -> () {
        let linker = Self::get_linker().expect("Failed to find linker");
        let output_dir = &get_cson_config().read().unwrap().output_dir;

        let target_name = &get_cson_config().read().unwrap().target_name;
        let output_path = output_dir.join(PathBuf::from(target_name));

        std::process::Command::new(linker)
            .arg("-g")
            .args(input.to_args())
            .arg("-o")
            .arg(output_path.to_str().unwrap())
            .args(get_cson_config().read().unwrap().get_link_dir_args::<Self>())
            .args(get_cson_config().read().unwrap().get_lib_args::<Self>())
            .status()
            .expect(format!("Failed to link executable {}", output_path.to_str().unwrap()).as_str());
    }

    fn link_to_static_lib(input: ObjectCollection) -> Option<StaticLib> {
        return None;
    }

    fn link_to_dynamic_lib(input: ObjectCollection) -> Option<SharedLib> {
        return None;
    }
}

#[test]
fn test_gnu_compile() {
    // toolchain::compiler::compile::<GNU>(Source::new(PathBuf::from("./test/project/func.cpp").as_path()));
}