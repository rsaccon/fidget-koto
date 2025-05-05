use koto::{derive::*, prelude::*, runtime};
use std::fmt;

type Inner = fidget::shapes::Sphere;

/// KotoObject wrapper for fidget Sphere
#[derive(Clone, KotoCopy, KotoType)]
#[koto(type_name = "Sphere")]
pub struct KSphere(Inner);

impl KotoObject for KSphere {
    fn display(&self, ctx: &mut DisplayContext) -> runtime::Result<()> {
        ctx.append(self.to_string());
        Ok(())
    }
}

impl From<Inner> for KSphere {
    fn from(tree: Inner) -> Self {
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
            "Sphere{{radius: {}, cx: {}, cy: {}, cz: {}}}",
            self.0.radius, self.0.center.x, self.0.center.y, self.0.center.z
        )
    }
}
#[koto_impl]
impl KSphere {
    /// Create KotoObject representing fidget::shapes::Sphere
    pub fn new(radius: f64, x: f64, y: f64, z: f64) -> KObject {
        KObject::from(Self(
            Inner {
                radius,
                center: fidget::shapes::Vec3 { x, y, z },
            }
            .into(),
        ))
    }

    /// Access the inner fidget Tree struct
    pub fn inner(&self) -> Inner {
        self.to_owned().0
    }
}
