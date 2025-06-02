use std::{
    fs,
    io::{Result, Write},
    path::Path,
};

pub fn write_file_with_dirs(path: &str, contents: &str) -> Result<()> {
    let path = Path::new(path);

    // Create parent directories if they don't exist
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Write file
    let mut file = fs::File::create(path)?;
    file.write_all(contents.as_bytes())?;

    Ok(())
}
