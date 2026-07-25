use slint::{ModelRc,VecModel,SharedString};

use crate::config::{create_config, load_engines};

slint::include_modules!();

mod config;
use rfd::FileDialog;


fn main() -> Result<(), slint::PlatformError> {
    match create_config() {
        Ok(_) => println!("Fichier crée"),
        Err(e) => println!("Erreur lors de la création du fichier : {e}") 
    }

    let window = MainWindow::new()?;

    let names: Vec<SharedString> = load_engines()
        .iter()
        .map(|engine| engine.name.clone().into())
        .collect();
    println!("{:?}", names);
    window.set_engines(
        ModelRc::new(
            VecModel::from(names)
        )
    );
    window.on_add_engine(|| {
        if let Some(path) = FileDialog::new()
            .pick_file()
        {
            let name = path
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .to_string();
            println!("{}",name);
        }
    });
    window.on_launch(|| {
        println!("Lancement de Doom !");
    });
    
    window.run()
}