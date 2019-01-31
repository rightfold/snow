use std::mem;
use std::sync::Arc;

/// A layout describes what a value looks like in memory. Each value begins
/// with a pointer to its layout. Knowing the layout of a value at runtime is
/// important for certain heaps, namely those that need to traverse the values
/// for garbage collection.
pub enum Layout {
    /// A generic layout is used for most values. A value with a generic layout
    /// consists of some number of pointers to other values and some amount of
    /// non-pointer auxiliary data.
    Generic{
        pointers: usize,
        auxiliary: usize,
    },

    /// The buffer layout is used for values with shared buffers. A value with
    /// the buffer layout contains just a pointer to the buffer and the size of
    /// the buffer. The buffer itself is on a global heap.
    Buffer,
}

impl Layout {
    /// O(1). Compute the size of a value with this layout. The size includes
    /// the size of the pointer to the layout that the value begins with.
    pub fn size(&self) -> usize {
        match self {
            Layout::Generic{pointers, auxiliary} =>
                8 + pointers * 8 + auxiliary,

            Layout::Buffer =>
                8 + mem::size_of::<Arc<[u8]>>()
        }
    }
}
