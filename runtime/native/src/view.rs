use std::sync::Arc;

use value::Value;

/// A view provides a convenient mechanism for peeking inside a value. A view
/// often less compact than the corresponding value, but only ever allocated on
/// the stack in the runtime system.
pub enum View<'a> {
    /// A generic view peeks inside a value with a generic layout.
    Generic{
        pointers: &'a [Value<'a>],
        auxiliary: &'a [u8],
    },

    /// A buffer view peeks inside a value with the buffer layout.
    Buffer(&'a Arc<[u8]>),
}
