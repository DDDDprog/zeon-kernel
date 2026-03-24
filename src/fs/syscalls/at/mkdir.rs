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

pub async fn sys_mkdirat(
    ctx: &ProcessCtx,
    dirfd: Fd,
    path: TUA<c_char>,
    mode: u16,
) -> libkernel::error::Result<usize> {
    let mut buf = [0; 1024];

    let task = ctx.shared().clone();
    let path = Path::new(UserCStr::from_ptr(path).copy_from_user(&mut buf).await?);
    let start_node = resolve_at_start_node(ctx, dirfd, path, AtFlags::empty()).await?;
    let mode = FilePermissions::from_bits_retain(mode);

    VFS.mkdir(path, start_node, mode, &task).await?;
    Ok(0)
}
