use std::env;
use std::path;

fn main() -> anyhow::Result<()> {
    let (input_dir, output_dir) = args()?;
    for file in imabacon::list_files(&input_dir)? {
        imabacon::convert(path::Path::new(&file), path::Path::new(&output_dir))?;
    }
    Ok(())
}

fn args() -> anyhow::Result<(String, String)> {
    let mut args = env::args().enumerate();

    args.next().unwrap(); // consume first arg

    let Some((_, input_dir)) = args.next() else {
        return Err(anyhow::Error::msg("Invalid input argument"));
    };
    let Some((_, out_dir)) = args.next() else {
        return Err(anyhow::Error::msg("Invalid output argument"));
    };

    Ok((input_dir, out_dir))
}
