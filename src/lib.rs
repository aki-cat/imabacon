use std::fs;
use std::io;
use std::path;

use image::ExtendedColorType;

pub fn convert(file_path: &path::Path, output_dir: &path::Path) -> anyhow::Result<()> {
    let file = io::BufReader::new(fs::File::open(file_path)?);
    let img = image::load(file, image::ImageFormat::Png)?;

    let buffer = img.to_rgb8();
    let file_name_base = String::from(file_path.file_stem().unwrap().to_str().unwrap());
    println!("{:?}", file_name_base);

    image::save_buffer_with_format(
        format!("{}/{file_name_base}.jpeg", output_dir.to_str().unwrap()),
        &buffer,
        buffer.width(),
        buffer.height(),
        ExtendedColorType::Rgb8,
        image::ImageFormat::Jpeg,
    )?;

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
