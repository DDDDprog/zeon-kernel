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

mod boot;
mod cpu_ops;
mod exceptions;
mod fdt;
mod memory;
mod proc;
pub mod psci;
pub mod ptrace;

pub struct Aarch64 {}

impl CpuOps for Aarch64 {
    fn id() -> usize {
        MPIDR_EL1.read(MPIDR_EL1::Aff0) as _
    }

    fn halt() -> ! {
        loop {
            wfi();
        }
    }

    fn disable_interrupts() -> usize {
        local_irq_save()
    }

    fn restore_interrupt_state(state: usize) {
        local_irq_restore(state);
    }

    fn enable_interrupts() {
        DAIF.modify(DAIF::I::Unmasked);
    }
}

impl VirtualMemory for Aarch64 {
    type PageTableRoot = PgTableArray<L0Table>;
    type ProcessAddressSpace = Arm64ProcessAddressSpace;
    type KernelAddressSpace = Arm64KernelAddressSpace;

    const PAGE_OFFSET: usize = PAGE_OFFSET;

    fn kern_address_space() -> &'static SpinLock<Self::KernelAddressSpace> {
        KERN_ADDR_SPC.get().unwrap()
    }
}

impl Arch for Aarch64 {
    type UserContext = ExceptionState;
    type PTraceGpRegs = Arm64PtraceGPRegs;

    fn new_user_context(entry_point: VA, stack_top: VA) -> Self::UserContext {
        ExceptionState {
            x: [0; 31],
            elr_el1: entry_point.value() as _,
            spsr_el1: 0,
            sp_el0: stack_top.value() as _,
            tpid_el0: 0,
        }
    }

    fn name() -> &'static str {
        "aarch64"
    }

    fn cpu_count() -> usize {
        boot::secondary::cpu_count()
    }

    fn do_signal(
        ctx: ProcessCtx,
        sig: SigId,
        action: UserspaceSigAction,
    ) -> impl Future<Output = Result<<Self as Arch>::UserContext>> {
        proc::signal::do_signal(ctx, sig, action)
    }

    fn do_signal_return(
        ctx: ProcessCtx,
    ) -> impl Future<Output = Result<<Self as Arch>::UserContext>> {
        proc::signal::do_signal_return(ctx)
    }

    fn context_switch(new: Arc<Task>) {
        proc::context_switch(new);
    }

    fn create_idle_task() -> OwnedTask {
        proc::idle::create_idle_task()
    }

    fn power_off() -> ! {
        // Try PSCI `SYSTEM_OFF` first (works on QEMU `-machine virt` and most
        // real hardware that implements the PSCI interface).
        const PSCI_SYSTEM_OFF: u32 = 0x8400_0008;
        unsafe {
            psci::do_psci_hyp_call(PSCI_SYSTEM_OFF, 0, 0, 0);
        }

        // Fallback: halt the CPU indefinitely.
        Self::halt()
    }

    fn restart() -> ! {
        const PSCI_SYSTEM_RESET: u32 = 0x8400_0009;
        unsafe {
            psci::do_psci_hyp_call(PSCI_SYSTEM_RESET, 0, 0, 0);
        }

        // Fallback: halt the CPU indefinitely.
        Self::halt()
    }

    fn get_cmdline() -> Option<String> {
        fdt::get_cmdline()
    }

    unsafe fn copy_from_user(
        src: UA,
        dst: *mut (),
        len: usize,
    ) -> impl Future<Output = Result<()>> {
        Arm64CopyFromUser::new(src, dst, len)
    }

    unsafe fn try_copy_from_user(src: UA, dst: *mut (), len: usize) -> Result<()> {
        try_copy_from_user(src, dst, len)
    }

    unsafe fn copy_to_user(
        src: *const (),
        dst: UA,
        len: usize,
    ) -> impl Future<Output = Result<()>> {
        Arm64CopyToUser::new(src, dst, len)
    }

    unsafe fn copy_strn_from_user(
        src: UA,
        dst: *mut u8,
        len: usize,
    ) -> impl Future<Output = Result<usize>> {
        Arm64CopyStrnFromUser::new(src, dst as *mut _, len)
    }
}
