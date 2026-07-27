// Copyright (c) 2026 Antoine Veillé
// SPDX-License-Identifier: CC-BY-SA-4.0



use std::fs;
use std::path::{PathBuf};
use serde::{Deserialize, Serialize};


#[derive(Serialize,Deserialize)]
pub struct Iwad{
    pub name: String,
    pub path: PathBuf,
}
#[derive(Serialize,Deserialize)]
pub struct Iwads {
    pub iwad : Vec<Iwad>,
}
impl Iwad {
    pub fn from_path(path : PathBuf) -> Iwad {
        let name = path.file_stem().unwrap().to_string_lossy().to_string();
        Iwad{
            name,path
        }
    }
    pub fn new() -> Iwad {
        Iwad{name:String::from(""), path:PathBuf::new()}
    }
}
pub fn create_config_iwads() -> Result<(), Box<dyn std::error::Error>> {
    let config = Iwads{
        iwad : vec![
            Iwad::new()
        ]
    };
    let toml_string = toml::to_string_pretty(&config)?;
    match fs::exists("iwads.toml") {
        Ok(bool) => {
            if bool == false {
                fs::write("iwads.toml", toml_string)?
            }
        },
        Err(e) => println!("Erreur at {e}")
    }
    Ok(())
}
pub fn load_iwads() -> Iwads {
    let data = fs::read_to_string("iwads.toml").unwrap();
    let iwads:Iwads = toml::from_str(&data).unwrap();
    iwads
}
pub fn save_config_iwads(config: &Iwads) -> Result<(), Box<dyn std::error::Error>> {

    let toml = toml::to_string(config)?;


    fs::write("iwads.toml", toml)?;
    Ok(())
}