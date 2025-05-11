use fidget::context::Tree;
use fidget::shapes::{Circle, Sphere, Vec2, Vec3};
use koto::{derive::*, prelude::*, runtime};
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

use super::super::super::KTree;

/// KotoObject wrapper for fidget Sphere
#[derive(Clone, KotoCopy, KotoType)]
pub struct KSphere(Sphere);

impl KotoObject for KSphere {
    fn display(&self, ctx: &mut DisplayContext) -> runtime::Result<()> {
        ctx.append(self.to_string());
        Ok(())
    }
}

impl From<Sphere> for KSphere {
    fn from(tree: Sphere) -> Self {
        Self(tree)
    }
}

impl From<KSphere> for KValue {
    fn from(obj: KSphere) -> Self {
        KObject::from(obj).into()
    }
}

impl fmt::Display for KSphere {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Sphere{{radius: {}, x: {}, y: {}, z: {}}}",
            self.0.radius, self.0.center.x, self.0.center.y, self.0.center.z
        )
    }
}

#[koto_impl]
impl KSphere {
    /// Create KotoObject representing fidget::shapes::Sphere
    pub fn new(radius: f64, x: f64, y: f64, z: f64) -> KObject {
        KObject::from(Self(
            Sphere {
                radius,
                center: Vec3 { x, y, z },
            }
            .into(),
        ))
    }

    /// Access the inner fidget Sphere struct
    pub fn inner(&self) -> Sphere {
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
