// Zeon - Pure Rust Operating System
// https://github.com/DDDDprog/zeon-kernel

pub extern "C" fn transition_to_el1(stack_addr: u64) {
    let ret_address = {
        let mut addr: u64;
        unsafe {
            asm!("mov {0}, lr", out(reg) addr);
        }
        addr
    };

    match CurrentEL.read_as_enum(CurrentEL::EL) {
        Some(CurrentEL::EL::Value::EL0) => park_cpu(),
        Some(CurrentEL::EL::Value::EL1) => return,
        Some(CurrentEL::EL::Value::EL2) => {
            SPSR_EL2.write(
                SPSR_EL2::M::EL1h
                    + SPSR_EL2::I::Masked
                    + SPSR_EL2::F::Masked
                    + SPSR_EL2::D::Masked
                    + SPSR_EL2::A::Masked,
            );
            HCR_EL2.write(HCR_EL2::RW::EL1IsAarch64);
            ELR_EL2.set(ret_address);
        }
        Some(CurrentEL::EL::Value::EL3) => {
            SPSR_EL3.write(
                SPSR_EL3::M::EL1h
                    + SPSR_EL3::I::Masked
                    + SPSR_EL3::F::Masked
                    + SPSR_EL3::D::Masked
                    + SPSR_EL3::A::Masked,
            );
            SCR_EL3.write(SCR_EL3::RW::NextELIsAarch64);
            ELR_EL3.set(ret_address);
        }
        None => park_cpu(),
    }

    SP_EL1.set(stack_addr);

    asm::eret();
}
