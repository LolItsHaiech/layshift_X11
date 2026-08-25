use std::env;

mod clipboard;
mod layout;

#[cfg(test)]
mod test;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();

    args.next();
    let source = args.next().ok_or("Missing source layout.")?;
    let target = args.next().ok_or("Missing target layout.")?;

    let source_layout = layout::Layout::new(&source)?;
    let target_layout = layout::Layout::new(&target)?;

    let text = clipboard::read()?;
    let result = layout::map_string(&text, &source_layout, &target_layout);
    clipboard::write(&result)?;
    Ok(())
}
