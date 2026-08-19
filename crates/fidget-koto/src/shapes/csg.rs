use fidget::{
    context::Tree,
    shapes::{Difference, Intersection, Inverse, Union},
};
use koto::{derive::*, prelude::*, runtime};
use std::fmt;

use crate::KotoTree;

/// KotoObject wrapper for fidget Difference
#[derive(Clone, KotoCopy, KotoType)]
pub struct KotoDifference(Difference);

impl KotoObject for KotoDifference {
    fn display(&self, ctx: &mut DisplayContext) -> runtime::Result<()> {
        ctx.append(self.to_string());
        Ok(())
    }
}

impl From<Difference> for KotoDifference {
    fn from(tree: Difference) -> Self {
        Self(tree)
    }
}

impl From<KotoDifference> for KValue {
    fn from(obj: KotoDifference) -> Self {
        KObject::from(obj).into()
    }
}

impl fmt::Display for KotoDifference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Difference{{}}")
    }
}

#[koto_impl]
impl KotoDifference {
    /// Create KotoObject representing fidget::shapes::Difference
    #[allow(clippy::new_ret_no_self)]
    pub fn new(a: Tree, b: Tree) -> KObject {
        KObject::from(KotoDifference(Difference {
            shape: a,
            cutout: b,
        }))
    }

    /// Access the inner fidget Difference struct
    pub fn inner(&self) -> Difference {
        self.to_owned().0
    }

    /// Access the inner fidget Tree struct
    #[koto_method]
    fn tree(&self) -> runtime::Result<KValue> {
        Ok(KValue::Object(KObject::from(KotoTree::from(Tree::from(
            self.inner(),
        )))))
    }
}

/// KotoObject wrapper for fidget Intersection
#[derive(Clone, KotoCopy, KotoType)]
pub struct KotoIntersection(Intersection);

impl KotoObject for KotoIntersection {
    fn display(&self, ctx: &mut DisplayContext) -> runtime::Result<()> {
        ctx.append(self.to_string());
        Ok(())
    }
}

impl From<Intersection> for KotoIntersection {
    fn from(tree: Intersection) -> Self {
        Self(tree)
    }
}

impl From<KotoIntersection> for KValue {
    fn from(obj: KotoIntersection) -> Self {
        KObject::from(obj).into()
    }
}

impl fmt::Display for KotoIntersection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Intersection{{}}")
    }
}

#[koto_impl]
impl KotoIntersection {
    /// Create KotoObject representing fidget::shapes::Intersection
    #[allow(clippy::new_ret_no_self)]
    pub fn new(a: Tree, b: Tree) -> KObject {
        KObject::from(Self(Intersection { input: vec![a, b] }))
    }

    /// Access the inner fidget Intersection struct
    pub fn inner(&self) -> Intersection {
        self.to_owned().0
    }

    /// Access the inner fidget Tree struct
    #[koto_method]
    fn tree(&self) -> runtime::Result<KValue> {
        Ok(KValue::Object(KObject::from(KotoTree::from(Tree::from(
            self.inner(),
        )))))
    }
}

/// KotoObject wrapper for fidget Inverse
#[derive(Clone, KotoCopy, KotoType)]
pub struct KotoInverse(Inverse);

impl KotoObject for KotoInverse {
    fn display(&self, ctx: &mut DisplayContext) -> runtime::Result<()> {
        ctx.append(self.to_string());
        Ok(())
    }
}

impl From<Inverse> for KotoInverse {
    fn from(tree: Inverse) -> Self {
        Self(tree)
    }
}

impl From<KotoInverse> for KValue {
    fn from(obj: KotoInverse) -> Self {
        KObject::from(obj).into()
    }
}

impl fmt::Display for KotoInverse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Inverse{{}}")
    }
}

#[koto_impl]
impl KotoInverse {
    /// Create KotoObject representing fidget::shapes::Inverse
    #[allow(clippy::new_ret_no_self)]
    pub fn new(shape: Tree) -> KObject {
        KObject::from(Self(Inverse { shape }))
    }

    /// Access the inner fidget Inverse struct
    pub fn inner(&self) -> Inverse {
        self.to_owned().0
    }

    /// Access the inner fidget Tree struct
    #[koto_method]
    fn tree(&self) -> runtime::Result<KValue> {
        Ok(KValue::Object(KObject::from(KotoTree::from(Tree::from(
            self.inner(),
        )))))
    }
}

/// KotoObject wrapper for fidget Union
#[derive(Clone, KotoCopy, KotoType)]
pub struct KotoUnion(Union);

impl KotoObject for KotoUnion {
    fn display(&self, ctx: &mut DisplayContext) -> runtime::Result<()> {
        ctx.append(self.to_string());
        Ok(())
    }
}

impl From<Union> for KotoUnion {
    fn from(tree: Union) -> Self {
        Self(tree)
    }
}

impl From<KotoUnion> for KValue {
    fn from(obj: KotoUnion) -> Self {
        KObject::from(obj).into()
    }
}

impl fmt::Display for KotoUnion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Union{{}}")
    }
}

#[koto_impl]
impl KotoUnion {
    /// Create KotoObject representing fidget::shapes::Union
    #[allow(clippy::new_ret_no_self)]
    pub fn new(a: Tree, b: Tree) -> KObject {
        KObject::from(Self(Union { input: vec![a, b] }))
    }

    /// Access the inner fidget Union struct
    pub fn inner(&self) -> Union {
        self.to_owned().0
    }

    /// Access the inner fidget Tree struct
    #[koto_method]
    fn tree(&self) -> runtime::Result<KValue> {
        Ok(KValue::Object(KObject::from(KotoTree::from(Tree::from(
            self.inner(),
        )))))
    }
}
