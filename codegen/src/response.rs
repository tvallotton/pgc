use serde::Serialize;

#[derive(Serialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Response {
    pub files: Vec<File>,
}

#[derive(Serialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct File {
    pub path: String,
    pub content: String,
}
