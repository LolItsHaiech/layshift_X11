mod cli;
mod clipboard;
mod config;
mod layout;
mod metadata;

#[cfg(test)]
mod test;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    cli::Cli::run()
}
