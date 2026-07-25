
use slint::{ModelRc, VecModel, SharedString};

use crate::config::{create_config, load_engines, Engine, save_config};

slint::include_modules!();

mod config;
use rfd::FileDialog;
fn update_list_engine(window: &MainWindow,engines: &Vec<Engine>) {
    window.set_engines(
        ModelRc::new(
            VecModel::from(get_name_of_engines(engines))
        )
    );
}
fn get_name_of_engines(engines : &Vec<Engine>)-> Vec<SharedString>{
    let names = engines
        .iter()
        .map(|engine| engine.name.clone().into())
        .collect();
    names

}
fn main() -> Result<(), slint::PlatformError> {
    match create_config() {
        Ok(_) => println!("Fichier crée"),
        Err(e) => println!("Erreur lors de la création du fichier : {e}") 
    }

    let window = MainWindow::new()?;
    let mut engines = load_engines();

    update_list_engine(&window, &engines);
    let window_handle = window.clone_strong();
    window.on_add_engine(move ||{
        let window = &window_handle;
        if let Some(path) = FileDialog::new()
            .pick_file()
        {
            let name = path
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .to_string();
            println!("{}", name);
            engines.push(
                Engine::create(&name)
            );
            update_list_engine(&window, &engines);
            match save_config(&mut engines) {
                Ok(_) => {}
                Err(e) => {println!("Error as {e}")}
            }
        }
    });
    window.on_launch(|| {
        println!("Lancement de Doom !");
    });
    
    window.run()
}