use std::io::{Read, Write};

pub async fn archive_to_zip_bytes(directory_path: &str) -> Result<Vec<u8>, crate::error::Error> {
    println!("Create archive from directory: {:?}", directory_path);

    let mut zip_bytes = Vec::new();

    let mut zip_writer = zip::ZipWriter::new(std::io::Cursor::new(&mut zip_bytes));

    let options: zip::write::FileOptions<zip::write::ExtendedFileOptions> =
        zip::write::FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .unix_permissions(0o755);

    println!("Exploring directory: {:?}", directory_path);

    for entry in walkdir::WalkDir::new(directory_path) {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let file_name = path.strip_prefix(directory_path)?;
            zip_writer.start_file(file_name.to_string_lossy(), options.clone())?;

            let mut file = std::fs::File::open(path)?;

            let mut buffer = Vec::new();

            file.read_to_end(&mut buffer)?;

            zip_writer.write_all(&buffer)?;

            println!("Added file: {:?}", file_name);
        } else if path.is_dir() {
            let dir_name = path.strip_prefix("./")?;
            zip_writer.add_directory(dir_name.to_string_lossy(), options.clone())?;
        }
    }

    zip_writer.finish()?;

    Ok(zip_bytes)
}

#[allow(dead_code)]
pub async fn save_bytes_to_file(
    bytes: Vec<u8>,
    file_path: &str,
) -> Result<(), crate::error::Error> {
    let mut file = std::fs::File::create(file_path)?;
    file.write_all(&bytes)?;
    Ok(())
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_zip() {
        let result = super::archive_to_zip_bytes("./src").await;
        assert!(result.is_ok());
    }
}
