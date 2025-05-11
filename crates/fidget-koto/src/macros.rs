/// Unary operation for KTree
#[macro_export]
macro_rules! unary_op {
    ($self:ident, $op_name:ident) => {{ Ok(KValue::Object(KTree($self.inner().$op_name()).into())) }};
}

/// Unary operation for Koto Shape objects
#[macro_export]
macro_rules! shape_unary_op {
    ($self:ident, $op_name:ident) => {{
        let self_tree = Tree::from($self.inner());
        Ok(KValue::Object(KTree::from(self_tree.$op_name()).into()))
    }};
}

/// Binary operation for KTree
#[macro_export]
macro_rules! binary_op {
    ($self:ident, $other:expr, $op_name:ident) => {{
        match $other {
            KValue::Object(other) => match crate::utils::maybe_tree(other) {
                Some(other) => Ok(KValue::Object(KTree($self.inner().$op_name(other)).into())),
                _ => unexpected_type("Object or Number", $other),
            },
            KValue::Number(num) => {
                let other = Tree::constant(f64::from(num));
                Ok(KValue::Object(KTree($self.inner().$op_name(other)).into()))
            }
            unexpected => unexpected_type("Object or Number", unexpected),
        }
    }};
}

/// Binary operation for Koto Shape
#[macro_export]
macro_rules! shape_binary_op {
    ($self:ident, $other:expr, $op_name:ident) => {{
        let self_tree = Tree::from($self.inner());
        match $other {
            KValue::Object(other) => match crate::utils::maybe_tree(other) {
                Some(other) => Ok(KValue::Object(
                    KTree::from(self_tree.$op_name(other)).into(),
                )),
                _ => unexpected_type("Object or Number", $other),
            },
            KValue::Number(num) => {
                let other = Tree::constant(f64::from(num));
                Ok(KValue::Object(
                    KTree::from(self_tree.$op_name(other)).into(),
                ))
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

/// Binary RHS operation for Koto shapes
#[macro_export]
macro_rules! shape_binary_op_rhs {
    ($self:ident, $other:expr, $op_name:ident) => {{
        let self_tree = Tree::from($self.inner());
        match $other {
            KValue::Number(num) => {
                let other = Tree::constant(f64::from(num));
                Ok(KValue::Object(
                    KTree::from(other.$op_name(self_tree)).into(),
                ))
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
            KValue::Object(other) => match crate::utils::maybe_tree(other) {
                Some(other) => {
                    $self.0 = $self.inner().$op_name(other);
                    Ok(())
                }
                _ => unexpected_type("Object or Number", $other),
            },
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
        let arg = &$ctx.args[0];
        match arg {
            KValue::Object(obj) => match crate::utils::maybe_tree(obj) {
                Some(tree) => {
                    let result = lhs_tree.$name(tree);
                    Ok(KValue::Object(Self(result).into()))
                }
                _ => unexpected_type("Object or Number", arg),
            },
            KValue::Number(num) => {
                let tree = lhs_tree.$name(Tree::constant(f64::from(num)));
                Ok(KValue::Object(Self(tree).into()))
            }
            unexpected => unexpected_type("|Object or Number}|", unexpected),
        }
    }};
}
