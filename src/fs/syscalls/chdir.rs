// Zeon - Pure Rust Operating System
// https://github.com/DDDDprog/zeon-kernel

pub async fn sys_getcwd(ctx: &ProcessCtx, buf: UA, len: usize) -> Result<usize> {
    let task = ctx.shared().clone();
    let path = task.cwd.lock_save_irq().1.as_str().to_string();
    let cstr = CString::from_str(&path).map_err(|_| KernelError::InvalidValue)?;
    let slice = cstr.as_bytes_with_nul();

    if slice.len() > len {
        return Err(KernelError::TooLarge);
    }

    copy_to_user_slice(slice, buf).await?;

    Ok(buf.value())
}

pub async fn sys_chdir(ctx: &ProcessCtx, path: TUA<c_char>) -> Result<usize> {
    let mut buf = [0; 1024];

    let path = Path::new(UserCStr::from_ptr(path).copy_from_user(&mut buf).await?);
    let task = ctx.shared().clone();
    let current_path = task.cwd.lock_save_irq().0.clone();
    let new_path = task.cwd.lock_save_irq().1.join(path);

    let node = VFS.resolve_path(path, current_path, &task).await?;

    *task.cwd.lock_save_irq() = (node, new_path);

    Ok(0)
}

pub async fn sys_chroot(ctx: &ProcessCtx, path: TUA<c_char>) -> Result<usize> {
    let task = ctx.shared().clone();
    task.creds
        .lock_save_irq()
        .caps()
        .check_capable(CapabilitiesFlags::CAP_SYS_CHROOT)?;

    let mut buf = [0; 1024];

    let path = Path::new(UserCStr::from_ptr(path).copy_from_user(&mut buf).await?);
    let current_path = task.root.lock_save_irq().0.clone();
    let new_path = task.root.lock_save_irq().1.join(path);

    let node = VFS.resolve_path(path, current_path, &task).await?;

    *task.root.lock_save_irq() = (node, new_path);

    Ok(0)
}

pub async fn sys_fchdir(ctx: &ProcessCtx, fd: Fd) -> Result<usize> {
    let task = ctx.shared().clone();
    let file = task
        .fd_table
        .lock_save_irq()
        .get(fd)
        .ok_or(KernelError::BadFd)?;

    *task.cwd.lock_save_irq() = (
        file.inode().ok_or(KernelError::BadFd)?,
        file.path().ok_or(KernelError::BadFd)?.to_owned(),
    );

    Ok(0)
}
