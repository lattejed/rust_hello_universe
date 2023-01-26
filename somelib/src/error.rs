/// `Error` for hello. There is no std lib `Error` so we can call this `Error` if we want. We
/// could also call it `HelloError` or whatever makes sense.
///
/// If we use the fully qualified `thiserror::Error`, we don't have to use `use thiserror`. In this
/// case we use `thiserror::Error` to prevent a collision with our `Error`
#[derive(Debug, thiserror::Error)]
pub enum Error {
    // Automatically gives use the required `Display` impl
    #[error("two consecutive random values found")]
    ConsecutiveRandom,
}
