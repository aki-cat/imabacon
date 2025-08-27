use std::path;

const EXAMPLE_FILE: &str = "ralseiblunt.png";

fn main() -> anyhow::Result<()> {
    image_batch_converter::convert(&path::Path::new(EXAMPLE_FILE))
}
