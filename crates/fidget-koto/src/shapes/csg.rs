use fidget::{
    context::Tree,
    shapes::{Difference, Intersection, Inverse, Union},
};
use koto::{derive::*, prelude::*, runtime};
use std::fmt;

use crate::KTree;

/// KotoObject wrapper for fidget Difference
#[derive(Clone, KotoCopy, KotoType)]
#[koto(type_name = "Difference")]
pub struct KDifference(Difference);

impl KotoObject for KDifference {
    fn display(&self, ctx: &mut DisplayContext) -> runtime::Result<()> {
        ctx.append(self.to_string());
        Ok(())
    }
}

impl From<Difference> for KDifference {
    fn from(tree: Difference) -> Self {
        Self(tree)
    }
}

impl From<KDifference> for KValue {
    fn from(obj: KDifference) -> Self {
        KObject::from(obj).into()
    }
}

impl fmt::Display for KDifference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Difference{{}}")
    }
}

#[koto_impl]
impl KDifference {
    /// Create KotoObject representing fidget::shapes::Difference
    pub fn new(a: Tree, b: Tree) -> KObject {
        KObject::from(KDifference(
            Difference {
                shape: a,
                cutout: b,
            }
            .into(),
        ))
    }

    /// Access the inner fidget Difference struct
    pub fn inner(&self) -> Difference {
        self.to_owned().0
    }

    /// Access the inner fidget Tree struct
    #[koto_method]
    fn tree(&self) -> runtime::Result<KValue> {
        Ok(KValue::Object(KObject::from(KTree::from(Tree::from(
            self.inner(),
        )))))
    }
}

/// KotoObject wrapper for fidget Intersection
#[derive(Clone, KotoCopy, KotoType)]
#[koto(type_name = "Intersection")]
pub struct KIntersection(Intersection);

impl KotoObject for KIntersection {
    fn display(&self, ctx: &mut DisplayContext) -> runtime::Result<()> {
        ctx.append(self.to_string());
        Ok(())
    }
}

impl From<Intersection> for KIntersection {
    fn from(tree: Intersection) -> Self {
        Self(tree)
    }
}

impl From<KIntersection> for KValue {
    fn from(obj: KIntersection) -> Self {
        KObject::from(obj).into()
    }
}

impl fmt::Display for KIntersection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Intersection{{}}")
    }
}

#[koto_impl]
impl KIntersection {
    /// Create KotoObject representing fidget::shapes::Intersection
    pub fn new(a: Tree, b: Tree) -> KObject {
        KObject::from(Self(Intersection { input: vec![a, b] }.into()))
    }

    /// Access the inner fidget Intersection struct
    pub fn inner(&self) -> Intersection {
        self.to_owned().0
    }

    /// Access the inner fidget Tree struct
    #[koto_method]
    fn tree(&self) -> runtime::Result<KValue> {
        Ok(KValue::Object(KObject::from(KTree::from(Tree::from(
            self.inner(),
        )))))
    }
}

/// KotoObject wrapper for fidget Inverse
#[derive(Clone, KotoCopy, KotoType)]
#[koto(type_name = "Inverse")]
pub struct KInverse(Inverse);

impl KotoObject for KInverse {
    fn display(&self, ctx: &mut DisplayContext) -> runtime::Result<()> {
        ctx.append(self.to_string());
        Ok(())
    }
}

impl From<Inverse> for KInverse {
    fn from(tree: Inverse) -> Self {
        Self(tree)
    }
}

impl From<KInverse> for KValue {
    fn from(obj: KInverse) -> Self {
        KObject::from(obj).into()
    }
}

impl fmt::Display for KInverse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Inverse{{}}")
    }
}

#[koto_impl]
impl KInverse {
    /// Create KotoObject representing fidget::shapes::Inverse
    pub fn new(shape: Tree) -> KObject {
        KObject::from(Self(Inverse { shape }.into()))
    }

    /// Access the inner fidget Inverse struct
    pub fn inner(&self) -> Inverse {
        self.to_owned().0
    }

    /// Access the inner fidget Tree struct
    #[koto_method]
    fn tree(&self) -> runtime::Result<KValue> {
        Ok(KValue::Object(KObject::from(KTree::from(Tree::from(
            self.inner(),
        )))))
    }
}

/// KotoObject wrapper for fidget Union
#[derive(Clone, KotoCopy, KotoType)]
#[koto(type_name = "Union")]
pub struct KUnion(Union);

impl KotoObject for KUnion {
    fn display(&self, ctx: &mut DisplayContext) -> runtime::Result<()> {
        ctx.append(self.to_string());
        Ok(())
    }
}

impl From<Union> for KUnion {
    fn from(tree: Union) -> Self {
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
    /// Create KotoObject representing fidget::shapes::Union
    pub fn new(a: Tree, b: Tree) -> KObject {
        KObject::from(Self(Union { input: vec![a, b] }.into()))
    }

    /// Access the inner fidget Union struct
    pub fn inner(&self) -> Union {
        self.to_owned().0
    }

    /// Access the inner fidget Tree struct
    #[koto_method]
    fn tree(&self) -> runtime::Result<KValue> {
        Ok(KValue::Object(KObject::from(KTree::from(Tree::from(
            self.inner(),
        )))))
    }
}
