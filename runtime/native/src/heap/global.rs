use std::mem;

use heap::Heap;
use heap::Soil;
use layout::Layout;
use value::Value;

extern "C" {
    fn malloc(size: usize) -> *mut u8;
}

/// The global heap allocates values but never frees them. It is used for
/// initializers of top-level definitions.
pub struct GlobalHeap;

impl Heap for GlobalHeap {
    unsafe fn allocate(&self, layout: &'static Layout, _soil: &mut Soil) -> Value {
        let memory = malloc(layout.size());
        *mem::transmute::<*mut u8, *mut &'static Layout>(memory) = layout;
        Value::from_raw(memory.add(8))
    }
}
