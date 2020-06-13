#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    ImageError(#[from] image::ImageError),
}
