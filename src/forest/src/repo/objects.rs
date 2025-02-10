
type Timestamp = u64;

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectType {
    Blob,
    Tree,
    Commit,
    Tag,
}

#[derive(Debug, Clone)]
pub struct Object {
    pub mode: i32,
    pub object_type: ObjectType,
    pub hash: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Blob {
    pub content: Vec<u8>,  
}

#[derive(Debug, Clone)]
pub struct Tree {
    pub objects: Vec<Object>,
}

#[derive(Debug, Clone)]
pub struct Commit {
    pub tree_hash: String,
    pub parent_hashes: Vec<String>,
    pub author: String,
    pub create_timestamp: Timestamp,
    pub commiter: String,
    pub commit_timestamp: Timestamp,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct Tag {
    pub commit_hash: String,
    pub name: String,
    pub tagger: String,
    pub tag_timestamp: Timestamp,
    pub message: String,
}