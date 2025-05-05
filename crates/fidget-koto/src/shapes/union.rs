use koto::{derive::*, prelude::*, runtime};
use std::fmt;

use fidget::context::Tree;

type Inner = fidget::shapes::Union;

/// KotoObject wrapper for fidget Intersection
#[derive(Clone, KotoCopy, KotoType)]
#[koto(type_name = "union")]
pub struct KUnion(Inner);

impl KotoObject for KUnion {
    fn display(&self, ctx: &mut DisplayContext) -> runtime::Result<()> {
        ctx.append(self.to_string());
        Ok(())
    }
}

impl From<Inner> for KUnion {
    fn from(tree: Inner) -> Self {
        Self(tree)
    }
}

impl From<KUnion> for KValue {
    fn from(obj: KUnion) -> Self {
        KObject::from(obj).into()
    }
}

impl fmt::Display for KUnion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Union{{}}")
    }
}
#[koto_impl]
impl KUnion {
    /// Create KotoObject representing fidget::shapes::Intersection
    pub fn new(a: Tree, b: Tree) -> KObject {
        KObject::from(Self(Inner { input: vec![a, b] }.into()))
    }

    /// Access the inner fidget Tree struct
    pub fn inner(&self) -> Inner {
        self.to_owned().0
    }
}
