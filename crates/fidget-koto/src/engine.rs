use koto::prelude::*;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use fidget::{Error, context::Tree};

use super::{DrawShape, ScriptContext, TreeObject};

const SHAPES_KEY: &str = "__fidget_shapes__";

/// Engine for evaluating a Koto script with Fidget-specific bindings
pub struct Engine {
    engine: Koto,
    context: Arc<Mutex<ScriptContext>>,
}

impl Default for Engine {
    fn default() -> Self {
        Self::new(Duration::from_secs(1))
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
    pub fn new(execution_limit: Duration) -> Self {
        let koto = Koto::with_settings(
            KotoSettings::default()
                .with_execution_limit(execution_limit)
                .with_module_imported_callback({
                    move |path| println!("TODO: handle import: {:?}", path)
                }),
        );

        koto.prelude().remove("io");
        koto.prelude().remove("koto");
        koto.prelude().remove("os");
        koto.prelude().remove("test");

        koto.prelude().insert("x", TreeObject::x());
        koto.prelude().insert("y", TreeObject::y());
        koto.prelude().insert("z", TreeObject::z());

        koto.prelude().add_fn("axes", move |_ctx| {
            let (x, y, z) = Tree::axes();
            Ok(KValue::Tuple(KTuple::from(vec![
                KValue::Object(TreeObject::from(x).into()),
                KValue::Object(TreeObject::from(y).into()),
                KValue::Object(TreeObject::from(z).into()),
            ])))
        });

        koto.exports()
            .data_mut()
            .insert(ValueKey::from(SHAPES_KEY), KValue::List(KList::default()));
        koto.prelude().insert("fidget", make_fidget_module());

        let context = Arc::new(Mutex::new(ScriptContext::new()));

        let mut out = Self {
            engine: koto,
            context,
        };
        out.add_core_fns();
        out
    }

    /// Add core functions
    pub fn add_core_fns(&mut self) {
        self.engine.prelude().add_fn("draw", move |ctx| {
            let args = ctx.args();
            match args {
                [KValue::Object(obj)] => {
                    if obj.is_a::<TreeObject>() {
                        if let Some(list) = ctx.vm.exports().data_mut().get_mut(SHAPES_KEY) {
                            if let KValue::List(list) = list {
                                list.data_mut().push(KValue::Object(obj.clone()));
                            }
                        }
                        Ok(KValue::Null)
                    } else {
                        unexpected_args("|Tree|", &args)
                    }
                }
                unexpected => unexpected_args("|Tree|", &unexpected),
            }
        });
    }

    /// Executes a full script
    pub fn run(&mut self, script: &str) -> Result<ScriptContext, Error> {
        self.context.lock().unwrap().clear();

        let core_script = include_str!("core.koto");
        if let Err(_) = self.engine.compile_and_run(core_script) {
            return Err(Error::BadNode);
        }

        match self.engine.compile_and_run(script) {
            Ok(_) => {
                if let Some(list) = self.engine.exports().data_mut().get_mut(SHAPES_KEY) {
                    if let KValue::List(list) = list {
                        for val in list.data().iter() {
                            match val {
                                KValue::Object(obj) if obj.is_a::<TreeObject>() => {
                                    let koto_tree = obj.cast::<TreeObject>();
                                    let tree = koto_tree.unwrap().inner();
                                    self.context.lock().unwrap().shapes.push(DrawShape {
                                        tree,
                                        color_rgb: [u8::MAX; 3],
                                    })
                                }
                                KValue::Tuple(tuple) => {
                                    let mut shape_tree = None;
                                    let mut color_rgb = [u8::MAX; 3];
                                    let f = |a| {
                                        if a < 0.0 {
                                            0
                                        } else if a > 1.0 {
                                            255
                                        } else {
                                            (a * 255.0) as u8
                                        }
                                    };
                                    for (i, val) in tuple.data().iter().enumerate() {
                                        if i == 0 {
                                            match val {
                                                KValue::Object(obj) if obj.is_a::<TreeObject>() => {
                                                    let koto_tree = obj.cast::<TreeObject>();
                                                    shape_tree = Some(koto_tree.unwrap().inner());
                                                }
                                                _ => (),
                                            }
                                        }
                                        if i >= 1 && i <= 3 {
                                            match val {
                                                KValue::Number(num) => {
                                                    color_rgb[i - 1] = f(f64::from(num))
                                                }
                                                _ => (),
                                            }
                                        }
                                    }
                                    if let Some(tree) = shape_tree {
                                        self.context
                                            .lock()
                                            .unwrap()
                                            .shapes
                                            .push(DrawShape { tree, color_rgb })
                                    }
                                }
                                _ => (),
                            }
                        }
                        list.data_mut().clear();
                    }
                }
            }
            Err(err) => println!("compile error:{}", err),
        }

        // Steal the ScriptContext's contents
        let mut lock = self.context.lock().unwrap();
        Ok(std::mem::take(&mut lock))
    }

    /// Evaluates a single expression, in terms of `x`, `y`, and `z`
    pub fn eval(&mut self, script: &str) -> Result<Tree, Error> {
        match self.engine.compile_and_run(script) {
            Ok(KValue::Object(obj)) if obj.is_a::<TreeObject>() => {
                let koto_tree = obj.cast::<TreeObject>();
                let tree = koto_tree.unwrap().inner();
                Ok(tree)
            }
            Ok(_) => Err(Error::BadNode),
            Err(_) => Err(Error::BadNode),
        }
    }
}

fn make_fidget_module() -> KMap {
    let module = KMap::with_type("fidget");

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

    module
}
