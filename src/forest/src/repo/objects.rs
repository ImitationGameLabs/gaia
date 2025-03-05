
use candid::{CandidType, Deserialize};
use gix_actor::bstr::BStr;
use gix_hash::{oid, ObjectId};
use gix_object::{
    BlobRef, TreeRef, CommitRef, TagRef, ObjectRef,
    tree::{EntryMode, EntryRef}
};
type Timestamp = u64;

#[derive(Debug, Clone, PartialEq, CandidType)]
pub enum Type {
    Blob,
    Tree,
    Commit,
    Tag,
}

#[derive(Debug, Clone, CandidType)]
pub enum Object<'a> {
    Blob(Blob<'a>),
    Tree(Tree<'a>),
    Commit(Commit<'a>),
    Tag(Tag<'a>),
}

impl<'a> From<ObjectRef<'a>> for Object<'a> {
    fn from(value: ObjectRef<'a>) -> Self {
        match value {
            ObjectRef::Blob(b) => Object::Blob(b.into()),
            ObjectRef::Tree(t) => Object::Tree(Tree {
                entries: t.entries.into_iter().map(|e| e.into()).collect()
            }),
            ObjectRef::Commit(c) => Object::Commit(c.into()),
            ObjectRef::Tag(t) => Object::Tag(t.into()),
        }
    }
}

#[derive(Debug, Clone, CandidType)]
pub struct Blob<'a> {
    pub data: &'a [u8],
}

impl <'a>From<BlobRef<'a>> for Blob<'a> {
    fn from(value: BlobRef<'a>) -> Self {
        Self { data: value.data.into() }
    }
}

impl <'a>Blob<'a> {
    pub fn to_gix_blob(&self) -> gix_object::BlobRef<'a> {
        BlobRef { data: self.data }
    }
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct Entry<'a> {
    pub mode: u16,
    pub filename: &'a [u8],
    pub oid: &'a [u8],
}

impl <'a> From<EntryRef<'a>> for Entry<'a> {
    fn from(value: EntryRef<'a>) -> Self {
        Self {
            mode: *value.mode,
            filename: value.filename,
            oid: value.oid.as_bytes(),
        }
    }
}


impl <'a>Entry<'a> {
    pub fn to_gix_entry_ref(self) -> EntryRef<'a> {
        EntryRef {
            mode: EntryMode(self.mode),
            filename: self.filename.into(),
            oid: oid::try_from_bytes(self.oid).unwrap(),
        }
    }
}

#[derive(Debug, Clone, CandidType)]
pub struct Tree<'a> {
    pub entries: Vec<Entry<'a>>,
}

#[derive(Debug, Clone, CandidType)]
pub struct Signature<'a>  {
    name: &'a [u8],
    email: &'a [u8],
    time: Timestamp,
}

impl<'a> From<gix_actor::SignatureRef<'a>> for Signature<'a> {
    fn from(value: gix_actor::SignatureRef<'a>) -> Self {
        Self {
            name: value.name,
            email: value.email,
            time: value.time.seconds as u64,
        }
    }
}

impl<'a> Signature<'a> {
    pub fn to_gix_signature_ref(&self) -> gix_actor::SignatureRef<'a> {
        gix_actor::SignatureRef {
            name: self.name.into(),
            email: self.email.into(),
            time: gix_date::Time::new(self.time as i64, 0),
        }
    }
}

#[derive(Debug, Clone, CandidType)]
pub struct Commit<'a> {
    pub tree_hash: &'a [u8],
    pub parent_hashes: Vec<&'a [u8]>,
    pub author: Signature<'a>,
    pub committer: Signature<'a>,
    pub message: &'a [u8],
}

impl<'a> From<CommitRef<'a>> for Commit<'a> {
    fn from(value: CommitRef<'a>) -> Self {
        Self {
            tree_hash: value.tree,
            parent_hashes: value.parents.into_iter().map(|p| p.into()).collect(),
            author: value.author.into(),
            committer: value.committer.into(),
            message: value.message,
        }
    }
}

impl<'a> Commit<'a> {
    pub fn to_gix_commit_ref(self) -> CommitRef<'a> {
        gix_object::CommitRef {
            tree: BStr::new(self.tree_hash),
            parents: self.parent_hashes.iter()
                .map(|h| (*h).into())
                .collect(),
            author: self.author.to_gix_signature_ref(),
            committer: self.committer.to_gix_signature_ref(),
            message: self.message.into(),
            extra_headers: Default::default(),
            encoding: None,
        }
    }
}

#[derive(Debug, Clone, CandidType)]
pub struct Tag<'a> {
    pub target: &'a [u8],
    pub target_kind: Type,
    /// The name of the tag, e.g. "v1.0".
    pub name: &'a [u8],
    /// The author of the tag.
    pub tagger: Option<Signature<'a>>,
    /// The message describing this release.
    pub message: &'a [u8],
    /// A cryptographic signature over the entire content of the serialized tag object thus far.
    pub pgp_signature: Option<&'a [u8]>,
}

impl<'a> From<TagRef<'a>> for Tag<'a> {
    fn from(value: TagRef<'a>) -> Self {
        Self {
            target: value.target,
            target_kind: match value.target_kind {
                gix_object::Kind::Blob => Type::Blob,
                gix_object::Kind::Tree => Type::Tree,
                gix_object::Kind::Commit => Type::Commit,
                gix_object::Kind::Tag => Type::Tag,
            },
            name: value.name,
            tagger: value.tagger.map(|s| s.into()),
            message: value.message,
            pgp_signature: value.pgp_signature.map(|s| s.into()),
        }
    }
}

impl<'a> Tag<'a> {
    pub fn to_gix_tag_ref(self) -> TagRef<'a> {
        TagRef {
            target: self.target.into(),
            target_kind: match self.target_kind {
                Type::Blob => gix_object::Kind::Blob,
                Type::Tree => gix_object::Kind::Tree,
                Type::Commit => gix_object::Kind::Commit,
                Type::Tag => gix_object::Kind::Tag,
            },
            name: self.name.into(),
            tagger: self.tagger.map(|s| s.to_gix_signature_ref()),
            message: self.message.into(),
            pgp_signature: self.pgp_signature.map(|s| s.into()),
        }
    }
}