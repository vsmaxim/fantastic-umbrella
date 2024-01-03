use std::path::{Path, PathBuf};

pub struct Paths {}

impl Paths {
    pub fn project_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).into()
    }

    pub fn openapi_path() -> PathBuf {
        Paths::project_root().join("openapi/petstore.yaml")
    }

    pub fn config_path() -> PathBuf {
        Paths::project_root().join("config.json")
    }
}
