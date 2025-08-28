use std::path;

fn main() -> anyhow::Result<()> {
    let files = imabacon::list_files()?;
    for file in files {
        imabacon::convert(&path::Path::new(&file))?;
    }
    Ok(())
}
