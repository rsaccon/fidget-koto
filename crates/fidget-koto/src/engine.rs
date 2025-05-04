use koto::{prelude::*, runtime};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use fidget::context::Tree;

use super::{DrawShape, ScriptContext, Sphere, TreeObject};

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
                    if obj.is_a::<TreeObject>() {
                        let koto_tree = obj.cast::<TreeObject>();
                        context_clone.lock().unwrap().shapes.push(DrawShape {
                            tree: koto_tree.unwrap().inner(),
                            color_rgb: [u8::MAX; 3],
                        });
                        Ok(KValue::Null)
                    } else if obj.is_a::<Sphere>() {
                        let koto_sphere = obj.cast::<Sphere>();
                        let fidget_sphere = koto_sphere.unwrap().inner();
                        let fidget_tree = Tree::from(fidget_sphere);
                        context_clone.lock().unwrap().shapes.push(DrawShape {
                            tree: fidget_tree,
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
                    if obj.is_a::<TreeObject>() {
                        let koto_tree = obj.cast::<TreeObject>();
                        let f = |a| {
                            let a = f64::from(a);
                            if a < 0.0 {
                                0
                            } else if a > 1.0 {
                                255
                            } else {
                                (a * 255.0) as u8
                            }
                        };
                        context_clone.lock().unwrap().shapes.push(DrawShape {
                            tree: koto_tree.unwrap().inner(),
                            color_rgb: [f(r), f(g), f(b)],
                        });
                        Ok(KValue::Null)
                    } else if obj.is_a::<Sphere>() {
                        let koto_sphere = obj.cast::<Sphere>();
                        let fidget_sphere = koto_sphere.unwrap().inner();
                        let fidget_tree = Tree::from(fidget_sphere);
                        let f = |a| {
                            let a = f64::from(a);
                            if a < 0.0 {
                                0
                            } else if a > 1.0 {
                                255
                            } else {
                                (a * 255.0) as u8
                            }
                        };
                        context_clone.lock().unwrap().shapes.push(DrawShape {
                            tree: fidget_tree,
                            color_rgb: [f(r), f(g), f(b)],
                        });
                        Ok(KValue::Null)
                    } else {
                        unexpected_args("|Tree|", &args)
                    }
                }
                unexpected => unexpected_args("|Tree|", &unexpected),
            }
        });

        prelude.add_fn("_circle", move |ctx| {
            let _args = ctx.args();
            Ok(KValue::Null)
        });

        prelude.add_fn("sphere", move |ctx| {
            let args = ctx.args();
            match args {
                [KValue::Number(radius)] => {
                    let sphere = Sphere::new(
                        f64::from(radius),
                        f64::from(0.0),
                        f64::from(0.0),
                        f64::from(0.0),
                    );
                    Ok(KValue::Object(KObject::from(sphere)))
                }
                [
                    KValue::Number(radius),
                    KValue::Number(cx),
                    KValue::Number(cy),
                    KValue::Number(cz),
                ] => {
                    let sphere = Sphere::new(
                        f64::from(radius),
                        f64::from(cx),
                        f64::from(cy),
                        f64::from(cz),
                    );
                    Ok(KValue::Object(KObject::from(sphere)))
                }
                unexpected => unexpected_args("|Sphere|", &unexpected),
            }
        });

        prelude.add_fn("_move", move |ctx| {
            // export move = |shape, dx, dy, dz = 0.0|
            //   ax, ay, az = axes()
            //   shape.remap_xyz ax - dx, ay - dy, az - dz
            let _args = ctx.args();
            Ok(KValue::Null)
        });

        prelude.add_fn("_scale", move |ctx| {
            let _args = ctx.args();
            Ok(KValue::Null)
        });

        prelude.add_fn("_union", move |ctx| {
            let _args = ctx.args();
            Ok(KValue::Null)
        });

        prelude.add_fn("_intersection", move |ctx| {
            let _args = ctx.args();
            Ok(KValue::Null)
        });

        prelude.add_fn("_inverse", move |ctx| {
            let _args = ctx.args();
            Ok(KValue::Null)
        });

        prelude.add_fn("_difference", move |ctx| {
            let _args = ctx.args();
            Ok(KValue::Null)
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

        self.engine.prelude().insert("x", TreeObject::x());
        self.engine.prelude().insert("y", TreeObject::y());
        self.engine.prelude().insert("z", TreeObject::z());

        let core_script = include_str!("core.koto");
        if let Err(err) = self.engine.compile_and_run(core_script) {
            return Err(err);
        }

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
    pub fn eval(&mut self, script: &str) -> Result<Tree, fidget::Error> {
        self.engine.prelude().insert("x", TreeObject::x());
        self.engine.prelude().insert("y", TreeObject::y());
        self.engine.prelude().insert("z", TreeObject::z());

        match self.engine.compile_and_run(script) {
            Ok(KValue::Object(obj)) if obj.is_a::<TreeObject>() => {
                let koto_tree = obj.cast::<TreeObject>();
                let tree = koto_tree.unwrap().inner();
                Ok(tree)
            }
            Ok(_) => Err(fidget::Error::BadNode),
            Err(_) => Err(fidget::Error::BadNode), // TODO: koto compilation error
        }
    }
}

/// Koto axes doc: TODO
fn axes(_ctx: &mut CallContext) -> runtime::Result<KValue> {
    let (x, y, z) = Tree::axes();
    Ok(KValue::Tuple(KTuple::from(vec![
        KValue::Object(TreeObject::from(x).into()),
        KValue::Object(TreeObject::from(y).into()),
        KValue::Object(TreeObject::from(z).into()),
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
                    KValue::Object(obj) if obj.is_a::<TreeObject>() => {
                        let tree = obj.cast::<TreeObject>()?.inner();
                        let result = tree.$name();
                        Ok(TreeObject::from(result).into())
                    }
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
                    (KValue::Object(obj_a), KValue::Object(obj_b))
                        if obj_a.is_a::<TreeObject>() && obj_b.is_a::<TreeObject>() =>
                    {
                        let tree_a = obj_a.cast::<TreeObject>()?.inner();
                        let tree_b = obj_b.cast::<TreeObject>()?.inner();
                        let result = tree_a.$name(tree_b);
                        Ok(TreeObject::from(result).into())
                    }
                    (KValue::Object(obj), KValue::Number(num)) if obj.is_a::<TreeObject>() => {
                        let tree_a = obj.cast::<TreeObject>()?.inner();
                        let tree_b = Tree::constant(f64::from(num));
                        let result = tree_a.$name(tree_b);
                        Ok(TreeObject::from(result).into())
                    }
                    (KValue::Number(num), KValue::Object(obj)) if obj.is_a::<TreeObject>() => {
                        let tree_a = Tree::constant(f64::from(num));
                        let tree_b = obj.cast::<TreeObject>()?.inner();
                        let result = tree_a.$name(tree_b);
                        Ok(TreeObject::from(result).into())
                    }
                    (KValue::Number(num1), KValue::Number(num2)) => {
                        let tree_a = Tree::constant(f64::from(num1));
                        let tree_b = Tree::constant(f64::from(num2));
                        let result = tree_a.$name(tree_b);
                        Ok(TreeObject::from(result).into())
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
