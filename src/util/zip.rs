use std::io::{Read, Write};

pub async fn archive_to_zip_bytes(directory_path: &str) -> Result<Vec<u8>, crate::error::Error> {
    let absolute_directory_path = std::path::Path::new(directory_path).canonicalize()?;

    println!(
        "Create archive from directory: {:?}",
        absolute_directory_path
    );

    let mut zip_bytes = Vec::new();

    let mut zip_writer = zip::ZipWriter::new(std::io::Cursor::new(&mut zip_bytes));

    let options: zip::write::FileOptions<zip::write::ExtendedFileOptions> =
        zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    println!("Exploring directory: {:?}", absolute_directory_path);

    for entry in walkdir::WalkDir::new(&absolute_directory_path) {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let absolute_path = path.canonicalize()?;

            let file_name = absolute_path
                .strip_prefix(&absolute_directory_path)?
                .to_string_lossy();

            zip_writer.start_file(&file_name, options.clone())?;

            let mut file = std::fs::File::open(path)?;

            let mut buffer = Vec::new();

            file.read_to_end(&mut buffer)?;

            zip_writer.write_all(&buffer)?;

            println!("Added file: {:?}", file_name);
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
    async fn test_zip_relative() {
        let result = super::archive_to_zip_bytes("./src").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_zip_relative_abstruct() {
        let result = super::archive_to_zip_bytes("src").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_zip_absolute() {
        let result = super::archive_to_zip_bytes(
            std::path::Path::new("src")
                .canonicalize()
                .unwrap()
                .to_str()
                .unwrap(),
        )
        .await;
        assert!(result.is_ok());
    }
}
