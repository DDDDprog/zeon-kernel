pub async fn sys_fchmod(ctx: &ProcessCtx, fd: Fd, mode: u16) -> Result<usize> {
    let task = ctx.shared().clone();
    let file = task
        .fd_table
        .lock_save_irq()
        .get(fd)
        .ok_or(KernelError::BadFd)?;
    let permissions = FilePermissions::from_bits_retain(mode);

    let inode = file.inode().ok_or(KernelError::BadFd)?;
    let mut attr = inode.getattr().await?;

    if !can_chmod(task, attr.uid) {
        return Err(KernelError::NotPermitted);
    }

    attr.permissions = permissions;
    inode.setattr(attr).await?;

    Ok(0)
}
