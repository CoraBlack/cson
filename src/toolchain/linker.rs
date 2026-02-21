use crate::object::output::{Object, ObjectCollection, SharedLib, StaticLib};

pub trait Linker {
    fn get_linker() -> Option<String>;

    fn link_to_object(input: ObjectCollection) -> Option<Object>;
    fn link_to_execuable(input: ObjectCollection) -> ();
    fn link_to_static_lib(input: ObjectCollection) -> Option<StaticLib>;
    fn link_to_dynamic_lib(input: ObjectCollection) -> Option<SharedLib>;
}

pub fn link_to_execuable<T: Linker>(input: ObjectCollection) -> () {
    T::link_to_execuable(input);
}