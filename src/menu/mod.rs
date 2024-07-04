use std::process::exit;

use inquire::Select;

use crate::features::{palette::show_palette, readme::{create_default_readme, create_interactive_readme}};

pub fn show_main_menu() {
    let mut exit_program = false;

    while !exit_program {
        let options = vec!["Color palette", "Create README.md", "Exit"];

        println!("\n\n");
        let ans = Select::new("Select an option:", options).prompt();

        if let Ok(choice) = ans {
            if choice == "Color palette" {
                show_palette();
            }

            if choice == "Create README.md" {
                show_readme_menu();
            }

            if choice == "Exit" {
                exit_program = true;
            }
        }
    }

    exit(0);
}

pub fn show_readme_menu() {
    let interactive_opt = "Interactive";
    let default_opt = "Default";
    let options = vec![interactive_opt, default_opt];

    let ans = Select::new("How would you like to create your README.md?", options).prompt();

    if let Ok(choice) = ans {
        if choice == interactive_opt {
            create_interactive_readme();
        };

        if choice == default_opt {
            create_default_readme();
        }
    }
}
