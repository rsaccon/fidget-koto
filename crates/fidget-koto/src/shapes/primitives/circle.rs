use fidget::{
    context::Tree,
    shapes::{Circle, types::Vec2},
};
use koto::{derive::*, prelude::*, runtime};
use std::fmt;

use crate::KotoTree;

/// KotoObject wrapper for fidget Circle
#[derive(Clone, KotoCopy, KotoType)]
pub struct KotoCircle(Circle);

impl KotoObject for KotoCircle {
    fn display(&self, ctx: &mut DisplayContext) -> runtime::Result<()> {
        ctx.append(self.to_string());
        Ok(())
    }
}

impl From<Circle> for KotoCircle {
    fn from(tree: Circle) -> Self {
        Self(tree)
    }
}

impl From<KotoCircle> for KValue {
    fn from(obj: KotoCircle) -> Self {
        KObject::from(obj).into()
    }
}

impl fmt::Display for KotoCircle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Circle{{radius: {}, x: {}, y: {}}}",
            self.0.radius, self.0.center.x, self.0.center.y
        )
    }
}

#[koto_impl]
impl KotoCircle {
    /// Create KotoObject representing fidget::shapes::Circle
    #[allow(clippy::new_ret_no_self)]
    pub fn new(radius: f64, x: f64, y: f64) -> KObject {
        KObject::from(Self(Circle {
            radius,
            center: Vec2 { x, y },
        }))
    }

    /// Access the inner fidget Circle struct
    pub fn inner(&self) -> Circle {
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
