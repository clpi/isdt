use std::path::PathBuf;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Demo {
    name: String,
    path: PathBuf,
    sections: Vec<Section>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Step {
    url: String,
    talking_point: String,
    click_instruction: String,
    coords: (i32, i32),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Section {
    title: String,
    steps: Vec<Step>
}

impl Demo {

    pub fn new<P: Into<PathBuf>>(name: String, path: P) -> Demo {
        Demo {
            name,
            sections: Vec::new(),
            path: path.into()
        }
    }

    pub fn add_section(&mut self, name: String) {

    }
}

impl Step {

}