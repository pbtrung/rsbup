use slog::Logger;
use std::path::PathBuf;

#[derive(Clone)]
pub struct Repo {
    name: String,
    config_dir: PathBuf,
    compression_level: u32,
    log: Logger,
}