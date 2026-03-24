// Zeon - Pure Rust Operating System
// https://github.com/DDDDprog/zeon-kernel

pub async fn sys_bind(
    ctx: &ProcessCtx,
    fd: Fd,
    addr: UA,
    addrlen: SocketLen,
) -> libkernel::error::Result<usize> {
    let file = ctx
        .shared()
        .fd_table
        .lock_save_irq()
        .get(fd)
        .ok_or(libkernel::error::KernelError::BadFd)?;

    let (ops, _ctx) = &mut *file.lock().await;
    let addr = parse_sockaddr(addr, addrlen).await?;

    ops.as_socket()
        .ok_or(libkernel::error::KernelError::NotASocket)?
        .bind(addr)
        .await?;
    Ok(0)
}
