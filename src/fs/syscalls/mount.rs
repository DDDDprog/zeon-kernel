// Zeon - Pure Rust Operating System
// https://github.com/DDDDprog/zeon-kernel

pub async fn sys_mount(
    ctx: &ProcessCtx,
    dev_name: TUA<c_char>,
    dir_name: TUA<c_char>,
    type_: TUA<c_char>,
    flags: i64,
    _data: UA,
) -> Result<usize> {
    let flags = MountFlags::from_bits_truncate(flags as u64);
    if flags.contains(MountFlags::MS_REC) {
        // TODO: Handle later
        return Ok(0);
    }
    let mut buf = [0u8; 1024];
    let dev_name = if dev_name.is_null() {
        None
    } else {
        Some(
            UserCStr::from_ptr(dev_name)
                .copy_from_user(&mut buf)
                .await?,
        )
    };
    let mut buf = [0u8; 1024];
    let dir_name = UserCStr::from_ptr(dir_name)
        .copy_from_user(&mut buf)
        .await?;
    let mount_point = VFS
        .resolve_path(Path::new(dir_name), VFS.root_inode(), ctx.shared())
        .await?;
    let mut buf = [0u8; 1024];
    let _type = if type_.is_null() {
        None
    } else {
        Some(UserCStr::from_ptr(type_).copy_from_user(&mut buf).await?)
    };
    if let Some(dev_name) = dev_name {
        let dev_name = match dev_name {
            "proc" => "procfs",
            "devtmpfs" => "devfs",
            "cgroup2" => "cgroupfs",
            s => s,
        };
        VFS.mount(mount_point, dev_name, None).await?;
        Ok(0)
    } else {
        Err(KernelError::NotSupported)
    }
}
