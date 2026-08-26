mod cli;
mod clipboard;
mod layout;

#[cfg(test)]
mod test;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    cli::Cli::run()
}
