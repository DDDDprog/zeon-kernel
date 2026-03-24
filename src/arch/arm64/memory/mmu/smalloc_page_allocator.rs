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

// Zeon - Pure Rust Operating System
// https://github.com/DDDDprog/zeon-kernel

pub struct SmallocPageAlloc<'a> {
    smalloc: &'a mut Smalloc<PageOffsetTranslator>,
}

impl<'a> SmallocPageAlloc<'a> {
    pub fn new(smalloc: &'a mut Smalloc<PageOffsetTranslator>) -> Self {
        Self { smalloc }
    }
}

impl PageAllocator for SmallocPageAlloc<'_> {
    fn allocate_page_table<T: PgTable>(&mut self) -> Result<TPA<PgTableArray<T>>> {
        Ok(TPA::from_value(
            self.smalloc.alloc(PAGE_SIZE, PAGE_SIZE)?.value(),
        ))
    }
}
