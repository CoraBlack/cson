use crate::object::{
    source::Source,
    output::Object
};

#[derive(Clone)]
pub struct CompilerPair {
    pub cc:  String,
    pub cxx: String,
}

pub trait Compiler {
    const DEFINE_FLAG_PREFIX:   &'static str;
    const INCLUDE_FLAG_PREFIX:  &'static str;
    const LINK_DIR_FLAG_PREFIX: &'static str;
    const LINK_LIB_FLAG_PREFIX: &'static str;
    fn get_compiler() -> CompilerPair;

    fn compile(src: Source) -> Object;
}

pub fn compile<T: Compiler>(src: Source) -> Object {
    T::compile(src)
}