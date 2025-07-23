use thiserror::Error;
#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to deserialize request: {0}.\nThis may be a versioning issue between pgc and the codegen plugin being used.")]
    RequestDeserialization(#[from] serde_json::Error),
}
