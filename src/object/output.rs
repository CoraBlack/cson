use std::time::SystemTime;

#[derive(Clone)]
pub struct Object {
    pub path: std::path::PathBuf,
    pub modified: Option<SystemTime>,
}

#[derive(Clone)]
pub struct ObjectCollection {
    pub objects: Vec<Object>,
}

impl ObjectCollection {
    pub fn to_args(&self) -> Vec<String> {
        let mut arg_str = Vec::new();
        for obj in &self.objects {
            arg_str.push(obj.path.clone().to_str().unwrap().to_string());
        }

        arg_str
    }
}

// pub struct Execuable {

// }

// pub struct StaticLib {

// }

// pub struct SharedLib {
    
// }