use clap::{Parser, Subcommand};

use crate::{clipboard, layout};

#[derive(Parser, Debug)]
#[command(name = "layshift")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Map { source: String, target: String },
}

impl Cli {
    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        let cli = Cli::parse();

        match cli.command {
            Commands::Map { source, target } => Cli::map(source, target),
        }
    }

    fn map(source: String, target: String) -> Result<(), Box<dyn std::error::Error>> {
        let source_layout = layout::Layout::new(&source)?;
        let target_layout = layout::Layout::new(&target)?;

        let text = clipboard::read()?;
        let result = layout::map_string(&text, &source_layout, &target_layout);

        clipboard::write(&result)?;
        Ok(())
    }
}
