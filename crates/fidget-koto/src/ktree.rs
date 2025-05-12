use fidget::context::Tree;
use koto::{derive::*, prelude::*, runtime};
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

const BAN_MSG: &str = "cannot compare Tree types during function tracing";

type Inner = Tree;

/// KotoObject wrapper for fidget Tree
#[derive(Clone, KotoCopy, KotoType)]
#[koto(type_name = "Tree")]
pub struct KTree(Inner);

impl KotoObject for KTree {
    fn display(&self, ctx: &mut DisplayContext) -> runtime::Result<()> {
        ctx.append(self.to_string());
        Ok(())
    }

    fn negate(&self) -> runtime::Result<KValue> {
        unary_op!(self, neg)
    }

    fn add(&self, other: &KValue) -> runtime::Result<KValue> {
        binary_op!(self, other, add)
    }

    fn add_rhs(&self, other: &KValue) -> runtime::Result<KValue> {
        binary_op_rhs!(self, other, add)
    }

    fn subtract(&self, other: &KValue) -> runtime::Result<KValue> {
        binary_op!(self, other, sub)
    }

    fn subtract_rhs(&self, other: &KValue) -> runtime::Result<KValue> {
        binary_op_rhs!(self, other, sub)
    }

    fn multiply(&self, other: &KValue) -> runtime::Result<KValue> {
        binary_op!(self, other, mul)
    }

    fn multiply_rhs(&self, other: &KValue) -> runtime::Result<KValue> {
        binary_op_rhs!(self, other, mul)
    }

    fn divide(&self, other: &KValue) -> runtime::Result<KValue> {
        binary_op!(self, other, div)
    }

    fn divide_rhs(&self, other: &KValue) -> runtime::Result<KValue> {
        binary_op_rhs!(self, other, div)
    }

    fn remainder(&self, other: &KValue) -> runtime::Result<KValue> {
        binary_op!(self, other, modulo)
    }

    fn remainder_rhs(&self, other: &KValue) -> runtime::Result<KValue> {
        binary_op!(self, other, modulo)
    }

    fn power(&self, other: &KValue) -> runtime::Result<KValue> {
        match other {
            KValue::Number(num) => {
                let other = i64::from(num);
                Ok(KValue::Object(Self(self.inner().pow(other)).into()))
            }
            unexpected => unexpected_type("Number", unexpected),
        }
    }

    fn add_assign(&mut self, other: &KValue) -> runtime::Result<()> {
        compound_assign_op!(self, other, add)
    }

    fn subtract_assign(&mut self, other: &KValue) -> runtime::Result<()> {
        compound_assign_op!(self, other, sub)
    }

    fn multiply_assign(&mut self, other: &KValue) -> runtime::Result<()> {
        compound_assign_op!(self, other, mul)
    }

    fn divide_assign(&mut self, other: &KValue) -> runtime::Result<()> {
        compound_assign_op!(self, other, div)
    }

    fn remainder_assign(&mut self, other: &KValue) -> runtime::Result<()> {
        compound_assign_op!(self, other, modulo)
    }

    fn power_assign(&mut self, other: &KValue) -> runtime::Result<()> {
        match other {
            KValue::Number(num) => {
                let other = i64::from(num);
                self.0 = self.inner().pow(other);
                Ok(())
            }
            unexpected => unexpected_type("Object or Number", unexpected),
        }
    }

    fn less(&self, other: &KValue) -> runtime::Result<bool> {
        unexpected_type(BAN_MSG, other)
    }

    fn less_or_equal(&self, other: &KValue) -> runtime::Result<bool> {
        unexpected_type(BAN_MSG, other)
    }

    fn greater(&self, other: &KValue) -> runtime::Result<bool> {
        unexpected_type(BAN_MSG, other)
    }

    fn greater_or_equal(&self, other: &KValue) -> runtime::Result<bool> {
        unexpected_type(BAN_MSG, other)
    }

    fn equal(&self, other: &KValue) -> runtime::Result<bool> {
        unexpected_type(BAN_MSG, other)
    }

    fn not_equal(&self, other: &KValue) -> runtime::Result<bool> {
        unexpected_type(BAN_MSG, other)
    }
}

impl From<Inner> for KTree {
    fn from(tree: Inner) -> Self {
        Self(tree)
    }
}

impl From<KTree> for KValue {
    fn from(obj: KTree) -> Self {
        KObject::from(obj).into()
    }
}

impl fmt::Display for KTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tree{{}}")
    }
}

#[koto_impl]
impl KTree {
    /// Create KotoObject representing Tree::x()
    pub fn x() -> KObject {
        KObject::from(Self(Inner::x().into()))
    }

    /// Create KotoObject representing Tree::y()
    pub fn y() -> KObject {
        KObject::from(Self(Inner::y().into()))
    }

    /// Create KotoObject representing Tree::z()
    pub fn z() -> KObject {
        KObject::from(Self(Inner::z().into()))
    }

    /// Access the inner fidget Tree struct
    pub fn inner(&self) -> Inner {
        self.to_owned().0
    }

    #[koto_method]
    fn remap_xyz(ctx: MethodContext<Self>) -> runtime::Result<KValue> {
        let args = ctx.args;
        if args.len() != 3 {
            return unexpected_args("3 arguments: |x, y, z|", args);
        }
        match args {
            [
                KValue::Object(obj_x),
                KValue::Object(obj_y),
                KValue::Object(obj_z),
            ] => {
                if obj_x.is_a::<Self>() && obj_y.is_a::<Self>() && obj_z.is_a::<Self>() {
                    let tree = ctx.instance().unwrap().inner();
                    let x = obj_x.cast::<Self>()?.inner();
                    let y = obj_y.cast::<Self>()?.inner();
                    let z = obj_z.cast::<Self>()?.inner();
                    let tree = tree.remap_xyz(x, y, z);
                    Ok(KObject::from(Self(tree.into())).into())
                } else {
                    unexpected_args("|x, y, z|", args)
                }
            }
            _ => unexpected_args("|x, y, z|", args),
        }
    }

    #[koto_method]
    fn min(ctx: MethodContext<Self>) -> runtime::Result<KValue> {
        binary_fn!(ctx, min)
    }

    #[koto_method]
    fn max(ctx: MethodContext<Self>) -> runtime::Result<KValue> {
        binary_fn!(ctx, max)
    }

    #[koto_method]
    fn compare(ctx: MethodContext<Self>) -> runtime::Result<KValue> {
        binary_fn!(ctx, compare)
    }

    #[koto_method]
    fn and(ctx: MethodContext<Self>) -> runtime::Result<KValue> {
        binary_fn!(ctx, and)
    }

    #[koto_method]
    fn or(ctx: MethodContext<Self>) -> runtime::Result<KValue> {
        binary_fn!(ctx, or)
    }

    #[koto_method]
    fn atan2(ctx: MethodContext<Self>) -> runtime::Result<KValue> {
        binary_fn!(ctx, atan2)
    }

    #[koto_method]
    fn abs(&self) -> runtime::Result<KValue> {
        Ok(KValue::Object(Self(self.0.clone().abs()).into()))
    }
    #[koto_method]
    fn square(&self) -> runtime::Result<KValue> {
        Ok(KValue::Object(Self(self.0.clone().square()).into()))
    }

    #[koto_method]
    fn sqrt(&self) -> runtime::Result<KValue> {
        Ok(KValue::Object(Self(self.0.clone().sqrt()).into()))
    }

    #[koto_method]
    fn sin(&self) -> runtime::Result<KValue> {
        Ok(KValue::Object(Self(self.0.clone().sin()).into()))
    }

    #[koto_method]
    fn cos(&self) -> runtime::Result<KValue> {
        Ok(KValue::Object(Self(self.0.clone().cos()).into()))
    }

    #[koto_method]
    fn tan(&self) -> runtime::Result<KValue> {
        Ok(KValue::Object(Self(self.0.clone().tan()).into()))
    }

    #[koto_method]
    fn asin(&self) -> runtime::Result<KValue> {
        Ok(KValue::Object(Self(self.0.clone().asin()).into()))
    }

    #[koto_method]
    fn acos(&self) -> runtime::Result<KValue> {
        Ok(KValue::Object(Self(self.0.clone().acos()).into()))
    }

    #[koto_method]
    fn atan(&self) -> runtime::Result<KValue> {
        let result = self.0.clone().atan();
        Ok(KValue::Object(Self(result).into()))
    }

    #[koto_method]
    fn exp(&self) -> runtime::Result<KValue> {
        let result = self.0.clone().exp();
        Ok(KValue::Object(Self(result).into()))
    }

    #[koto_method]
    fn ln(&self) -> runtime::Result<KValue> {
        Ok(KValue::Object(Self(self.0.clone().ln()).into()))
    }

    #[koto_method]
    fn not(&self) -> runtime::Result<KValue> {
        Ok(KValue::Object(Self(self.0.clone().not()).into()))
    }

    #[koto_method]
    fn ceil(&self) -> runtime::Result<KValue> {
        Ok(KValue::Object(Self(self.0.clone().ceil()).into()))
    }

    #[koto_method]
    fn floor(&self) -> runtime::Result<KValue> {
        Ok(KValue::Object(Self(self.0.clone().floor()).into()))
    }

    #[koto_method]
    fn round(&self) -> runtime::Result<KValue> {
        Ok(KValue::Object(Self(self.0.clone().round()).into()))
    }

    #[koto_method]
    fn neg(&self) -> runtime::Result<KValue> {
        Ok(KValue::Object(Self(self.0.clone().neg()).into()))
    }
}
