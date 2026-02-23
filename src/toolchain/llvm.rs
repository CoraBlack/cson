use crate::toolchain::ToolChainTrait;

pub struct LLVM {
    #[allow(dead_code)] data: ()
}

impl ToolChainTrait for LLVM {
    const CC:     &'static str = "clang";
    const CXX:    &'static str = "clang++";
    const DEBUG_FLAG:  &'static str = "-g";

    const EXECUTABLE_LINKER: &'static str = "clang++";
    // #[cfg(windows)]
    // const STATIC_LIB_LINKER: &'static str = "llvm-lib";
    // #[cfg(not(windows))] 
    const STATIC_LIB_LINKER: &'static str = "ar";
    const SHARED_LIB_LINKER: &'static str = "clang++";
    const OBJECT_LIB_LINKER: &'static str = "clang++";

    const EXECUTABLE_OUTPUT_FLAG: &'static str = "-o";
    const STATIC_LIB_OUTPUT_FLAG: &'static str = "rcs";
    const SHARED_LIB_OUTPUT_FLAG: &'static str = "-shared -fPIC -o";
    const OBJECT_LIB_OUTPUT_FLAG: &'static str = "-o";

    const EXECUTABLE_EXTENSION: &'static str = "";
    const STATIC_LIB_EXTENSION: &'static str = "a";
    const SHARED_LIB_EXTENSION: &'static str = "so";
    const OBJECT_LIB_EXTENSION: &'static str = "o";

    const ONLY_COMPILE_FLAG:    &'static str = "-c";
    const DEFINE_FLAG_PREFIX:   &'static str = "-D";
    const INCLUDE_FLAG_PREFIX:  &'static str = "-I";
    const LINK_DIR_FLAG_PREFIX: &'static str = "-L";
    const LINK_LIB_FLAG_PREFIX: &'static str = "-l";
}