use anyhow::{anyhow, Error};
use md5::{Digest, Md5};
use std::{fs::File, io::Read};
use zip::ZipArchive;

mod qoi;

fn main() -> Result<(), Error> {
    let file_arg = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow!("could not get file arg"))?;
    let file = File::open(&file_arg)?;

    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;

        let mut file_content = Vec::new();
        entry.read_to_end(&mut file_content)?;

        let (qoi_header, pixel_data) = qoi::process_qoi(&file_content)?;

        println!(
            "{} {} {} {}",
            entry.name(),
            qoi_header.width,
            qoi_header.height,
            qoi_header.channels as u8,
        );

        println!("{:x}", Md5::digest(&pixel_data));
    }

    Ok(())
}
