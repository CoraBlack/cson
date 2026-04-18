//! Toolchain abstraction layer.
//!
//! `ToolChainTrait` centralizes command names, flags, and file extensions
//! for GNU/LLVM/MSVC implementations.

use crate::utils;

pub mod compiler;
pub mod gnu;
pub mod linker;
pub mod llvm;
pub mod msvc;

pub enum ToolChain {
    GNU(),
    LLVM(),
    MSVC(),
}

pub enum TargetType {
    ObjectLib,
    Executable,
    StaticLib,
    SharedLib,
}

pub trait ToolChainTrait {
    const CC: &'static str;
    const CXX: &'static str;
    const DEBUG_FLAG: &'static str;

    const EXECUTABLE_LINKER: &'static str;
    const STATIC_LIB_LINKER: &'static str;
    const SHARED_LIB_LINKER: &'static str;
    const OBJECT_LIB_LINKER: &'static str;

    const EXECUTABLE_OUTPUT_FLAG: &'static str;
    const STATIC_LIB_OUTPUT_FLAG: &'static str;
    const SHARED_LIB_OUTPUT_FLAG: &'static str;
    const OBJECT_LIB_OUTPUT_FLAG: &'static str;

    const EXECUTABLE_EXTENSION: &'static str;
    const STATIC_LIB_EXTENSION: &'static str;
    const SHARED_LIB_EXTENSION: &'static str;
    const OBJECT_LIB_EXTENSION: &'static str;

    const ONLY_COMPILE_FLAG: &'static str;
    const DEFINE_FLAG_PREFIX: &'static str;
    const INCLUDE_FLAG_PREFIX: &'static str;
    const LINK_DIR_FLAG_PREFIX: &'static str;
    const LINK_LIB_FLAG_PREFIX: &'static str;
}

/// Ensure required external binaries exist in `PATH` for selected toolchain.
pub fn check_toolchain_availability<T: ToolChainTrait>() -> () {
    utils::check_executable_exists(T::CC);
    utils::check_executable_exists(T::CXX);
    utils::check_executable_exists(T::EXECUTABLE_LINKER);
    utils::check_executable_exists(T::STATIC_LIB_LINKER);
    utils::check_executable_exists(T::SHARED_LIB_LINKER);
    utils::check_executable_exists(T::OBJECT_LIB_LINKER);
}
