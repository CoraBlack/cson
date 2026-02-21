use crate::toolchain::gnu::GNU;

pub mod compiler;
pub mod gnu;
pub mod linker;

pub enum ToolChain {
    GNU(GNU),
    LLVM(),
    MSVC(),
}