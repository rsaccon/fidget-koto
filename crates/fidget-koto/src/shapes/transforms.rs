use koto::{derive::*, prelude::*, runtime};
use std::fmt;

use fidget::context::Tree;
use fidget::shapes::{Move, Scale, Vec3};

/// KotoObject wrapper for fidget Move
#[derive(Clone, KotoCopy, KotoType)]
pub struct KMove(Move);

impl KotoObject for KMove {
    fn display(&self, ctx: &mut DisplayContext) -> runtime::Result<()> {
        ctx.append(self.to_string());
        Ok(())
    }
}

impl From<Move> for KMove {
    fn from(tree: Move) -> Self {
        Self(tree)
    }
}

impl From<KMove> for KValue {
    fn from(obj: KMove) -> Self {
        KObject::from(obj).into()
    }
}

impl fmt::Display for KMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Move{{x: {}, y: {}, z: {}}}",
            self.0.offset.x, self.0.offset.y, self.0.offset.z
        )
    }
}
#[koto_impl]
impl KMove {
    /// Create KotoObject representing fidget::shapes::Move
    pub fn new(shape: Tree, x: f64, y: f64, z: f64) -> KObject {
        KObject::from(Self(
            Move {
                shape,
                offset: Vec3 { x, y, z },
            }
            .into(),
        ))
    }

    /// Access the inner fidget Tree struct
    pub fn inner(&self) -> Move {
        self.to_owned().0
    }
}

/// KotoObject wrapper for fidget Scale
#[derive(Clone, KotoCopy, KotoType)]
pub struct KScale(Scale);

impl KotoObject for KScale {
    fn display(&self, ctx: &mut DisplayContext) -> runtime::Result<()> {
        ctx.append(self.to_string());
        Ok(())
    }
}

impl From<Scale> for KScale {
    fn from(tree: fidget::shapes::Scale) -> Self {
        Self(tree)
    }
}

impl From<KScale> for KValue {
    fn from(obj: KScale) -> Self {
        KObject::from(obj).into()
    }
}

impl fmt::Display for KScale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Scale{{x: {}, y: {}, z: {}}}",
            self.0.scale.x, self.0.scale.y, self.0.scale.z
        )
    }
}
#[koto_impl]
impl KScale {
    /// Create KotoObject representing fidget::shapes::Scale
    pub fn new(shape: Tree, x: f64, y: f64, z: f64) -> KObject {
        KObject::from(Self(
            Scale {
                shape,
                scale: Vec3 { x, y, z },
            }
            .into(),
        ))
    }

    /// Access the inner fidget Tree struct
    pub fn inner(&self) -> Scale {
        self.to_owned().0
    }
}
