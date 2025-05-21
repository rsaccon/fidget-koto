use std::sync::Arc;
use std::sync::Mutex;

use crate::{
    DrawShape, KCircle, KDifference, KIntersection, KInverse, KMove, KScale, KSphere, KTree,
    KUnion, ScriptContext, utils::maybe_tree,
};
use fidget::context::Tree;
use koto::prelude::*;

#[derive(Clone)]
#[allow(missing_docs)]
pub struct FidgetLib {}

impl FidgetLib {
    pub fn add_core(&self, module: &KMap) {
        module.insert("x", KTree::x());
        module.insert("y", KTree::y());
        module.insert("z", KTree::z());

        module.add_fn("axes", move |_ctx| {
            let (x, y, z) = Tree::axes();
            Ok(KValue::Tuple(KTuple::from(vec![
                KValue::Object(KTree::from(x).into()),
                KValue::Object(KTree::from(y).into()),
                KValue::Object(KTree::from(z).into()),
            ])))
        });
    }

    pub fn add_tree_ops(&self, module: &KMap) {
        macro_rules! add_unary_fn {
            ($name_string:literal, $name:ident) => {
                module.add_fn($name_string, move |ctx| {
                    let args = ctx.args();
                    if args.len() != 1 {
                        return unexpected_args("1 argument: Tree | Number", args);
                    }
                    match &args[0] {
                        KValue::Object(obj) => match maybe_tree(obj) {
                            Some(tree) => Ok(KTree::from(tree.$name()).into()),
                            _ => unexpected_type("invalid type", &args[0]),
                        },
                        // TODO: check and handle KNumber
                        unexpected => unexpected_type("invalid type", unexpected),
                    }
                });
            };
        }

        macro_rules! add_binary_fn {
            ($name_string:literal, $name:ident) => {
                module.add_fn($name_string, move |ctx| {
                    let args = ctx.args();
                    if args.len() != 2 {
                        return unexpected_args("2 arguments: Tree|Number, Tree|Number", args);
                    }
                    match (&args[0], &args[1]) {
                        (KValue::Object(obj_a), KValue::Object(obj_b)) => {
                            match (maybe_tree(obj_a), maybe_tree(obj_b)) {
                                (Some(tree_a), Some(tree_b)) => {
                                    Ok(KTree::from(tree_a.$name(tree_b)).into())
                                }
                                _ => unexpected_args("Tree, Tree", args),
                            }
                        }
                        (KValue::Object(obj), KValue::Number(num)) => match maybe_tree(obj) {
                            Some(tree_a) => {
                                let tree_b = Tree::constant(f64::from(num));
                                Ok(KTree::from(tree_a.$name(tree_b)).into())
                            }
                            _ => unexpected_args("Tree, Number", args),
                        },
                        (KValue::Number(num), KValue::Object(obj)) => match maybe_tree(obj) {
                            Some(tree_b) => {
                                let tree_a = Tree::constant(f64::from(num));
                                Ok(KTree::from(tree_a.$name(tree_b)).into())
                            }
                            _ => unexpected_args("Number, Tree", args),
                        },
                        (KValue::Number(num1), KValue::Number(num2)) => {
                            let tree_a = Tree::constant(f64::from(num1));
                            let tree_b = Tree::constant(f64::from(num2));
                            Ok(KTree::from(tree_a.$name(tree_b)).into())
                        }
                        _ => unexpected_args("Tree|Number, Tree|Number", args),
                    }
                });
            };
        }

        add_binary_fn!("min", min);
        add_binary_fn!("max", max);
        add_binary_fn!("compare", compare);
        // AND not possible with koto for now because `and` is reserved keyword
        // OR not possible with koto for now because `or` is reserved keyword
        add_binary_fn!("atan2", atan2);

        add_unary_fn!("abs", abs);
        add_unary_fn!("sqrt", sqrt);
        add_unary_fn!("square", square);
        add_unary_fn!("sin", sin);
        add_unary_fn!("cos", cos);
        add_unary_fn!("tan", tan);
        add_unary_fn!("asin", asin);
        add_unary_fn!("acos", acos);
        add_unary_fn!("atan", atan);
        add_unary_fn!("exp", exp);
        add_unary_fn!("ln", ln);
        add_unary_fn!("not", not);
        add_unary_fn!("ceil", ceil);
        add_unary_fn!("floor", floor);
        add_unary_fn!("round", round);
    }

    pub fn add_shape_fns(&self, module: &KMap) {
        module.add_fn("circle", move |ctx| {
            let args = ctx.args();
            match args {
                [KValue::Number(radius)] => {
                    let result = KCircle::new(f64::from(radius), f64::from(0.0), f64::from(0.0));
                    Ok(KValue::Object(KObject::from(result)))
                }
                [KValue::Number(radius), KValue::Number(x), KValue::Number(y)] => {
                    let result = KCircle::new(f64::from(radius), f64::from(x), f64::from(y));
                    Ok(KValue::Object(KObject::from(result)))
                }
                unexpected => unexpected_args("|Circle|", &unexpected),
            }
        });

        module.add_fn("sphere", move |ctx| {
            let args = ctx.args();
            match args {
                [KValue::Number(radius)] => {
                    let result = KSphere::new(
                        f64::from(radius),
                        f64::from(0.0),
                        f64::from(0.0),
                        f64::from(0.0),
                    );
                    Ok(KValue::Object(KObject::from(result)))
                }
                [
                    KValue::Number(radius),
                    KValue::Number(x),
                    KValue::Number(y),
                    KValue::Number(z),
                ] => {
                    let result =
                        KSphere::new(f64::from(radius), f64::from(x), f64::from(y), f64::from(z));
                    Ok(KValue::Object(KObject::from(result)))
                }
                unexpected => unexpected_args("|Sphere|", &unexpected),
            }
        });

        module.add_fn("union", move |ctx| {
            let args = ctx.args();
            match args {
                [KValue::Object(obj_a), KValue::Object(obj_b)] => {
                    if let (Some(tree_a), Some(tree_b)) = (maybe_tree(obj_a), maybe_tree(obj_b)) {
                        let result = KUnion::new(tree_a, tree_b);
                        Ok(KValue::Object(KObject::from(result)))
                    } else {
                        unexpected_args("|Union|", &args)
                    }
                }
                unexpected => unexpected_args("|Union|", &unexpected),
            }
        });

        module.add_fn("intersection", move |ctx| {
            let args = ctx.args();
            match args {
                [KValue::Object(obj_a), KValue::Object(obj_b)] => {
                    if let (Some(tree_a), Some(tree_b)) = (maybe_tree(obj_a), maybe_tree(obj_b)) {
                        let result = KIntersection::new(tree_a, tree_b);
                        Ok(KValue::Object(KObject::from(result)))
                    } else {
                        unexpected_args("|Intersection|", &args)
                    }
                }
                unexpected => unexpected_args("|Intersection|", &unexpected),
            }
        });

        module.add_fn("difference", move |ctx| {
            let args = ctx.args();
            match args {
                [KValue::Object(obj_a), KValue::Object(obj_b)] => {
                    if let (Some(tree_a), Some(tree_b)) = (maybe_tree(obj_a), maybe_tree(obj_b)) {
                        let result = KDifference::new(tree_a, tree_b);
                        Ok(KValue::Object(KObject::from(result)))
                    } else {
                        unexpected_args("|Difference|", &args)
                    }
                }
                unexpected => unexpected_args("|Difference|", &unexpected),
            }
        });

        module.add_fn("inverse", move |ctx| {
            let args = ctx.args();
            match args {
                [KValue::Object(obj)] => {
                    if let Some(tree) = maybe_tree(obj) {
                        let result = KInverse::new(tree);
                        Ok(KValue::Object(KObject::from(result)))
                    } else {
                        unexpected_args("|Inverse|", &args)
                    }
                }
                unexpected => unexpected_args("|Inverse|", &unexpected),
            }
        });

        module.add_fn("move", move |ctx| {
            let args = ctx.args();
            match args {
                [
                    KValue::Object(obj),
                    KValue::Number(x),
                    KValue::Number(y),
                    KValue::Number(z),
                ] => {
                    if let Some(tree) = maybe_tree(obj) {
                        let result = KMove::new(tree, f64::from(x), f64::from(y), f64::from(z));
                        Ok(KValue::Object(KObject::from(result)))
                    } else {
                        unexpected_args("|Move|", &args)
                    }
                }
                unexpected => unexpected_args("|Move|", &unexpected),
            }
        });

        module.add_fn("scale", move |ctx| {
            let args = ctx.args();
            match args {
                [
                    KValue::Object(obj),
                    KValue::Number(x),
                    KValue::Number(y),
                    KValue::Number(z),
                ] => {
                    if let Some(tree) = maybe_tree(obj) {
                        let result = KScale::new(tree, f64::from(x), f64::from(y), f64::from(z));
                        Ok(KValue::Object(KObject::from(result)))
                    } else {
                        unexpected_args("|Scale|", &args)
                    }
                }
                unexpected => unexpected_args("|Scale|", &unexpected),
            }
        });
    }

    pub fn add_draw(&self, module: &KMap, context: Arc<Mutex<ScriptContext>>) {
        module.add_fn("draw", move |ctx| {
            let args = ctx.args();
            match args {
                [KValue::Object(obj)] => {
                    if let Some(tree) = maybe_tree(&obj) {
                        context.lock().unwrap().shapes.push(DrawShape {
                            tree,
                            color_rgb: [u8::MAX; 3],
                        });
                        Ok(KValue::Null)
                    } else {
                        unexpected_args("|Tree|", &args)
                    }
                }
                [
                    KValue::Object(obj),
                    KValue::Number(r),
                    KValue::Number(g),
                    KValue::Number(b),
                ] => {
                    if let Some(tree) = maybe_tree(&obj) {
                        context.lock().unwrap().shapes.push(DrawShape {
                            tree,
                            color_rgb: [to_u8(r), to_u8(g), to_u8(b)],
                        });
                        Ok(KValue::Null)
                    } else {
                        unexpected_args("|Tree|", &args)
                    }
                }
                unexpected => unexpected_args("|Tree|", &unexpected),
            }
        });
    }
}

fn to_u8(number: &KNumber) -> u8 {
    let number = f64::from(number);
    if number < 0.0 {
        0
    } else if number > 1.0 {
        255
    } else {
        (number * 255.0) as u8
    }
}
