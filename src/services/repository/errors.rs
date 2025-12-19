#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("chain not found")]
    NotFound,

    #[error(transparent)]
    Db(#[from] sqlx::Error),
}
