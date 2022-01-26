mod api;
mod bridge_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */
mod constants;
mod detectors;
mod tuner;

#[derive(thiserror::Error, Debug)]
pub(crate) enum TunerError {
    #[error("Unknown algorithm")]
    UnknownAlgorithm,

    #[error("Tuner not initialized")]
    TunerNotInitialized,
}
