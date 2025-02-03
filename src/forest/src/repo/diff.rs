
use std::path::PathBuf;

#[derive(Debug)]
pub enum Diff {
    Added(PathBuf),
    Modified {
        path: PathBuf,
        old: String,
        new: String,
    },
    Deleted(PathBuf),
    Renamed { old_path: PathBuf, new_path: PathBuf },
    ContentChanged { path: PathBuf, hunks: Vec<DiffHunk> },
}

#[derive(Debug)]
pub struct DiffHunk {
    pub old_start: u32,
    pub old_lines: u32,
    pub new_start: u32,
    pub new_lines: u32,
    pub content: String,
}
