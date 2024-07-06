use std::{fs, process::exit};

use clap::Parser;

use crate::{
    args::Args,
    features::{
        palette::show_palette,
        readme::{create_default_readme, create_interactive_readme},
    },
};

pub fn handle_args() {
    let args = Args::parse();

    if args.palette {
        show_palette();
        exit(0);
    }

    if let Some(readme) = args.readme {
        match readme {
            crate::args::ReadmeOpions::Interactive => {
                create_interactive_readme();
            }
            crate::args::ReadmeOpions::Default => {
                create_default_readme();
            }
        }
    }

    if let Some(generator) = args.generator {
        match generator {
            crate::args::GeneratorOptions::ToTemplate { file } => {}
            crate::args::GeneratorOptions::FromTemplate { file } => {}
        }
    }
}
