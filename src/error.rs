#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to create temp dir")]
    TempDir(#[from] std::io::Error),

    #[error("Failed to create zip archive")]
    ZipArchive(#[from] zip::result::ZipError),

    #[error("Failed to explore directory")]
    WalkDir(#[from] walkdir::Error),

    #[error("Failed to strip prefix")]
    StripPrefix(#[from] std::path::StripPrefixError),
}
