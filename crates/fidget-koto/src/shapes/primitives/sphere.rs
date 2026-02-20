use fidget::{
    context::Tree,
    shapes::{Sphere, types::Vec3},
};
use koto::{derive::*, prelude::*, runtime};
use std::fmt;

use crate::KotoTree;

/// KotoObject wrapper for fidget Sphere
#[derive(Clone, KotoCopy, KotoType)]
pub struct KotoSphere(Sphere);

impl KotoObject for KotoSphere {
    fn display(&self, ctx: &mut DisplayContext) -> runtime::Result<()> {
        ctx.append(self.to_string());
        Ok(())
    }
}

impl From<Sphere> for KotoSphere {
    fn from(tree: Sphere) -> Self {
        Self(tree)
    }
}

impl From<KotoSphere> for KValue {
    fn from(obj: KotoSphere) -> Self {
        KObject::from(obj).into()
    }
}

impl fmt::Display for KotoSphere {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Sphere{{radius: {}, x: {}, y: {}, z: {}}}",
            self.0.radius, self.0.center.x, self.0.center.y, self.0.center.z
        )
    }
}

#[koto_impl]
impl KotoSphere {
    /// Create KotoObject representing fidget::shapes::Sphere
    #[allow(clippy::new_ret_no_self)]
    pub fn new(radius: f64, x: f64, y: f64, z: f64) -> KObject {
        KObject::from(Self(Sphere {
            radius,
            center: Vec3 { x, y, z },
        }))
    }

    /// Access the inner fidget Sphere struct
    pub fn inner(&self) -> Sphere {
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
