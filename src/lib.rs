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
