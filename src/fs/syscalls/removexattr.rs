pub async fn sys_removexattr(
    ctx: &ProcessCtx,
    path: TUA<c_char>,
    name: TUA<c_char>,
) -> Result<usize> {
    let mut buf = [0; 1024];

    let path = Path::new(UserCStr::from_ptr(path).copy_from_user(&mut buf).await?);
    let task = ctx.shared().clone();

    let node = VFS.resolve_path(path, VFS.root_inode(), &task).await?;
    let mut buf = [0; 1024];
    removexattr(
        node,
        UserCStr::from_ptr(name).copy_from_user(&mut buf).await?,
    )
    .await?;
    Ok(0)
}

pub async fn sys_lremovexattr(
    ctx: &ProcessCtx,
    path: TUA<c_char>,
    name: TUA<c_char>,
) -> Result<usize> {
    let mut buf = [0; 1024];

    let path = Path::new(UserCStr::from_ptr(path).copy_from_user(&mut buf).await?);
    let task = ctx.shared().clone();

    let node = VFS
        .resolve_path_nofollow(path, VFS.root_inode(), &task)
        .await?;
    let mut buf = [0; 1024];
    removexattr(
        node,
        UserCStr::from_ptr(name).copy_from_user(&mut buf).await?,
    )
    .await?;
    Ok(0)
}

pub async fn sys_fremovexattr(ctx: &ProcessCtx, fd: Fd, name: TUA<c_char>) -> Result<usize> {
    let node = {
        let task = ctx.shared().clone();
        let file = task
            .fd_table
            .lock_save_irq()
            .get(fd)
            .ok_or(KernelError::BadFd)?;

        file.inode().ok_or(KernelError::BadFd)?
    };
    let mut buf = [0; 1024];
    removexattr(
        node,
        UserCStr::from_ptr(name).copy_from_user(&mut buf).await?,
    )
    .await?;
    Ok(0)
}
