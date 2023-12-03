#[derive(thiserror::Error, Debug)]
enum MyError {
    #[error("Environment variable not found")]
    EnvironmentError(#[from] std::env::VarError),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}