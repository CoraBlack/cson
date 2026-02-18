use std::time::SystemTime;

use crate::utils;

pub struct Object {
    pub path: std::path::PathBuf,
    pub modified: Option<SystemTime>,
}

pub struct ObjectCollection {
    pub objects: Vec<Object>,
}

impl ObjectCollection {
    pub fn to_arg_str(&self) -> String {
        let mut arg_str = String::new();
        for obj in &self.objects {
            arg_str.push_str(utils::normalize_path(obj.path.clone()).to_str().unwrap());
            arg_str.push(' ');
        }

        arg_str
    }
}

pub struct Execuable {

}

pub struct StaticLib {

}

pub struct SharedLib {
    
}