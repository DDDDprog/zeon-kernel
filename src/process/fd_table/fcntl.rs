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

pub async fn sys_fcntl(ctx: &ProcessCtx, fd: Fd, op: u32, arg: usize) -> Result<usize> {
    let task = ctx.shared();

    match op {
        F_DUPFD => dup_fd(ctx, fd, Some(Fd(arg as i32))).map(|new_fd| new_fd.as_raw() as _),
        F_GETFD => {
            let fds = task.fd_table.lock_save_irq();
            let fd = fds
                .entries
                .get(fd.as_raw() as usize)
                .and_then(|entry| entry.as_ref())
                .ok_or(KernelError::BadFd)?;
            Ok(fd.flags.bits() as _)
        }
        F_SETFD => {
            let mut fds = task.fd_table.lock_save_irq();
            let fd = fds
                .entries
                .get_mut(fd.as_raw() as usize)
                .and_then(|entry| entry.as_mut())
                .ok_or(KernelError::BadFd)?;

            let new_flags = FdFlags::from_bits_retain(arg as _);
            if new_flags.contains_unknown_bits() {
                return Err(KernelError::InvalidValue);
            }
            fd.flags = new_flags;
            Ok(0)
        }
        F_GETFL => {
            let open_fd = {
                let mut fds = task.fd_table.lock_save_irq();
                let fd = fds
                    .entries
                    .get_mut(fd.as_raw() as usize)
                    .and_then(|entry| entry.as_mut())
                    .ok_or(KernelError::BadFd)?;

                fd.file.clone()
            };

            Ok(open_fd.flags().await.bits() as _)
        }
        F_SETFL => {
            let fl = OpenFlags::from_bits_retain(arg as _);
            if fl.contains_unknown_bits() {
                return Err(KernelError::InvalidValue);
            }
            let open_fd = {
                let mut fds = task.fd_table.lock_save_irq();
                let fd = fds
                    .entries
                    .get_mut(fd.as_raw() as usize)
                    .and_then(|entry| entry.as_mut())
                    .ok_or(KernelError::BadFd)?;

                fd.file.clone()
            };
            // TODO: Ignore sync/dsync when implemented
            open_fd.set_flags(fl).await;
            Ok(0)
        }
        _ => Err(KernelError::InvalidValue),
    }
}
