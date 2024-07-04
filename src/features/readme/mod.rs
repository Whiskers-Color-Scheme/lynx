use std::{env, fs};

use colored::Colorize;

use inquire::{MultiSelect, Text};

pub fn create_interactive_readme() {
    let port_name = get_answer("What's the port name?");
    let port_url = get_answer("What's the port website?");
    let port_user = get_answer("What's your github username?");

    let readme_options = vec![
        "Thanks To",
        "Steps Install",
        "List Install",
        "Code List Install",
        "Usage",
        "FAQ",
    ];

    let readme_sections =
        MultiSelect::new("Select the sections you would like to add", readme_options)
            .prompt()
            .expect("Error getting sections");

    let mut readme_content = String::new();

    let header = include_str!("./templates/header.md")
        .replace("{%port_name%}", &port_name)
        .replace("{%port_url%}", &port_url);

    readme_content += &header;

    for option in readme_sections.to_owned() {
        if option == "Thanks To" {
            let content = include_str!("./templates/thanks_to.md");
            readme_content += &content;
        }

        if option == "Steps Install" {
            let content = include_str!("./templates/install.md");
            readme_content += &content;
        }

        if option == "List Install" {
            let content = include_str!("./templates/list_install.md");
            readme_content += &content;
        }

        if option == "Code List Install" {
            let content = include_str!("./templates/code_list_install.md");
            readme_content += &content;
        }

        if option == "Usage" {
            let content = include_str!("./templates/usage.md");
            readme_content += &content;
        }

        if option == "FAQ" {
            let content = include_str!("./templates/faq.md");
            readme_content += &content;
        }
    }

    let maintainers = include_str!("./templates/maintainers.md").replace("{%user%}", &port_user);

    readme_content += &maintainers;

    let mut readme_path = env::current_dir().unwrap().to_owned();
    readme_path.push("README.md");

    fs::write(&readme_path, readme_content.as_bytes()).expect("Error writing README.md");

    println!("{}", "Readme created successfully".green());
}

pub fn create_default_readme() {
    let readme_content = include_str!("./templates/default_readme.md");
    let mut readme_path = env::current_dir().unwrap().to_owned();
    readme_path.push("README.md");

    fs::write(&readme_path, readme_content.as_bytes()).expect("Error writing README.md");

    println!("{}", "Readme created successfully".green());
}

fn get_answer(question: impl Into<String>) -> String {
    let question = question.into();
    let mut valid_answer = false;
    let mut answer = String::new();

    while !valid_answer {
        let res = Text::new(&question).prompt();

        match res {
            Ok(response) => {
                answer = response;

                if !answer.trim().is_empty() {
                    valid_answer = true;
                } else {
                    println!("Can't have an empty answer");
                }
            }
            Err(_) => println!("Invalid answer. Please try again."),
        }
    }

    return answer;
}
