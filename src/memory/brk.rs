/*

pub async fn sys_brk(ctx: &ProcessCtx, addr: VA) -> Result<usize, Infallible> {
    let mut vm = ctx.shared().vm.lock_save_irq();

    // The query case `brk(0)` is special and is handled separately from modifications.
    if addr.is_null() {
        let current_brk_val = vm.current_brk().value();
        return Ok(current_brk_val);
    }

    // For non-null addresses, attempt to resize the break.
    let resize_result = vm.resize_brk(addr);

    match resize_result {
        // Success: The break was resized. The function returns the new address.
        Ok(new_brk) => Ok(new_brk.value()),
        // Failure: The resize was invalid (e.g., collision, shrink below start).
        // The contract is to return the current, unchanged break address.
        Err(_) => {
            let current_brk_val = vm.current_brk().value();
            Ok(current_brk_val)
        }
    }
}
