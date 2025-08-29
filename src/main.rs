use std::env;
use std::path;

fn main() -> anyhow::Result<()> {
    let app_args = imabacon::args::parse(env::args())?;
    println!("\nArguments provided: {:?}\n", app_args);
    for file in imabacon::list_files(&app_args.input)? {
        match imabacon::convert(path::Path::new(&file), path::Path::new(&app_args.output)) {
            Ok(new_file) => println!("> Converted: {file} => {new_file}"),
            Err(msg) => println!("Skipped: {msg}."),
        }
    }
    Ok(())
}
