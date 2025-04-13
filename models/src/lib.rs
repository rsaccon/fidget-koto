//! Koto bindings to Fidget

use fidget::context::Tree;

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
