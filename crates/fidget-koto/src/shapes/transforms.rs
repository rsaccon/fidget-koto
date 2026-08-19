use fidget::{
    context::Tree,
    shapes::{Move, Scale, types::Vec3},
};
use koto::{derive::*, prelude::*, runtime};
use std::fmt;

use crate::KotoTree;

/// KotoObject wrapper for fidget Move
#[derive(Clone, KotoCopy, KotoType)]
pub struct KotoMove(Move);

impl KotoObject for KotoMove {
    fn display(&self, ctx: &mut DisplayContext) -> runtime::Result<()> {
        ctx.append(self.to_string());
        Ok(())
    }
}

impl From<Move> for KotoMove {
    fn from(tree: Move) -> Self {
        Self(tree)
    }
}

impl From<KotoMove> for KValue {
    fn from(obj: KotoMove) -> Self {
        KObject::from(obj).into()
    }
}

impl fmt::Display for KotoMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Move{{x: {}, y: {}, z: {}}}",
            self.0.offset.x, self.0.offset.y, self.0.offset.z
        )
    }
}

#[koto_impl]
impl KotoMove {
    /// Create KotoObject representing fidget::shapes::Move
    #[allow(clippy::new_ret_no_self)]
    pub fn new(shape: Tree, x: f64, y: f64, z: f64) -> KObject {
        KObject::from(Self(Move {
            shape,
            offset: Vec3 { x, y, z },
        }))
    }

    /// Access the inner fidget Move struct
    pub fn inner(&self) -> Move {
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

/// KotoObject wrapper for fidget Scale
#[derive(Clone, KotoCopy, KotoType)]
pub struct KotoScale(Scale);

impl KotoObject for KotoScale {
    fn display(&self, ctx: &mut DisplayContext) -> runtime::Result<()> {
        ctx.append(self.to_string());
        Ok(())
    }
}

impl From<Scale> for KotoScale {
    fn from(tree: fidget::shapes::Scale) -> Self {
        Self(tree)
    }
}

impl From<KotoScale> for KValue {
    fn from(obj: KotoScale) -> Self {
        KObject::from(obj).into()
    }
}

impl fmt::Display for KotoScale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Scale{{x: {}, y: {}, z: {}}}",
            self.0.scale.x, self.0.scale.y, self.0.scale.z
        )
    }
}

#[koto_impl]
impl KotoScale {
    /// Create KotoObject representing fidget::shapes::Scale
    #[allow(clippy::new_ret_no_self)]
    pub fn new(shape: Tree, x: f64, y: f64, z: f64) -> KObject {
        KObject::from(Self(Scale {
            shape,
            scale: Vec3 { x, y, z },
        }))
    }

    /// Access the inner fidget Move struct
    pub fn inner(&self) -> Scale {
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
