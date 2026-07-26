use slint::{ModelRc, VecModel, SharedString};
use std::rc::Rc;
use std::cell::RefCell;

use crate::config::{create_config, load_engines, Engine, save_config};

slint::include_modules!();

mod config;
use rfd::{FileDialog, MessageButtons, MessageDialog, MessageDialogResult, MessageLevel};

// J'ai modifié la signature pour accepter &[Engine] ce qui est plus idiomatique en Rust
fn update_list_engine(window: &MainWindow, engines: &[Engine]) {
    window.set_engines(
        ModelRc::new(
            VecModel::from(get_name_of_engines(engines))
        )
    );
}

fn get_name_of_engines(engines: &[Engine]) -> Vec<SharedString> {
    engines
        .iter()
        .map(|engine| engine.name.clone().into())
        .collect()
}

fn main() -> Result<(), slint::PlatformError> {
    if let Err(e) = create_config() {
        println!("Erreur lors de la création du fichier config : {e}");
    }

    let window = MainWindow::new()?;

    // 1. On enveloppe les moteurs dans Rc et RefCell pour pouvoir les partager
    let engines = Rc::new(RefCell::new(load_engines()));

    update_list_engine(&window, &engines.borrow());

    // --- Configuration du bouton AJOUTER ---
    let window_handle_add = window.clone_strong();
    let engines_add = engines.clone(); // On clone la référence Rc

    window.on_add_engine(move || {
        let window = &window_handle_add;
        if let Some(path) = FileDialog::new().pick_file() {
            let name = path.file_stem().unwrap().to_string_lossy().to_string();
            println!("{}", name);
            // On emprunte pour modifier (borrow_mut)
            let engine =  Engine::new(&name, Option::from(path), None, None);
            engines_add.borrow_mut().push(engine);
            update_list_engine(window, &engines_add.borrow());
            println!("{}",engines_add.borrow().len());
            if let Err(e) = save_config(&engines_add.borrow()) {
                println!("Erreur lors de la sauvegarde : {e}");
            }
        }
    });

    // --- Configuration du bouton SUPPRIMER ---
    let window_handle_remove = window.clone_strong();
    let engines_remove = engines.clone(); // On clone l'autre référence Rc

    window.on_remove_engine(move || {
        let window = &window_handle_remove;
        let result = MessageDialog::new()
            .set_level(MessageLevel::Warning)
            .set_title("Confirmation")
            .set_description("Es-tu sûr de vouloir supprimer ce moteur ?")
            .set_buttons(MessageButtons::YesNo)
            .show();

        if result == MessageDialogResult::Yes {
            let index: usize = window.get_selected_engine().try_into().unwrap();
            println!("Moteur à l'index {} supprimé", index);
            // On retire le moteur
            if engines_remove.borrow().is_empty(){
                engines_remove.borrow_mut().push(Engine::new("Empty", None, None, None))
            }
            engines_remove.borrow_mut().remove(index);

           //on verifie que engines n'est pas vide pour eviter less erreur
            if engines_remove.borrow().is_empty() {
                engines_remove.borrow_mut().push(Engine::new("Empty", None, None, None))
            }

            // NOUVEAU : On met à jour l'interface visuelle après la suppression
            update_list_engine(window, &engines_remove.borrow());

            // NOUVEAU : On sauvegarde la configuration après la suppression
            if let Err(e) = save_config(&engines_remove.borrow()) {
                println!("Erreur lors de la sauvegarde : {e}");
            }
        }
    });
    let window_handle_option = window.clone_strong();
    window.on_option(move || {
        let window = &window_handle_option;
        window.set_option_open(true);
    });
    let window_handle_option = window.clone_strong();
    window.on_cancel_option(move || {
        let window = &window_handle_option;
        window.set_option_open(false);
    });
    window.on_launch(|| {

        println!("Lancement de Doom !");
    });

    window.run()
}