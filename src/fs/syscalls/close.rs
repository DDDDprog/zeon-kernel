pub async fn sys_close(ctx: &ProcessCtx, fd: Fd) -> Result<usize> {
    close(ctx, fd).await?;
    Ok(0)
}

bitflags! {
    pub struct CloseRangeFlags: i32 {
        const CLOSE_RANGE_UNSHARE = 1 << 1;
        const CLOSE_RANGE_CLOEXEC = 1 << 2;
    }
}

pub async fn sys_close_range(ctx: &ProcessCtx, first: Fd, last: Fd, flags: i32) -> Result<usize> {
    let flags = CloseRangeFlags::from_bits_truncate(flags);
    if flags.contains(CloseRangeFlags::CLOSE_RANGE_UNSHARE) {
        todo!("Implement CLOSE_RANGE_UNSHARE");
    }
    if flags.contains(CloseRangeFlags::CLOSE_RANGE_CLOEXEC) {
        todo!("Implement CLOSE_RANGE_CLOEXEC");
    }

    for i in first.as_raw()..=last.as_raw() {
        close(ctx, Fd(i)).await?;
    }
    Ok(0)
}
