use layout::Layout;
use value::Value;

pub mod global;

/// A heap allows for allocation and garbage collection of values.
pub trait Heap {
    /// Allocate memory for a value, and return the value. This method is
    /// unsafe for two reasons:
    ///
    ///  1. The memory of the value is not initialized.
    ///  2. Garbage collection may be performed, requiring that the given soil
    ///     is coherent, and that all values in the soil belong to this heap.
    unsafe fn allocate(&self, layout: &'static Layout, soil: &mut Soil) -> Value;
}

/// A soil is a collection of roots.
pub type Soil<'a> = Iterator<Item=Value<'a>>;
