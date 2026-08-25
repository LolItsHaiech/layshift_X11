use std::env;

mod clipboard;
mod layout;

#[cfg(test)]
mod test;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let text = clipboard::read()?;

    let source_layout = layout::Layout::new(&args[1])?;
    let target_layout = layout::Layout::new(&args[2])?;

    let result = layout::map_string(&text, &source_layout, &target_layout);
    clipboard::write(&result)?;
    Ok(())
}
