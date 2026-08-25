use std::process::Command;

pub fn read() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("wl-paste").output()?;
    Ok(String::from_utf8(output.stdout)?)
}

pub fn write(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    Command::new("wl-copy").arg(text).status()?;
    Ok(())
}
