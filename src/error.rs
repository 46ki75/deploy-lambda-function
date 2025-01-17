#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid Option: {0}")]
    InvalidOption(String),

    #[error("Failed to create temp dir")]
    TempDir(#[from] std::io::Error),

    #[error("Failed to create zip archive")]
    ZipArchive(#[from] zip::result::ZipError),

    #[error("Failed to explore directory")]
    WalkDir(#[from] walkdir::Error),

    #[error("Failed to strip prefix {0}")]
    StripPrefix(#[from] std::path::StripPrefixError),

    // #[error("Failed to call lambda API")]
    // Lambda(
    //     #[from]
    //     aws_smithy_runtime_api::client::result::SdkError<
    //         aws_sdk_lambda::operation::update_function_code::UpdateFunctionCodeError,
    //         aws_smithy_runtime_api::client::orchestrator::HttpResponse,
    //     >,
    // ),
    #[error("Failed to call lambda API")]
    UpdateFunctionCodeError(String),
}
