use koto::{derive::*, prelude::*, runtime};
use std::fmt;

type Inner = fidget::shapes::Circle;

/// KotoObject wrapper for fidget Circle
#[derive(Clone, KotoCopy, KotoType)]
#[koto(type_name = "Circle")]
pub struct KCircle(Inner);

impl KotoObject for KCircle {
    fn display(&self, ctx: &mut DisplayContext) -> runtime::Result<()> {
        ctx.append(self.to_string());
        Ok(())
    }
}

impl From<Inner> for KCircle {
    fn from(tree: Inner) -> Self {
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
        // write!(f, "Vec2{{x: {}, y: {}}}", self.0.x, self.0.y)
        write!(f, "Circle{{}}")
    }
}
#[koto_impl]
impl KCircle {
    /// Create KotoObject representing fidget::shapes::Circle
    pub fn new(radius: f64, x: f64, y: f64) -> KObject {
        KObject::from(Self(
            Inner {
                center: fidget::shapes::Vec2 { x, y },
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
