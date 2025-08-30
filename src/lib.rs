pub mod args;

use std::fs;
use std::io;
use std::path;

use image::ExtendedColorType;

pub fn run(app_args: args::AppArgs) -> anyhow::Result<()> {
    println!("\nArguments provided: {:?}\n", app_args);
    list_files(&app_args.input)?.for_each(|file| {
        match convert(path::Path::new(&file), path::Path::new(&app_args.output)) {
            Ok(new_file) => println!("> Converted: {file} => {new_file}"),
            Err(msg) => println!("Skipped: {msg}."),
        }
    });
    Ok(())
}

pub fn list_files(input_dir: &str) -> anyhow::Result<impl Iterator<Item = String>> {
    let out = fs::read_dir(input_dir)?.filter_map(|dir_entry| {
        let entry = dir_entry.unwrap();
        let entry_type = entry.file_type().unwrap();

        match entry_type.is_file() {
            true => Some(String::from(entry.path().as_path().to_str().unwrap())),
            false => None,
        }
    });
    Ok(out)
}

pub fn convert(file_path: &path::Path, output_dir: &path::Path) -> anyhow::Result<String> {
    let Some(extension) = file_path.extension() else {
        return Err(anyhow::Error::msg(format!(
            "`{}` has no extension",
            file_path.to_str().unwrap()
        )));
    };
    let extension = extension.to_str().unwrap();

    let format = match extension {
        "png" => image::ImageFormat::Png,
        "tiff" | "tif" => image::ImageFormat::Tiff,
        _ => {
            return Err(anyhow::Error::msg(format!(
                "`{}` has unsupported extension",
                file_path.to_str().unwrap()
            )));
        }
    };

    let file = io::BufReader::new(fs::File::open(file_path)?);
    let img = image::load(file, format)?;

    let buffer = img.to_rgb8();
    let file_name_base = String::from(file_path.file_stem().unwrap().to_str().unwrap());

    let new_file = format!("{}/{file_name_base}.jpg", output_dir.to_str().unwrap());
    image::save_buffer_with_format(
        &new_file,
        &buffer,
        buffer.width(),
        buffer.height(),
        ExtendedColorType::Rgb8,
        image::ImageFormat::Jpeg,
    )?;

    Ok(new_file)
}
