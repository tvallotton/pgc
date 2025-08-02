use std::rc::Rc;

use thiserror::Error;
#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to deserialize request: {0}.\nThis may be a versioning issue between pgc and the codegen plugin being used.")]
    RequestDeserialization(#[from] serde_json::Error),

    #[error("language {0} is not supported.")]
    NotSupportedLanguage(Rc<str>),

    #[error("failed to render or parse a template: {0}.\nThis is a bug in pgc, please report the issue at \"https://github.com/tvallotton/pgc\".")]
    TemplateError(#[from] minijinja::Error),
}
