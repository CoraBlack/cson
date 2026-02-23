use std::path::PathBuf;

use crate::{cxon::get_cxon_config, object::output::ObjectCollection, toolchain::{TargetType, ToolChainTrait}};

pub struct LinkArgs {
    pub linker: String,
    pub output_path: PathBuf,

    pub output_flag: String,
    pub other_flags: Vec<String>,
    pub link_dir_args: Vec<String>,
    pub link_lib_args: Vec<String>,
}

pub fn link<T: ToolChainTrait>(input: ObjectCollection, target_type: TargetType) -> () {
    let output_dir = &get_cxon_config().read().unwrap().output_dir;
    let target_name = &get_cxon_config().read().unwrap().get_target_name();
    let output_path = output_dir.join(PathBuf::from(target_name));

    let mut other_flags = Vec::new();

    match target_type {
        TargetType::Executable => link_to_executable_cmd::<T>(input, LinkArgs {
            linker:        T::EXECUTABLE_LINKER.to_string(),
            output_path,
            output_flag:   T::EXECUTABLE_OUTPUT_FLAG.to_string(),
            link_dir_args: get_cxon_config().read().unwrap().get_link_dir_args::<T>(),
            link_lib_args: get_cxon_config().read().unwrap().get_lib_args::<T>(),
            other_flags,
        }),
        TargetType::StaticLib  => link_to_static_lib_cmd::<T>(input, LinkArgs {
            linker:        T::STATIC_LIB_LINKER.to_string(),
            output_path,
            output_flag:   T::STATIC_LIB_OUTPUT_FLAG.to_string(),
            link_dir_args: get_cxon_config().read().unwrap().get_link_dir_args::<T>(),
            link_lib_args: get_cxon_config().read().unwrap().get_lib_args::<T>(),
            other_flags,
        }),
        TargetType::SharedLib  => link_to_shared_lib_cmd::<T>(input, LinkArgs {
            linker:        T::SHARED_LIB_LINKER.to_string(),
            output_path,
            output_flag:   T::SHARED_LIB_OUTPUT_FLAG.to_string(),
            link_dir_args: get_cxon_config().read().unwrap().get_link_dir_args::<T>(),
            link_lib_args: get_cxon_config().read().unwrap().get_lib_args::<T>(),
            other_flags,
        }),
        TargetType::ObjectLib  => link_to_object_cmd::<T>(input, LinkArgs {
            linker:        T::OBJECT_LIB_LINKER.to_string(),
            output_path,
            output_flag:   T::OBJECT_LIB_OUTPUT_FLAG.to_string(),
            link_dir_args: get_cxon_config().read().unwrap().get_link_dir_args::<T>(),
            link_lib_args: get_cxon_config().read().unwrap().get_lib_args::<T>(),
            other_flags,
        }),
    }
}

pub fn link_to_executable_cmd<T: ToolChainTrait>(input: ObjectCollection, args: LinkArgs) -> () {
    std::process::Command::new(T::EXECUTABLE_LINKER)
        .arg(T::DEBUG_FLAG)
        .args(input.to_args())
        .arg(T::EXECUTABLE_OUTPUT_FLAG)
        .arg(args.output_path.with_added_extension(T::EXECUTABLE_EXTENSION).to_str().unwrap())
        .args(args.link_dir_args)
        .args(args.link_lib_args)
        .status()
        .expect(format!("Failed to link executable {}", args.output_path.to_str().unwrap()).as_str());
}

pub fn link_to_static_lib_cmd<T: ToolChainTrait>(input: ObjectCollection, args: LinkArgs) -> () {
    std::process::Command::new(T::STATIC_LIB_LINKER)
        .args(T::STATIC_LIB_OUTPUT_FLAG.to_string().split(' '))
        .arg(args.output_path.with_extension(T::STATIC_LIB_EXTENSION).to_str().unwrap())
        .args(input.to_args())
        .args(args.link_dir_args)
        .args(args.link_lib_args)
        .status()
        .expect(format!("Failed to link static library {}", args.output_path.to_str().unwrap()).as_str());
}

pub fn link_to_shared_lib_cmd<T: ToolChainTrait>(input: ObjectCollection, args: LinkArgs) -> () {
    std::process::Command::new(T::SHARED_LIB_LINKER)
        .arg(T::DEBUG_FLAG)
        .args(T::SHARED_LIB_OUTPUT_FLAG.to_string().split(' '))
        .arg(args.output_path.with_extension(T::SHARED_LIB_EXTENSION).to_str().unwrap())
        .args(input.to_args())
        .args(args.link_dir_args)
        .args(args.link_lib_args)
        .status()
        .expect(format!("Failed to link shared library {}", args.output_path.to_str().unwrap()).as_str());
}

pub fn link_to_object_cmd<T: ToolChainTrait>(input: ObjectCollection, args: LinkArgs) -> () {
    std::process::Command::new(T::OBJECT_LIB_LINKER)
        .arg(T::DEBUG_FLAG)
        .args(input.to_args())
        .args(T::OBJECT_LIB_OUTPUT_FLAG.to_string().split(' '))
        .arg(args.output_path.with_extension(T::OBJECT_LIB_EXTENSION).to_str().unwrap())
        .args(args.link_dir_args)
        .args(args.link_lib_args)
        .status()
        .expect(format!("Failed to link object file {}", args.output_path.to_str().unwrap()).as_str());
}