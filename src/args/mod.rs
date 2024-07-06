use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[clap(
        short,
        long,
        default_value_t = false,
        help = "Print the whole color palette"
    )]
    pub palette: bool,

    #[arg(
        short = 'r',
        long = "readme",
        value_enum,
        help = "Create a readme file"
    )]
    pub readme: Option<ReadmeOpions>,

    #[command(subcommand)]
    pub generator: Option<GeneratorOptions>,
}

#[derive(ValueEnum, PartialEq, Clone)]
pub enum ReadmeOpions {
    Interactive,
    Default,
}

#[derive(Subcommand)]
pub enum GeneratorOptions {
    ToTemplate {
        #[arg(short, long, help = "The file to generate a template from")]
        file: PathBuf
    },
    FromTemplate {
        #[arg(short, long, help = "The template file")]
        file: PathBuf
    }
}
