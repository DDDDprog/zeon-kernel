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

pub async fn sys_symlinkat(
    ctx: &ProcessCtx,
    old_name: TUA<c_char>,
    new_dirfd: Fd,
    new_name: TUA<c_char>,
) -> Result<usize> {
    let mut buf = [0; 1024];
    let mut buf2 = [0; 1024];

    let task = ctx.shared().clone();
    let source = Path::new(
        UserCStr::from_ptr(old_name)
            .copy_from_user(&mut buf)
            .await?,
    );
    let target = Path::new(
        UserCStr::from_ptr(new_name)
            .copy_from_user(&mut buf2)
            .await?,
    );
    let start_node = resolve_at_start_node(ctx, new_dirfd, target, AtFlags::empty()).await?;

    VFS.symlink(source, target, start_node, &task).await?;

    Ok(0)
}
