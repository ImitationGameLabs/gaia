
#[derive(Debug, Clone)]
pub struct Ref {
    pub name: String,          // The name of the reference (e.g., "main", "v1.0", "HEAD")
    pub target_hash: String,   // The hash of the object this reference points to (usually a commit)
    pub ref_type: RefType,     // The type of reference (branch, tag, etc.)
}

#[derive(Debug, Clone, PartialEq)]
pub enum RefType {
    Branch,                    // A branch reference
    Tag,                       // A tag reference
    Head,                      // The HEAD reference
    Remote,                    // A remote reference (e.g., "origin/main")
    Other,                     // Other types of references
}
