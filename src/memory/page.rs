// Zeon - Pure Rust Operating System
// https://github.com/DDDDprog/zeon-kernel

pub struct PgAllocGetter {}

impl PageAllocGetter<ArchImpl> for PgAllocGetter {
    fn global_page_alloc() -> &'static libkernel::memory::allocators::phys::FrameAllocator<ArchImpl>
    {
        PAGE_ALLOC.get().unwrap()
    }
}

pub type ClaimedPage =
    libkernel::memory::page::ClaimedPage<ArchImpl, PgAllocGetter, PageOffsetTranslator>;
