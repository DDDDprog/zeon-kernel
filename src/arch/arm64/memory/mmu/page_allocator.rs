/*
 *  ███████╗███████╗ ██████╗ ███╗   ██╗
 *  ╚══███╔╝██╔════╝██╔═══██╗████╗  ██║
 *    ███╔╝ █████╗  ██║   ██║██╔██╗ ██║
 *   ███╔╝  ██╔══╝  ██║   ██║██║╚██╗██║
 *  ███████╗███████╗╚██████╔╝██║ ╚████║
 *  ╚══════╝╚══════╝ ╚═════╝ ╚═╝  ╚═══╝
 *
 * Zeon - Pure Rust Operating System
 * https://github.com/DDDDprog/zeon-kernel
 */

pub struct PageTableAllocator<'a> {
    data: PhantomData<&'a u8>,
}

impl PageTableAllocator<'_> {
    pub fn new() -> Self {
        Self { data: PhantomData }
    }
}

impl PageAllocator for PageTableAllocator<'_> {
    fn allocate_page_table<T: PgTable>(&mut self) -> Result<TPA<PgTableArray<T>>> {
        let pg = ClaimedPage::alloc_zeroed()?;

        Ok(pg.leak().pa().cast())
    }
}
