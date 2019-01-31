use std::marker::PhantomData;
use std::mem;
use std::slice;
use std::sync::Arc;

use layout::Layout;
use view::View;

/// A value points to some data on a heap. Prior to the pointee is a pointer to
/// the layout of the value.
pub struct Value<'a> {
    p: *const u8,
    phantom: PhantomData<&'a ()>,
}

impl<'a> Value<'a> {
    /// O(1). Create a value from a pointer to its data.
    pub unsafe fn from_raw(p: *const u8) -> Self {
        Value{p, phantom: PhantomData}
    }

    /// O(1). Find the layout of this value.
    pub fn layout(&self) -> &'static Layout {
        unsafe { *mem::transmute::<*const u8, *const _>(self.p.offset(-8)) }
    }

    /// O(1). Construct a view of this value.
    pub fn view(&self) -> View {
        match self.layout() {
            Layout::Generic{pointers, auxiliary} =>
                unsafe { self.generic_view(*pointers, *auxiliary) },

            Layout::Buffer =>
                unsafe { self.buffer_view() },
        }
    }

    unsafe fn generic_view(&self, pointers: usize, auxiliary: usize) -> View {
        let raw_pointers: *const Value = mem::transmute(self.p);
        let raw_auxiliary = self.p.add(8 * pointers);
        View::Generic{
            pointers: slice::from_raw_parts(raw_pointers, pointers),
            auxiliary: slice::from_raw_parts(raw_auxiliary, auxiliary),
        }
    }

    unsafe fn buffer_view(&self) -> View {
        let raw: *const Arc<[u8]> = mem::transmute(self.p);
        View::Buffer(&*raw)
    }
}
