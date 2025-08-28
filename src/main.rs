use std::path;

fn main() -> anyhow::Result<()> {
    let files = imabacon::list_files();
    for i in 0..4 {
        imabacon::convert(&path::Path::new(&format!("in/ralseiblunt{i}.png")))?;
    }
    Ok(())
}
