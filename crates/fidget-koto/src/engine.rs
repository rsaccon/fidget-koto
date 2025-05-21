use crate::{ScriptContext, fidget_lib::FidgetLib, utils::maybe_tree};
use fidget::context::Tree;
use koto::prelude::*;
use std::sync::{Arc, Mutex};
use std::time::Duration;

const DEFAULT_EXECUTION_LIMIT: u64 = 1;

/// Engine initialization settings
pub struct EngineSettings {
    default_imports: bool,
    execution_limit: Duration,
}

/// Engine for evaluating a Koto script with Fidget-specific bindings
pub struct Engine {
    pub settings: EngineSettings,
    engine: Koto,
    context: Arc<Mutex<ScriptContext>>,
}

impl Default for Engine {
    fn default() -> Self {
        Self::new(true, DEFAULT_EXECUTION_LIMIT)
    }
}

impl Engine {
    pub fn new_without_default_imports() -> Self {
        Self::new(false, DEFAULT_EXECUTION_LIMIT)
    }

    /// Constructs a script evaluation engine with Fidget bindings
    ///
    /// The context includes a variety of functions that operate on [`Tree`]
    /// handles.
    ///
    /// In addition, it includes everything in [`core.koto`](fidget_koto::core),
    /// which is effectively our standard library.
    pub fn new(default_imports: bool, execution_limit_in_secs: u64) -> Self {
        let settings = EngineSettings {
            default_imports,
            execution_limit: Duration::from_secs(execution_limit_in_secs),
        };
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

        let fidget_lib = FidgetLib {};
        let module = KMap::with_type("fidget");
        let context = Arc::new(Mutex::new(ScriptContext::new()));

        fidget_lib.add_core(&module);
        fidget_lib.add_tree_ops(&module);
        fidget_lib.add_shape_fns(&module);
        fidget_lib.add_draw(&module, context.clone());
        prelude.insert("fidget", module);

        if settings.default_imports {
            fidget_lib.add_core(&prelude);
            fidget_lib.add_tree_ops(&prelude);
            fidget_lib.add_shape_fns(&prelude);
            fidget_lib.add_draw(&prelude, context.clone());
        }

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
