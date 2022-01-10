mod api;
mod bridge_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */
mod harmonic_partials;
mod tuner;

#[derive(Debug)]
pub enum TunerError {
    FftFailed,
}

impl std::fmt::Display for TunerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TunerError is here!")
    }
}

impl std::error::Error for TunerError {}
