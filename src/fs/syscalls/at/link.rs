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

pub async fn sys_linkat(
    ctx: &ProcessCtx,
    old_dirfd: Fd,
    old_path: TUA<c_char>,
    new_dirfd: Fd,
    new_path: TUA<c_char>,
    flags: i32,
) -> Result<usize> {
    let mut buf = [0; 1024];
    let mut buf2 = [0; 1024];

    let task = ctx.shared().clone();
    let mut flags = AtFlags::from_bits_retain(flags);

    // following symlinks is implied for any other syscall.
    // for linkat though, we need to specify nofollow since
    // linkat implicitly does not follow symlinks unless specified.
    if !flags.contains(AtFlags::AT_SYMLINK_FOLLOW) {
        flags.insert(AtFlags::AT_SYMLINK_NOFOLLOW);
    }

    if flags.contains(AtFlags::AT_EMPTY_PATH)
        && !task
            .creds
            .lock_save_irq()
            .caps()
            .is_capable(CapabilitiesFlags::CAP_DAC_READ_SEARCH)
    {
        return Err(FsError::NotFound.into()); // weird error but thats what linkat(2) says
    }

    let old_path = Path::new(
        UserCStr::from_ptr(old_path)
            .copy_from_user(&mut buf)
            .await?,
    );
    let new_path = Path::new(
        UserCStr::from_ptr(new_path)
            .copy_from_user(&mut buf2)
            .await?,
    );
    let old_start_node = resolve_at_start_node(ctx, old_dirfd, old_path, flags).await?;
    let new_start_node = resolve_at_start_node(ctx, new_dirfd, new_path, flags).await?;

    let target_inode =
        resolve_path_flags(old_dirfd, old_path, old_start_node.clone(), &task, flags).await?;

    let attr = target_inode.getattr().await?;

    if attr.file_type == FileType::Directory {
        return Err(FsError::IsADirectory.into());
    }

    // newpath does not follow flags, and doesnt follow symlinks either
    if VFS
        .resolve_path_nofollow(new_path, new_start_node.clone(), &task)
        .await
        .is_ok()
    {
        return Err(FsError::AlreadyExists.into());
    }

    // parent newpath should follow symlinks though
    let parent_inode = if let Some(parent) = new_path.parent() {
        VFS.resolve_path(parent, new_start_node, &task).await?
    } else {
        new_start_node
    };

    if parent_inode.getattr().await?.file_type != FileType::Directory {
        return Err(FsError::NotADirectory.into());
    }

    VFS.link(
        target_inode,
        parent_inode,
        new_path.file_name().ok_or(KernelError::InvalidValue)?,
    )
    .await?;

    Ok(0)
}
