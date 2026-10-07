use std::path::PathBuf;

pub struct Document {
    pub id: usize,
    pub path: PathBuf,
    pub content: String,
}