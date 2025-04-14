//! Koto bindings to Fidget
//!
//! There are two main ways to use these bindings.
//!
//! The simplest option is to call [`eval`], which evaluates a single expression
//! with pre-defined variables `x`, `y`, `z`.
//!
//! ```
//! use fidget::{
//!     shape::EzShape,
//!     vm::VmShape,
//! };
//! use fidget_koto::eval;
//!
//! let tree = fidget_koto::eval("x + y")?;
//! let shape = VmShape::from(tree);
//! let mut eval = VmShape::new_point_eval();
//! let tape = shape.ez_point_tape();
//! assert_eq!(eval.eval(&tape, 1.0, 2.0, 0.0)?.0, 3.0);
//! # Ok::<(), fidget::Error>(())
//! ```
//!
//! `eval` returns a single value.  To evaluate a script with multiple outputs,
//! construct an [`Engine`] then call [`Engine::run`]:
//!
//! ```
//! use fidget::{
//!     shape::EzShape,
//!     vm::VmShape
//! };
//! use fidget_koto::Engine;
//!
//! let mut engine = Engine::default();
//! let mut out = engine.run("draw(x + y - 1)")?;
//!
//! assert_eq!(out.shapes.len(), 1);
//! let shape = VmShape::from(out.shapes.pop().unwrap().tree);
//! let mut eval = VmShape::new_point_eval();
//! let tape = shape.ez_point_tape();
//! assert_eq!(eval.eval(&tape, 0.5, 2.0, 0.0)?.0, 1.5);
//! # Ok::<(), fidget::Error>(())
//! ```
//!
//! Within a call to [`Engine::run`], `draw` and `draw_rgb` insert shapes into  // TODO: fix doc
//! [`ScriptContext::shapes`], which is returned after script evaluation is     // TODO: fix doc
//! complete.                                                                   // TODO: fix doc
//!
//! Scripts are evaluated in a Koto context that includes [`core.koto`](core),
//! which defines a few simple shapes and transforms.  `x`, `y`, and `z` are
//! defined in the root scope, and `axes()` returns an object with `x`/`y`/`z`
//! members.

use fidget::{Error, context::Tree};

#[macro_use]
mod macros;

mod engine;
mod tree_object;

pub use engine::Engine;
pub use tree_object::TreeObject;

//////////////////////////////////////////////////////////////////////////////////

/// Shape to render
///
/// Populated by calls to `draw(...)` or `draw_rgb(...)` in a Koto script
pub struct DrawShape {
    /// Tree to render
    pub tree: Tree,
    /// Color to use when drawing the shape
    pub color_rgb: [u8; 3],
}

/// Context for shape evaluation
///
/// This object stores a set of shapes, which is populated by calls to `draw` or
/// `draw_rgb` during script evaluation.
pub struct ScriptContext {
    /// List of shapes populated since the last call to [`clear`](Self::clear)
    pub shapes: Vec<DrawShape>,
}

impl Default for ScriptContext {
    fn default() -> Self {
        Self::new()
    }
}

impl ScriptContext {
    /// Builds a new empty script context
    pub fn new() -> Self {
        Self { shapes: vec![] }
    }
    /// Resets the script context
    pub fn clear(&mut self) {
        self.shapes.clear();
    }
}

////////////////////////////////////////////////////////////////////////////////

/// One-shot evaluation of a single expression, in terms of `x, y, z`
pub fn eval(s: &str) -> Result<Tree, Error> {
    let mut engine = Engine::default();
    engine.eval(s)
}

////////////////////////////////////////////////////////////////////////////////
