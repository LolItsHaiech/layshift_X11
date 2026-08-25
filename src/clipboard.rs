use std::process::Command;

pub fn read() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("wl-paste").output()?;

    if !output.status.success() {
        return Err("Failed to read from Wayland clipboard.".into());
    }

    Ok(String::from_utf8(output.stdout)?)
}

pub fn write(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("wl-copy").arg(text).status()?;

    if !status.success() {
        return Err("Failed to write to wayland clipboard.".into());
    }

    Ok(())
}
