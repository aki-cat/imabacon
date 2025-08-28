use std::fs;
use std::io;
use std::path;

use image::ExtendedColorType;

pub fn convert(file_path: &path::Path) -> anyhow::Result<()> {
    let file = io::BufReader::new(fs::File::open(file_path)?);
    let img = image::load(file, image::ImageFormat::Png)?;

    let buffer = img.to_rgb8();
    let file_name_base = file_path.file_stem().unwrap();
    println!("{:?}", file_name_base);

    image::save_buffer_with_format(
        format!("out/{}.jpeg", file_name_base.to_str().unwrap()),
        &buffer,
        buffer.width(),
        buffer.height(),
        ExtendedColorType::Rgb8,
        image::ImageFormat::Jpeg,
    )?;

    Ok(())
}

pub fn list_files() -> anyhow::Result<Vec<String>> {
    let out = fs::read_dir("in")?
        .filter_map(|dir_entry| {
            let entry = dir_entry.unwrap();
            let entry_type = entry.file_type().unwrap();

            match entry_type.is_file() {
                true => Some(String::from(entry.path().as_path().to_str().unwrap())),
                false => None,
            }
        })
        .collect();
    Ok(out)
}
