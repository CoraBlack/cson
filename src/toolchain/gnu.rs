use crate::toolchain::ToolChainTrait;

pub struct GNU{
    #[allow(dead_code)] data: ()
}

impl ToolChainTrait for GNU {
    const CC:     &'static str = "gcc";
    const CXX:    &'static str = "g++";
    const DEBUG_FLAG:  &'static str = "-g";

    const EXECUTABLE_LINKER: &'static str = "g++";
    const STATIC_LIB_LINKER: &'static str = "ar";
    const SHARED_LIB_LINKER: &'static str = "g++";
    const OBJECT_LIB_LINKER: &'static str = "lr";

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