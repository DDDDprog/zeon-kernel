/*

fn arch_init_stage1(
    dtb_ptr: TPA<u8>,
    image_start: PA,
    image_end: PA,
    highmem_pgtable_base: TPA<PgTableArray<L0Table>>,
) -> VA {
    (|| -> Result<VA> {
        setup_console_logger();

        setup_allocator(dtb_ptr, image_start, image_end)?;

        let dtb_addr = {
            let mut fixmaps = FIXMAPS.lock_save_irq();
            fixmaps.setup_fixmaps(highmem_pgtable_base);

            unsafe { fixmaps.remap_fdt(dtb_ptr) }.unwrap()
        };

        set_fdt_va(dtb_addr.cast());
        setup_logical_map(highmem_pgtable_base)?;
        let stack_addr = setup_stack_and_heap(highmem_pgtable_base)?;
        setup_kern_addr_space(highmem_pgtable_base)?;

        Ok(stack_addr)
    })()
    .unwrap_or_else(|_| park_cpu())
}

#[unsafe(no_mangle)]
fn arch_init_stage2(frame: *mut ExceptionState) -> *mut ExceptionState {
    // Save the ID map addr for booting secondaries.
    save_idmap(PA::from_value(TTBR0_EL1.get_baddr() as _));

    // Disable the ID map.
    TCR_EL1.modify(TCR_EL1::EPD0::DisableTTBR0Walks);
    barrier::isb(barrier::SY);

    // We now have enough memory setup to switch to the real page allocator.
    let smalloc = INITAL_ALLOCATOR
        .lock_save_irq()
        .take()
        .expect("Smalloc should not have been taken yet");

    let (page_alloc, frame_list) = unsafe { FrameAllocator::init(smalloc) };

    if PAGE_ALLOC.set(page_alloc).is_err() {
        panic!("Cannot setup physical memory allocator");
    }

    if SLAB_ALLOC.set(SlabAllocator::new(frame_list)).is_err() {
        panic!("Cannot setup slab allocator");
    }

    KernelHeap::init_for_this_cpu();

    // Don't trap wfi/wfe in el0.
    SCTLR_EL1.modify(SCTLR_EL1::NTWE::DontTrap + SCTLR_EL1::NTWI::DontTrap);

    exceptions_init().expect("Failed to initialize exceptions");
    ArchImpl::enable_interrupts();

    unsafe { run_initcalls() };
    probe_for_fdt_devices();

    unsafe { setup_percpu(cpu_count()) };

    cpu_messenger_init(cpu_count());

    if let Err(e) = vdso_init() {
        panic!("VDSO setup failed: {e}");
    }

    let cmdline = super::fdt::get_cmdline();

    kmain(cmdline.unwrap_or_default(), frame);

    boot_secondaries();

    // Prove that we can send IPIs through the messenger.
    frame
}

fn arch_init_secondary(ctx_frame: *mut ExceptionState) -> *mut ExceptionState {
    // Disable the ID map.
    TCR_EL1.modify(TCR_EL1::EPD0::DisableTTBR0Walks);
    barrier::isb(barrier::SY);

    // Don't trap secondaries wfi/wfe in el0.
    SCTLR_EL1.modify(SCTLR_EL1::NTWE::DontTrap + SCTLR_EL1::NTWI::DontTrap);

    // Setup heap per-cpu data.
    KernelHeap::init_for_this_cpu();

    // Enable interrupts and exceptions.
    secondary_exceptions_init();

    if let Some(ic) = get_interrupt_root() {
        ic.enable_core(ArchImpl::id());
    }

    // Arm the per-CPU system timer so this core starts receiving timer IRQs.
    kick_current_cpu();

    ArchImpl::enable_interrupts();

    secondary_booted();

    sched_init_secondary();

    dispatch_userspace_task(ctx_frame);

    ctx_frame
}

#[unsafe(no_mangle)]
pub extern "C" fn park_cpu() -> ! {
    loop {
        asm::wfe();
    }
}
