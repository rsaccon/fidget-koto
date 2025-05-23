use fidget::{
    context::Tree,
    shapes::{Circle, Vec2},
};
use koto::{derive::*, prelude::*, runtime};
use std::fmt;

use crate::KTree;

/// KotoObject wrapper for fidget Circle
#[derive(Clone, KotoCopy, KotoType)]
#[koto(type_name = "Circle")]
pub struct KCircle(Circle);

impl KotoObject for KCircle {
    fn display(&self, ctx: &mut DisplayContext) -> runtime::Result<()> {
        ctx.append(self.to_string());
        Ok(())
    }
}

impl From<Circle> for KCircle {
    fn from(tree: Circle) -> Self {
        Self(tree)
    }
}

impl From<KCircle> for KValue {
    fn from(obj: KCircle) -> Self {
        KObject::from(obj).into()
    }
}

impl fmt::Display for KCircle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Circle{{radius: {}, x: {}, y: {}}}",
            self.0.radius, self.0.center.x, self.0.center.y
        )
    }
}

#[koto_impl]
impl KCircle {
    /// Create KotoObject representing fidget::shapes::Circle
    pub fn new(radius: f64, x: f64, y: f64) -> KObject {
        KObject::from(Self(
            Circle {
                radius,
                center: Vec2 { x, y },
            }
            .into(),
        ))
    }

    /// Access the inner fidget Circle struct
    pub fn inner(&self) -> Circle {
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
