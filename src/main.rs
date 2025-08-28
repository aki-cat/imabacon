use std::path;

fn main() -> anyhow::Result<()> {
    for i in 0..4 {
        image_batch_converter::convert(&path::Path::new(&format!("in/ralseiblunt{i}.png")))?;
    }
    Ok(())
}
