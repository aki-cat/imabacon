mod args;

use std::path;

fn main() -> anyhow::Result<()> {
    let app_args = args::parse()?;
    println!("{:?}", app_args);
    for file in imabacon::list_files(&app_args.input)? {
        imabacon::convert(path::Path::new(&file), path::Path::new(&app_args.output))?;
    }
    Ok(())
}
