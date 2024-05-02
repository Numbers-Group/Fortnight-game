use serde_derive::Deserialize;
use serde_derive::Serialize;
use crate::Condition;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Project {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "author")]
    pub author: String,
    #[serde(rename = "git_link")]
    pub git_link: String,
    #[serde(rename = "build_version")]
    pub build_version: String,
    #[serde(rename = "project_path")]
    pub path: String,
}


pub fn create_project() -> Result<(), Box<dyn std::error::Error>> {
    let project = Project {
        name: String::from("game3"),
        author: String::from("0xBLCKLPTN"),
        git_link: String::from("https://github.com/geita.git"),
        build_version: String::from("0.1"),
        path: String::from("TEST/")
    };
    let j = serde_json::to_string(&project)?;

    println!("{}", j);

    let mut file = std::fs::File::create("/Users/twofacedjanus/Documents/geita/geita_projects/game3.json")?;
    serde_json::to_writer_pretty(&mut file, &project)?;

    Ok(())
}

pub fn delete_project(project_name: String) {
    todo!();
}
