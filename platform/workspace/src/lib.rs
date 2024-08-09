use std::path::PathBuf;

pub struct Workspace {
    pub id: String,
    pub folders: Vec<String>,
    pub configuration: PathBuf,
}
