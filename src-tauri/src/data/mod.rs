use std::path::PathBuf;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Debug, Hash, PartialEq)]
pub struct Point { x: i32, y: i32 }


#[derive(Serialize, Deserialize, Default, Debug, Hash, PartialEq)]
pub struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

#[derive(Serialize, Deserialize, Default, Debug, Hash, PartialEq)]
pub struct Demo {
    name: String,
    path: PathBuf,
    sections: Vec<Section>,
}

#[derive(Serialize, Deserialize, Default, Debug, Hash, PartialEq)]
pub struct Step {
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    talking_point: String,
    click_instruction: String,
    hover: (Point, Point),
    coords: Point,
    fade: bool,
}

#[derive(Serialize, Deserialize, Default, Debug, Hash, PartialEq)]
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

    pub fn from_metadata() -> Result<(), serde_yaml::Error> {
        let st = serde_yaml::to_string(&Self::default())?;
        let des: Self = serde_yaml::from_str(&st)?;
        Ok(())
    }

    pub fn save_metadata(&self) -> Result<(), serde_yaml::Error> {
        let st = serde_yaml::to_string(&self)?;
        Ok(())
    }

    pub fn from_xml() -> Self { Self::default() }
    pub fn to_xml(&self) -> String { "".into() }

    pub fn add_section(&mut self, name: String) {

    }

    pub fn export_yaml(&mut self) {
        
    }
}

impl Step {

    pub fn new() -> Self {
        Step::default()
    }

}

impl Section {

    pub fn new() -> Self {
        Section::default()
    }
}