// Copyright (c) 2026 Antoine Veillé
// SPDX-License-Identifier: CC-BY-SA-4.0
use std::process::Command;
use slint::{ModelRc, VecModel, SharedString};
use std::rc::Rc;
use std::cell::RefCell;
use crate::engine_manager::{create_config_engines, load_engines, Engine, save_config};

slint::include_modules!();

mod engine_manager;
mod iwads_manager;

use rfd::{FileDialog, MessageButtons, MessageDialog, MessageDialogResult, MessageLevel};
use crate::iwads_manager::{create_config_iwads, load_iwads, Iwad, save_config_iwads, Iwads};

fn update_list_engine(window: &MainWindow, engines: &[Engine]) {
    window.set_engines(
        ModelRc::new(
            VecModel::from(get_name_of_engines(engines))
        )
    );
}
fn update_list_iwads(window: &MainWindow, iwads: &Iwads) {

    let items: Vec<SharedString> = iwads
        .iwad
        .iter()
        .map(|item| SharedString::from(&item.name))
        .collect();

    let model = VecModel::from(items);
    window.set_iwads(ModelRc::new(model));
}
fn get_name_of_engines(engines: &[Engine]) -> Vec<SharedString> {
    engines
        .iter()
        .map(|engine| engine.name.clone().into())
        .collect()
}

fn main() -> Result<(), slint::PlatformError> {
    if let Err(e) = create_config_engines() {
        println!("Erreur lors de la création du fichier config engines: {e}");
    }
    if let Err (e) = create_config_iwads(){
        println!("Erreur lors de la création du fichier config iwads: {e}");
    }

    let window = MainWindow::new()?;


    //==================================================
    //            ENGINE SYSTEM
    //==================================================
    // On enveloppe les moteurs dans Rc et RefCell pour pouvoir les partager
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
            .set_description("Are you sure you want to remove this engine?")
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

            // On met à jour l'interface visuelle après la suppression
            update_list_engine(window, &engines_remove.borrow());

            //   On sauvegarde la configuration après la suppression
            if let Err(e) = save_config(&engines_remove.borrow()) {
                println!("Erreur lors de la sauvegarde : {e}");
            }
        }
    });
    let window_handle_option = window.clone_strong();
    let engines_ref = engines.clone();
    window.on_option(move || {
        let window = window_handle_option.clone_strong();
        let index = window.get_selected_engine();
        if index < 0 {
            return; // Aucun élément sélectionné
        }

        let engines_ref = engines_ref.borrow();
        if let Some(current_engine) = engines_ref.get(index as usize) {
            //Nom de l'engine
            window.set_name_engine(SharedString::from(&current_engine.name));

            // Exécutable
            let exec_str = current_engine.executable.to_string_lossy();
            window.set_executable(SharedString::from(exec_str.as_ref()));

            // 3. Icône (Si optionnel)
            let icon_path = &current_engine.icon;
            let path_str = icon_path.to_string_lossy();

            if !path_str.is_empty() {
                window.set_icon_path(SharedString::from(path_str.as_ref()));

                if let Ok(img) = slint::Image::load_from_path(icon_path) {
                    window.set_icon_engine(img);
                }
            } else {
                window.set_icon_path(SharedString::from(""));
                window.set_icon_engine(slint::Image::default());
            }
        }
        window.set_option_open(true);
    });
    let window_handle_option = window.clone_strong();
    window.on_cancel_option(move || {
        let window = &window_handle_option;
        window.set_option_open(false);
    });
    //==================================================
    //            IWADS SYSTEM
    //==================================================
    let iwads = Rc::new(RefCell::new(load_iwads()));
    update_list_iwads(&window, &iwads.borrow());

    let iwads_add = iwads.clone();
    let window_handle_iwad = window.clone_strong();
    window.on_add_iwad(move || {
        let window =  &window_handle_iwad;
        if let Some(path) = FileDialog::new().pick_file() {
            iwads_add.borrow_mut().iwad.push(Iwad::from_path(path));
            update_list_iwads(window, &iwads_add.borrow());
            if let Err(e) = save_config_iwads(&iwads_add.borrow()) {
                eprintln!("Erreur lors de la sauvegarde : {}", e);
            }
        }
    });
    let window_handle_iwad = window.clone_strong();
    let iwads_add = iwads.clone();

    window.on_remove_iwad(move || {
        let window = window_handle_iwad.clone_strong();


        let selected_index = window.get_selected_iwad();
        if selected_index < 0 {
            return; // Aucun IWAD sélectionné dans l'interface
        }
        let index = selected_index as usize;

        // 2. Demande de confirmation
        let result_iwad = MessageDialog::new()
            .set_level(MessageLevel::Warning)
            .set_title("Confirmation")
            .set_description("Êtes-vous sûr de vouloir supprimer cet IWAD ?")
            .set_buttons(MessageButtons::YesNo)
            .show();

        if result_iwad == MessageDialogResult::Yes {
            let mut iwads_borrow = iwads_add.borrow_mut();

            // 3. Suppression sécurisée
            if index < iwads_borrow.iwad.len() {
                iwads_borrow.iwad.remove(index);
                println!("IWAD à l'index {} supprimé", index);

                // Si la liste devient vide, on peut rajouter un élément par défaut si souhaité
                if iwads_borrow.iwad.is_empty() {
                    iwads_borrow.iwad.push(Iwad::new());
                }

                // 4. Mettre à jour l'interface visuelle
                update_list_iwads(&window, &iwads_borrow);

                // 5. Sauvegarder la configuration sur disque
                if let Err(e) = save_config_iwads(&iwads_borrow) {
                    eprintln!("Erreur lors de la sauvegarde : {e}");
                }
            }
        }
    });
    let engines_launch = engines.clone();
    let iwads_launch = iwads.clone();
    let window_handle_launch = window.clone_strong();

    window.on_launch(move || {
        let window = &window_handle_launch;

        let engine_index = window.get_selected_engine();
        let iwad_index = window.get_selected_iwad();

        // Vérification engine
        let engines_ref = engines_launch.borrow();
        let Some(engine) = engines_ref.get(engine_index as usize) else {
            MessageDialog::new()
                .set_level(MessageLevel::Error)
                .set_title("Error")
                .set_description("No engine selected.")
                .show();
            return;
        };

        // Vérification que l'executable existe
        if engine.executable.as_os_str().is_empty() {
            MessageDialog::new()
                .set_level(MessageLevel::Error)
                .set_title("Error")
                .set_description("The engine does not have an executable configured.")
                .show();
            return;
        }

        // Vérification IWAD
        let iwads_ref = iwads_launch.borrow();
        let Some(iwad) = iwads_ref.iwad.get(iwad_index as usize) else {
            MessageDialog::new()
                .set_level(MessageLevel::Error)
                .set_title("Error")
                .set_description("No IWAD selected.")
                .show();
            return;
        };

        if iwad.path.as_os_str().is_empty() {
            MessageDialog::new()
                .set_level(MessageLevel::Error)
                .set_title("E")
                .set_description("The IWAD does not have a configured path.")
                .show();
            return;
        }

        // Lancement
        match Command::new(&engine.executable)
            .arg("-iwad")
            .arg(&iwad.path)
            .spawn()
        {
            Ok(_) => println!("Doom Started !"),
            Err(e) => {
                MessageDialog::new()
                    .set_level(MessageLevel::Error)
                    .set_title("Erreur de lancement")
                    .set_description(&format!("Unable to start engine:\n{e}"))
                    .show();
            }
        }
    });

    window.run()
}