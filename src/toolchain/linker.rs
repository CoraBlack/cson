//! Link stage implementation.
//!
//! This module links object files into the configured target artifact.

use std::path::PathBuf;

use crate::{
    cxon::CxonConfig,
    error::{fail, fail_result},
    object::output::ObjectCollection,
    toolchain::{TargetType, ToolChainTrait},
};

struct LinkArgs {
    pub linker: String,
    pub output_path: PathBuf,

    pub output_flag: String,
    pub other_flags: Vec<String>,
    /// Runtime-injected dependency artifact files passed directly to linker.
    pub extra_link_files: Vec<String>,
    pub link_dir_args: Vec<String>,
    pub link_lib_args: Vec<String>,
}

/// Link all compiled objects into final output according to target type.
pub fn link<T: ToolChainTrait>(
    input: ObjectCollection,
    target_type: TargetType,
    cxon: &CxonConfig,
) -> () {
    let output_dir = &cxon.output_dir;
    let target_name = &cxon.get_target_name();
    let output_path = output_dir.join(PathBuf::from(target_name));

    let other_flags = Vec::new();

    match target_type {
        TargetType::Executable => link_to_executable_cmd::<T>(
            input,
            LinkArgs {
                linker: T::EXECUTABLE_LINKER.to_string(),
                output_path,
                output_flag: T::EXECUTABLE_OUTPUT_FLAG.to_string(),
                extra_link_files: cxon.get_extra_link_file_args(),
                link_dir_args: cxon.get_link_dir_args::<T>(),
                link_lib_args: cxon.get_lib_args::<T>(),
                other_flags,
            },
        ),
        TargetType::StaticLib => link_to_static_lib_cmd::<T>(
            input,
            LinkArgs {
                linker: T::STATIC_LIB_LINKER.to_string(),
                output_path,
                output_flag: T::STATIC_LIB_OUTPUT_FLAG.to_string(),
                extra_link_files: cxon.get_extra_link_file_args(),
                link_dir_args: cxon.get_link_dir_args::<T>(),
                link_lib_args: cxon.get_lib_args::<T>(),
                other_flags,
            },
        ),
        TargetType::SharedLib => link_to_shared_lib_cmd::<T>(
            input,
            LinkArgs {
                linker: T::SHARED_LIB_LINKER.to_string(),
                output_path,
                output_flag: T::SHARED_LIB_OUTPUT_FLAG.to_string(),
                extra_link_files: cxon.get_extra_link_file_args(),
                link_dir_args: cxon.get_link_dir_args::<T>(),
                link_lib_args: cxon.get_lib_args::<T>(),
                other_flags,
            },
        ),
        TargetType::ObjectLib => link_to_object_cmd::<T>(
            input,
            LinkArgs {
                linker: T::OBJECT_LIB_LINKER.to_string(),
                output_path,
                output_flag: T::OBJECT_LIB_OUTPUT_FLAG.to_string(),
                extra_link_files: cxon.get_extra_link_file_args(),
                link_dir_args: cxon.get_link_dir_args::<T>(),
                link_lib_args: cxon.get_lib_args::<T>(),
                other_flags,
            },
        ),
    }
}

/// Link objects into an executable target.
fn link_to_executable_cmd<T: ToolChainTrait>(input: ObjectCollection, args: LinkArgs) -> () {
    let status = std::process::Command::new(args.linker)
        .args(input.to_args())
        .arg(args.output_flag)
        .arg(
            args.output_path
                .with_added_extension(T::EXECUTABLE_EXTENSION)
                .to_string_lossy()
                .to_string(),
        )
        .args(args.extra_link_files)
        .args(args.link_dir_args)
        .args(args.link_lib_args)
        .args(args.other_flags)
        .status();

    let status = fail_result(
        status,
        format!("failed to invoke linker for {}", args.output_path.display()),
    );

    if !status.success() {
        fail(format!(
            "failed to link executable {}",
            args.output_path.display()
        ));
    }
}

/// Archive or link objects into a static library target.
fn link_to_static_lib_cmd<T: ToolChainTrait>(input: ObjectCollection, args: LinkArgs) -> () {
    let status = std::process::Command::new(args.linker)
        .args(args.output_flag.split(' '))
        .arg(
            args.output_path
                .with_extension(T::STATIC_LIB_EXTENSION)
                .to_string_lossy()
                .to_string(),
        )
        .args(args.extra_link_files)
        .args(input.to_args())
        .args(args.link_dir_args)
        .args(args.link_lib_args)
        .args(args.other_flags)
        .status();

    let status = fail_result(
        status,
        format!("failed to invoke linker for {}", args.output_path.display()),
    );

    if !status.success() {
        fail(format!(
            "failed to link static library {}",
            args.output_path.display()
        ));
    }
}

/// Link objects into a shared library target.
fn link_to_shared_lib_cmd<T: ToolChainTrait>(input: ObjectCollection, args: LinkArgs) -> () {
    let status = std::process::Command::new(args.linker)
        .args(args.output_flag.split(' '))
        .arg(
            args.output_path
                .with_extension(T::SHARED_LIB_EXTENSION)
                .to_string_lossy()
                .to_string(),
        )
        .args(args.extra_link_files)
        .args(input.to_args())
        .args(args.link_dir_args)
        .args(args.link_lib_args)
        .args(args.other_flags)
        .status();

    let status = fail_result(
        status,
        format!("failed to invoke linker for {}", args.output_path.display()),
    );

    if !status.success() {
        fail(format!(
            "failed to link shared library {}",
            args.output_path.display()
        ));
    }
}

/// Link objects into an object-library-like output target.
fn link_to_object_cmd<T: ToolChainTrait>(input: ObjectCollection, args: LinkArgs) -> () {
    let status = std::process::Command::new(args.linker)
        .args(input.to_args())
        .args(args.output_flag.split(' '))
        .arg(
            args.output_path
                .with_extension(T::OBJECT_LIB_EXTENSION)
                .to_string_lossy()
                .to_string(),
        )
        .args(args.extra_link_files)
        .args(args.link_dir_args)
        .args(args.link_lib_args)
        .args(args.other_flags)
        .status();

    let status = fail_result(
        status,
        format!("failed to invoke linker for {}", args.output_path.display()),
    );

    if !status.success() {
        fail(format!(
            "failed to link object file {}",
            args.output_path.display()
        ));
    }
}
