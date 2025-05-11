use fidget::context::Tree;
use fidget::shapes::{Circle, Vec2};
use koto::{derive::*, prelude::*, runtime};
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

use super::super::super::KTree;

/// KotoObject wrapper for fidget Circle
#[derive(Clone, KotoCopy, KotoType)]
pub struct KCircle(Circle);

impl KotoObject for KCircle {
    fn display(&self, ctx: &mut DisplayContext) -> runtime::Result<()> {
        ctx.append(self.to_string());
        Ok(())
    }

    fn negate(&self) -> runtime::Result<KValue> {
        shape_unary_op!(self, neg)
    }

    fn add(&self, other: &KValue) -> runtime::Result<KValue> {
        shape_binary_op!(self, other, add)
    }

    fn add_rhs(&self, other: &KValue) -> runtime::Result<KValue> {
        shape_binary_op_rhs!(self, other, add)
    }

    fn subtract(&self, other: &KValue) -> runtime::Result<KValue> {
        shape_binary_op!(self, other, sub)
    }

    fn subtract_rhs(&self, other: &KValue) -> runtime::Result<KValue> {
        shape_binary_op_rhs!(self, other, sub)
    }

    fn multiply(&self, other: &KValue) -> runtime::Result<KValue> {
        shape_binary_op!(self, other, mul)
    }

    fn multiply_rhs(&self, other: &KValue) -> runtime::Result<KValue> {
        shape_binary_op_rhs!(self, other, mul)
    }

    fn divide(&self, other: &KValue) -> runtime::Result<KValue> {
        shape_binary_op!(self, other, div)
    }

    fn divide_rhs(&self, other: &KValue) -> runtime::Result<KValue> {
        shape_binary_op_rhs!(self, other, div)
    }

    fn remainder(&self, other: &KValue) -> runtime::Result<KValue> {
        shape_binary_op!(self, other, modulo)
    }

    fn remainder_rhs(&self, other: &KValue) -> runtime::Result<KValue> {
        shape_binary_op!(self, other, modulo)
    }

    // TODO: other ops
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
}
