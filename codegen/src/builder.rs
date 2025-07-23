use super::error::Error;
use crate::{
    query_namespace::QueryNamespace,
    request::Request,
    response::{File, Response},
};
pub struct Builder {
    request: Request,
    query_namespace: QueryNamespace,
}

impl Builder {
    pub fn new(request: Request) -> Self {
        Builder {
            request,
            query_namespace: QueryNamespace::new(),
        }
    }

    pub fn build(&self) -> Result<Response, Error> {
        Ok(Response { files: vec![] })
    }
}
