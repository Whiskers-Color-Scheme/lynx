use inquire::Select;

use crate::features::palette::show_palette;

pub fn show_main_menu() {

    let art = r#"
██      ██    ██ ███    ██ ██   ██ 
██       ██  ██  ████   ██  ██ ██  
██        ████   ██ ██  ██   ███   
██         ██    ██  ██ ██  ██ ██  
███████    ██    ██   ████ ██   ██
"#;

    println!("{}", art);

    let palette_opt = "Get color palette";
    let readme_opt = "Create a README.md";
    let options = vec![palette_opt, readme_opt];

    println!("\n\n\n\n");
    let ans = Select::new("Select an option", options).prompt();

    if let Ok(choice) = ans {
        if choice == palette_opt {
            show_palette();
        }

        if choice == readme_opt {
            show_readme_menu();
        }
    }
}

pub fn show_readme_menu() {
    let interactive_opt = "Create a README.md (interactive mode)";
    let default_opt = "Create a default README.md";
    let options = vec![interactive_opt, default_opt];

    let ans = Select::new("How would you like to create your README.md?", options).prompt();

    if let Ok(choice) = ans {
        if choice == interactive_opt {};

        if choice == default_opt {}
    }
}
