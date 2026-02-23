use std::path::PathBuf;

use crate::{cxon::get_cxon_config, object::output::ObjectCollection, toolchain::ToolChainTrait};

// pub trait Linker {
//     fn get_linker() -> String;

//     // fn link_to_object(input: ObjectCollection) -> Option<Object>;
//     fn link_to_execuable(input: ObjectCollection) -> ();
//     // fn link_to_static_lib(input: ObjectCollection) -> Option<StaticLib>;
//     // fn link_to_dynamic_lib(input: ObjectCollection) -> Option<SharedLib>;
// }

pub fn link_to_executable<T: ToolChainTrait>(input: ObjectCollection) -> () {
    let linker = T::LINKER;
    let output_dir = &get_cxon_config().read().unwrap().output_dir;

    let target_name = &get_cxon_config().read().unwrap().get_target_name();
    let output_path = output_dir.join(PathBuf::from(target_name));

    std::process::Command::new(linker)
        .arg(T::DEBUG_FLAG)
        .args(input.to_args())
        .arg(T::OUTPUT_FLAG)
        .arg(output_path.to_str().unwrap())
        .args(get_cxon_config().read().unwrap().get_link_dir_args::<T>())
        .args(get_cxon_config().read().unwrap().get_lib_args::<T>())
        .status()
        .expect(format!("Failed to link executable {}", output_path.to_str().unwrap()).as_str());
}