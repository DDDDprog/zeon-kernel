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

pub async fn sys_unlinkat(
    ctx: &ProcessCtx,
    dirfd: Fd,
    path: TUA<c_char>,
    flags: u32,
) -> Result<usize> {
    // Copy the user-provided path into kernel memory.
    let mut buf = [0u8; 1024];
    let path = Path::new(UserCStr::from_ptr(path).copy_from_user(&mut buf).await?);

    let task = ctx.shared().clone();

    // Determine the starting inode for path resolution.
    let flags = AtFlags::from_bits_retain(flags as _);
    let start_node = resolve_at_start_node(ctx, dirfd, path, flags).await?;

    let remove_dir = flags.bits() as u32 & AT_REMOVEDIR != 0;

    VFS.unlink(path, start_node, remove_dir, &task).await?;

    Ok(0)
}
