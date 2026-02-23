use crate::toolchain::ToolChainTrait;

pub struct MSVC {
    #[allow(dead_code)] data: ()
}

impl ToolChainTrait for MSVC {
    const CC:     &'static str = "cl";
    const CXX:    &'static str = "cl";
    const DEBUG_FLAG:  &'static str = "/Zi";

    const EXECUTABLE_LINKER: &'static str = "cl";
    const STATIC_LIB_LINKER: &'static str = "lib";
    const SHARED_LIB_LINKER: &'static str = "link";
    const OBJECT_LIB_LINKER: &'static str = "lib";

    const EXECUTABLE_OUTPUT_FLAG: &'static str = "/Fe:";
    const STATIC_LIB_OUTPUT_FLAG: &'static str = "/OUT:";
    const SHARED_LIB_OUTPUT_FLAG: &'static str = "/OUT:";
    const OBJECT_LIB_OUTPUT_FLAG: &'static str = "/Fo:";

    const EXECUTABLE_EXTENSION: &'static str = "exe";
    const STATIC_LIB_EXTENSION: &'static str = "lib";
    const SHARED_LIB_EXTENSION: &'static str = "dll";
    const OBJECT_LIB_EXTENSION: &'static str = "obj";
    
    const ONLY_COMPILE_FLAG:    &'static str = "/c";
    const DEFINE_FLAG_PREFIX:   &'static str = "/D";
    const INCLUDE_FLAG_PREFIX:  &'static str = "/I";
    const LINK_DIR_FLAG_PREFIX: &'static str = "/LIBPATH:";
    const LINK_LIB_FLAG_PREFIX: &'static str = "/LD";
}