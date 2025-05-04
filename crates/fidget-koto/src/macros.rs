/// Binary operation for KTree
#[macro_export]
macro_rules! binary_op {
    ($self:ident, $other:expr, $op_name:ident) => {{
        match $other {
            KValue::Object(other) if other.is_a::<Self>() => {
                let other = other.cast::<Self>().unwrap().inner();
                Ok(KValue::Object(Self($self.inner().$op_name(other)).into()))
            }
            KValue::Number(num) => {
                let other = Tree::constant(f64::from(num));
                Ok(KValue::Object(Self($self.inner().$op_name(other)).into()))
            }
            unexpected => unexpected_type("Object or Number", unexpected),
        }
    }};
}

/// Binary RHS operation for KTree
#[macro_export]
macro_rules! binary_op_rhs {
    ($self:ident, $other:expr, $op_name:ident) => {{
        match $other {
            KValue::Number(num) => {
                let other = Tree::constant(f64::from(num));
                Ok(KValue::Object(Self(other.$op_name($self.inner())).into()))
            }
            unexpected => unexpected_type("Object or Number", unexpected),
        }
    }};
}

/// Compound operation for KTree
#[macro_export]
macro_rules! compound_assign_op {
    ($self:ident, $other:expr, $op_name:ident) => {{
        match $other {
            KValue::Object(other) if other.is_a::<Self>() => {
                let other = other.cast::<Self>().unwrap().inner();
                $self.0 = $self.inner().$op_name(other);
                Ok(())
            }
            KValue::Number(num) => {
                let other = Tree::constant(f64::from(num));
                $self.0 = $self.inner().$op_name(other);
                Ok(())
            }
            unexpected => unexpected_type("Object or Number", unexpected),
        }
    }};
}

/// Binary function for KTree
#[macro_export]
macro_rules! binary_fn {
    ($ctx:ident, $name:ident) => {{
        let args = $ctx.args;
        if args.len() != 1 {
            return unexpected_args("1 argument: |Object|", args);
        }
        let lhs_tree = $ctx.instance().unwrap().inner();
        match &$ctx.args[0] {
            KValue::Object(obj) if obj.is_a::<Self>() => {
                let koto_tree = obj.cast::<Self>();
                let tree = koto_tree.unwrap().inner();
                let result = lhs_tree.$name(tree);
                Ok(KValue::Object(Self(result).into()))
            }
            KValue::Number(num) => {
                let tree = lhs_tree.$name(Tree::constant(f64::from(num)));
                Ok(KValue::Object(Self(tree).into()))
            }
            unexpected => unexpected_type("|Object or Number}|", unexpected),
        }
    }};
}
