pub mod allocator;
pub mod cache;
pub mod heap;
#[allow(clippy::module_inception)]
pub(super) mod slab;

/// Returns the index into the slab/cache list for a given layout.
fn alloc_order(layout: core::alloc::Layout) -> Option<usize> {
    // We must take alignemnt into account too.
    let size = core::cmp::max(layout.size(), layout.align());

    let alloc_order = size.next_power_of_two().ilog2() as usize;

    if alloc_order > SLAB_MAX_OBJ_SHIFT as usize {
        return None;
    }

    // Since slabs use a `u16` as the 'next_free' pointer, our minimum order
    // must be 1.
    Some(if alloc_order == 0 { 1 } else { alloc_order })
}
