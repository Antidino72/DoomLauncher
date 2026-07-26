use serde::{Serialize,Deserialize};
use std::{fs, path::{PathBuf,}};

#[derive(Serialize, Deserialize, Clone)]
pub struct Engine {
    pub name: String,
    pub executable: PathBuf,
    pub icon: Option<PathBuf>,
    pub default_args: String,
}
#[derive(Serialize, Deserialize)]
pub struct Engines<> {
    engine: Vec<Engine>,

}

impl Engine {
    pub fn new(
        name: impl Into<String>,
        executable: Option<PathBuf>,
        icon: Option<PathBuf>,
        default_args: Option<String>,
    ) -> Self {
        Self {
            name: name.into(),
            executable: executable.unwrap_or_default(),
            icon,
            default_args: default_args.unwrap_or_default(),
        }
    }
}
pub fn save_config(config: &Vec<Engine>) -> Result<(), Box<dyn std::error::Error>> {
    let toml = toml::to_string(&Engines {
        engine: config.to_vec(),
    })?;

    fs::write("engines.toml", toml)?;
    Ok(())
}
pub fn create_config() -> Result<(), Box<dyn std::error::Error>> {
    let config = Engines{
        engine : vec![
            Engine::new("GZDoom", None, None, None),
        ]
    };
    let toml_string = toml::to_string_pretty(&config)?;
    match fs::exists("engines.toml") {
        Ok(bool) => {
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