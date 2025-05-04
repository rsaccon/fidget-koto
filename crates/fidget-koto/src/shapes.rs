// use fidget::context::Tree;
use koto::{derive::*, prelude::*, runtime};
use std::fmt;

type Inner = fidget::shapes::Sphere;

/// KotoObject wrapper for fidget Sphere
#[derive(Clone, KotoCopy, KotoType)]
pub struct Sphere(Inner);

impl KotoObject for Sphere {
    fn display(&self, ctx: &mut DisplayContext) -> runtime::Result<()> {
        ctx.append(self.to_string());
        Ok(())
    }
}

impl From<Inner> for Sphere {
    fn from(tree: Inner) -> Self {
        Self(tree)
    }
}

impl From<Sphere> for KValue {
    fn from(obj: Sphere) -> Self {
        KObject::from(obj).into()
    }
}

impl fmt::Display for Sphere {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "Vec2{{x: {}, y: {}}}", self.0.x, self.0.y)
        write!(f, "Sphere{{}}")
    }
}
#[koto_impl]
impl Sphere {
    /// Create KotoObject representing fidget::shapes::Sphere
    pub fn new(radius: f64, x: f64, y: f64, z: f64) -> KObject {
        KObject::from(Self(
            Inner {
                center: fidget::shapes::Vec3 { x, y, z },
                radius,
            }
            .into(),
        ))
    }

    /// Access the inner fidget Tree struct
    pub fn inner(&self) -> Inner {
        self.to_owned().0
    }
}
