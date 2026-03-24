// Zeon - Pure Rust Operating System
// https://github.com/DDDDprog/zeon-kernel

pub async fn sys_fstat(ctx: &ProcessCtx, fd: Fd, statbuf: TUA<Stat>) -> Result<usize> {
    let fd = ctx
        .shared()
        .fd_table
        .lock_save_irq()
        .get(fd)
        .ok_or(KernelError::BadFd)?;

    let inode = fd.inode().ok_or(KernelError::BadFd)?;

    let attr = inode.getattr().await?;

    copy_to_user(statbuf, attr.into()).await?;

    Ok(0)
}
