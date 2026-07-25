use serde::{Serialize,Deserialize};
use std::{fs, path::{PathBuf,}};

#[derive(Serialize, Deserialize)]
pub struct Engine {
    pub name: String,
    pub executable: PathBuf,
    icon: Option<PathBuf>,
}
#[derive(Serialize, Deserialize)]
pub struct Engines<> {
    engine: Vec<Engine>,

}

impl Engine {
    pub fn create(name : &str)-> Engine{
        Engine{
            name : name.to_string(),
            executable: PathBuf::new(),
            icon: None,
        }
    }
}
pub fn save_config(config : &mut Vec<Engine>) -> Result<(),Box<dyn std::error::Error>>{
    let mut  engines = Engines{engine: vec![] };
    engines.engine.append(config);
    let toml  = toml::to_string(&engines);
    match fs::write("engines.toml", toml?)? {
        _ => {} };
    Ok(())
}
pub fn create_config() -> Result<(), Box<dyn std::error::Error>> {
    let config = Engines{
        engine : vec![
            Engine::create("GZDoom")
        ]
    };
    let toml_string = toml::to_string_pretty(&config)?;
    match fs::exists("engines.toml") {
        Ok(bool) => {
            print!("{bool}");
            if bool == false {
                fs::write("engines.toml", toml_string)?
            }
        },
        Err(e) => println!("Erreur at {e}")
    }
    Ok(())
}


pub fn load_engines() -> Vec<Engine> {
    let data = std::fs::read_to_string("engines.toml").unwrap();
    let engines:Engines = toml::from_str(&data).unwrap();
    engines.engine
}