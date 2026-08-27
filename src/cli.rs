use std::fs;

use clap::{Parser, Subcommand};

use crate::{clipboard, config, layout};

#[derive(Parser, Debug)]
#[command(name = "layshift")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Map {
        source: Option<String>,
        target: Option<String>,
    },
    SetDefault {
        source: String,
        target: String,
    },
}

impl Cli {
    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        let cli = Cli::parse();

        match cli.command {
            Commands::Map { source, target } => Cli::map(source, target),
            Commands::SetDefault { source, target } => Cli::set_default(source, target),
        }
    }

    fn map(
        source: Option<String>,
        target: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (source, target) = match (source, target) {
            (None, None) => config::get_default_layouts()?,
            (Some(source), Some(target)) => (source, target),
            _ => return Err("Both source and target layouts are required.".into()),
        };

        let source_layout = layout::Layout::new(&source)?;
        let target_layout = layout::Layout::new(&target)?;

        let text = clipboard::read()?;
        let result = layout::map_string(&text, &source_layout, &target_layout);

        clipboard::write(&result)?;
        Ok(())
    }

    fn set_default(source: String, target: String) -> Result<(), Box<dyn std::error::Error>> {
        let result = format!("source = \"{}\"\narget = \"{}\"\n", source, target);
        fs::write(config::get_config_file(), result)?;
        Ok(())
    }
}
