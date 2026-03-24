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

pub async fn sys_truncate(ctx: &ProcessCtx, path: TUA<c_char>, new_size: usize) -> Result<usize> {
    let mut buf = [0; 1024];

    let task = ctx.shared().clone();
    let path = Path::new(UserCStr::from_ptr(path).copy_from_user(&mut buf).await?);

    let root = task.root.lock_save_irq().0.clone();
    let file = VFS
        .open(
            path,
            OpenFlags::O_WRONLY,
            root,
            FilePermissions::empty(),
            &task,
        )
        .await?;

    let (ops, ctx) = &mut *file.lock().await;

    ops.truncate(ctx, new_size).await.map(|_| 0)
}

pub async fn sys_ftruncate(ctx: &ProcessCtx, fd: Fd, new_size: usize) -> Result<usize> {
    let fd = ctx
        .shared()
        .fd_table
        .lock_save_irq()
        .get(fd)
        .ok_or(KernelError::BadFd)?;

    let (ops, ctx) = &mut *fd.lock().await;

    ops.truncate(ctx, new_size).await.map(|_| 0)
}
