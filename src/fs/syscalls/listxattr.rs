// Zeon - Pure Rust Operating System
// https://github.com/DDDDprog/zeon-kernel

pub async fn sys_listxattr(
    ctx: &ProcessCtx,
    path: TUA<core::ffi::c_char>,
    list: UA,
    size: usize,
) -> Result<usize> {
    let mut buf = [0; 1024];

    let path = Path::new(UserCStr::from_ptr(path).copy_from_user(&mut buf).await?);
    let task = ctx.shared().clone();

    let node = VFS.resolve_path(path, VFS.root_inode(), &task).await?;
    listxattr(node, list, size).await
}

pub async fn sys_llistxattr(
    ctx: &ProcessCtx,
    path: TUA<core::ffi::c_char>,
    list: UA,
    size: usize,
) -> Result<usize> {
    let mut buf = [0; 1024];

    let path = Path::new(UserCStr::from_ptr(path).copy_from_user(&mut buf).await?);
    let task = ctx.shared().clone();

    let node = VFS
        .resolve_path_nofollow(path, VFS.root_inode(), &task)
        .await?;
    listxattr(node, list, size).await
}

pub async fn sys_flistxattr(ctx: &ProcessCtx, fd: Fd, list: UA, size: usize) -> Result<usize> {
    let node = {
        let task = ctx.shared().clone();
        let file = task
            .fd_table
            .lock_save_irq()
            .get(fd)
            .ok_or(KernelError::BadFd)?;

        file.inode().ok_or(KernelError::BadFd)?
    };
    listxattr(node, list, size).await
}
