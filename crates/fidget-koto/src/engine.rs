use koto::{prelude::*, runtime};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use fidget::context::Tree;

use super::{
    DrawShape, KCircle, KDifference, KIntersection, KInverse, KMove, KScale, KSphere, KTree,
    KUnion, ScriptContext,
};

/// Engine initialization settings
pub struct EngineSettings {
    add_fidget_fns: bool,
    execution_limit: Duration,
}

/// Engine for evaluating a Koto script with Fidget-specific bindings
pub struct Engine {
    settings: EngineSettings,
    engine: Koto,
    context: Arc<Mutex<ScriptContext>>,
}

impl Default for Engine {
    fn default() -> Self {
        Self::new(EngineSettings {
            add_fidget_fns: false,
            execution_limit: Duration::from_secs(1),
        })
    }
}

impl Engine {
    /// Constructs a script evaluation engine with Fidget bindings
    ///
    /// The context includes a variety of functions that operate on [`Tree`]
    /// handles.
    ///
    /// In addition, it includes everything in [`core.koto`](fidget_koto::core),
    /// which is effectively our standard library.
    pub fn new(settings: EngineSettings) -> Self {
        let koto = Koto::with_settings(
            KotoSettings::default()
                .with_execution_limit(settings.execution_limit)
                .with_module_imported_callback({
                    move |path| {
                        println!("module import callback - path: {:?}", path);
                    }
                }),
        );

        let prelude = koto.prelude();
        prelude.remove("io");
        prelude.remove("koto");
        prelude.remove("os");
        prelude.remove("test");

        prelude.insert("axes", axes);

        if settings.add_fidget_fns {
            add_fidget_module_or_fns(&prelude);
        } else {
            let module = KMap::with_type("fidget");
            add_fidget_module_or_fns(&module);
            prelude.insert("fidget", module);
        }

        let context = Arc::new(Mutex::new(ScriptContext::new()));

        let context_clone = context.clone();
        prelude.add_fn("draw", move |ctx| {
            let args = ctx.args();
            match args {
                [KValue::Object(obj)] => {
                    if let Some(tree) = maybe_tree(&obj) {
                        context_clone.lock().unwrap().shapes.push(DrawShape {
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
                        context_clone.lock().unwrap().shapes.push(DrawShape {
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

        prelude.add_fn("circle", move |ctx| {
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

        prelude.add_fn("sphere", move |ctx| {
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

        prelude.add_fn("union", move |ctx| {
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

        prelude.add_fn("intersection", move |ctx| {
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

        prelude.add_fn("difference", move |ctx| {
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

        prelude.add_fn("inverse", move |ctx| {
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

        prelude.add_fn("move", move |ctx| {
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

        prelude.add_fn("scale", move |ctx| {
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

        Self {
            settings,
            engine: koto,
            context,
        }
    }

    /// Executes a full script
    pub fn run(&mut self, script: &str) -> Result<ScriptContext, koto::Error> {
        self.context.lock().unwrap().clear();

        ///////////////////////////////////////////////////////////////////////
        // BEGIN Experiment, everything hardcoded, just for trying out koto modules import
        //
        // Koto:
        //
        // simple_module.koto
        // ------------------
        // export
        //   radius: 0.6
        //
        // simple_main.koto
        // ------------------
        // s1 = sphere simple_module.radius, -0.5, -0.25,  0.0
        // draw s1
        //
        // Rust
        //
        // let simple_module_script = include_str!("../../../models/simple_module.koto");
        // let mut koto_module_loader = Koto::new();
        // if let Err(err) = koto_module_loader.compile_and_run(simple_module_script) {
        //     return Err(err);
        // }
        // if let Some(radius) = koto_module_loader.exports().data_mut().get_mut("radius") {
        //     let module = KMap::with_type("simple_module");
        //     module.insert("radius", radius.clone());
        //     self.engine
        //         .prelude()
        //         .insert("simple_module", module.clone());
        //     println!("added simple_module");
        // } else {
        //     println!("cannot find simple_module");
        // }
        // END Experiment
        ///////////////////////////////////////////////////////////////////////

        self.engine.prelude().insert("x", KTree::x());
        self.engine.prelude().insert("y", KTree::y());
        self.engine.prelude().insert("z", KTree::z());

        if self.settings.add_fidget_fns {}

        match self.engine.compile_and_run(script) {
            Ok(_) => (),
            Err(err) => {
                return Err(err);
            }
        }

        // Steal the ScriptContext's contents
        let mut lock = self.context.lock().unwrap();
        Ok(std::mem::take(&mut lock))
    }

    /// Evaluates a single expression, in terms of `x`, `y`, and `z`
    pub fn eval(&mut self, script: &str) -> Result<Tree, koto::Error> {
        self.engine.prelude().insert("x", KTree::x());
        self.engine.prelude().insert("y", KTree::y());
        self.engine.prelude().insert("z", KTree::z());

        match self.engine.compile_and_run(script) {
            Ok(KValue::Object(obj)) => match maybe_tree(&obj) {
                Some(tree) => Ok(tree),
                _ => Err(koto::Error::from(koto::runtime::Error::new(
                    // koto::runtime::ErrorKind::UnexpectedError(unexpected_type("Tree", unexpected)),
                    // TODO: try to use ErrorKind::UnexpectedType
                    koto::runtime::ErrorKind::UnexpectedError,
                ))),
                // _ => Err(fidget::Error::BadNode),
            },
            Ok(_) => Err(koto::Error::from(koto::runtime::Error::new(
                // koto::runtime::ErrorKind::UnexpectedError(unexpected_type("Tree", unexpected)),
                // TODO: try to use ErrorKind::UnexpectedType
                koto::runtime::ErrorKind::UnexpectedError,
            ))),
            Err(err) => Err(err),
        }
    }
}

/// Koto axes doc: TODO
fn axes(_ctx: &mut CallContext) -> runtime::Result<KValue> {
    let (x, y, z) = Tree::axes();
    Ok(KValue::Tuple(KTuple::from(vec![
        KValue::Object(KTree::from(x).into()),
        KValue::Object(KTree::from(y).into()),
        KValue::Object(KTree::from(z).into()),
    ])))
}

fn add_fidget_module_or_fns(module: &KMap) {
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

pub fn maybe_tree(obj: &KObject) -> Option<Tree> {
    if obj.is_a::<KTree>() {
        let k_tree = obj.cast::<KTree>();
        Some(k_tree.unwrap().inner())
    } else if obj.is_a::<KCircle>() {
        let k_tree = obj.cast::<KCircle>();
        Some(Tree::from(k_tree.unwrap().inner()))
    } else if obj.is_a::<KSphere>() {
        let k_tree = obj.cast::<KSphere>();
        Some(Tree::from(k_tree.unwrap().inner()))
    } else if obj.is_a::<KUnion>() {
        let k_tree = obj.cast::<KUnion>();
        Some(Tree::from(k_tree.unwrap().inner()))
    } else if obj.is_a::<KIntersection>() {
        let k_tree = obj.cast::<KIntersection>();
        Some(Tree::from(k_tree.unwrap().inner()))
    } else if obj.is_a::<KDifference>() {
        let k_tree = obj.cast::<KDifference>();
        Some(Tree::from(k_tree.unwrap().inner()))
    } else if obj.is_a::<KInverse>() {
        let k_tree = obj.cast::<KInverse>();
        Some(Tree::from(k_tree.unwrap().inner()))
    } else if obj.is_a::<KMove>() {
        let k_tree = obj.cast::<KMove>();
        Some(Tree::from(k_tree.unwrap().inner()))
    } else if obj.is_a::<KScale>() {
        let k_tree = obj.cast::<KScale>();
        Some(Tree::from(k_tree.unwrap().inner()))
    } else {
        None
    }
}
