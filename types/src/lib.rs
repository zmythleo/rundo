extern crate difference;

pub mod primitive_type;
pub mod string_type;

pub mod prelude {
    pub use primitive_type::*;
    pub use string_type::*;
    pub use Rundo;
}

pub const IMPLED_RUNDO: [&str; 15] = [
    "bool", "char", "i8", "u8", "i16", "u16", "i32", "u32", "i64", "u64", "f32", "f64", "isize",
    "usize", "string",
];

/// Every rundo node must implement Rundo trait to support undo/redo.
/// In most of case, you can derive Rundo,
/// of course, you can implement it by yourself.
pub trait Rundo {
    type Op: std::fmt::Debug;
    /// if this node has been changed between from the last step to current.
    fn dirty(&self) -> bool;
    /// Use Op to describe the change infos.
    fn change_op(&self) -> Option<Self::Op>;
    /// Reset the node change state which mean changes has been record by workspace,
    /// or changes will be ignore.
    fn reset(&mut self);
    /// Use an Op and back to the previous data status
    fn back(&mut self, op: &Self::Op);
    /// Go to the next version of the data should be with a Op.
    fn forward(&mut self, op: &Self::Op);
}
