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

pub trait SocketOps: Send + Sync {
    async fn bind(&self, _addr: SockAddr) -> libkernel::error::Result<()> {
        Err(KernelError::NotSupported)
    }

    async fn connect(&self, _addr: SockAddr) -> libkernel::error::Result<()> {
        Err(KernelError::NotSupported)
    }

    async fn listen(&self, _backlog: i32) -> libkernel::error::Result<()> {
        Err(KernelError::NotSupported)
    }

    async fn accept(&self) -> libkernel::error::Result<(Box<dyn SocketOps>, SockAddr)> {
        Err(KernelError::NotSupported)
    }

    async fn recv(
        &mut self,
        ctx: &mut FileCtx,
        buf: UA,
        count: usize,
        flags: RecvFlags,
    ) -> libkernel::error::Result<usize>;
    async fn recvfrom(
        &mut self,
        ctx: &mut FileCtx,
        buf: UA,
        count: usize,
        flags: RecvFlags,
        addr: Option<SockAddr>,
    ) -> libkernel::error::Result<(usize, Option<SockAddr>)>;
    async fn send(
        &mut self,
        ctx: &mut FileCtx,
        buf: UA,
        count: usize,
        flags: SendFlags,
    ) -> libkernel::error::Result<usize>;
    async fn sendto(
        &mut self,
        ctx: &mut FileCtx,
        buf: UA,
        count: usize,
        flags: SendFlags,
        addr: SockAddr,
    ) -> libkernel::error::Result<usize>;

    async fn shutdown(&self, _how: ShutdownHow) -> libkernel::error::Result<()> {
        Err(KernelError::NotSupported)
    }

    fn as_file(self: Box<Self>) -> Box<dyn FileOps>;
}

#[async_trait]
impl<T> FileOps for T
where
    T: SocketOps,
{
    async fn read(
        &mut self,
        ctx: &mut FileCtx,
        buf: UA,
        count: usize,
    ) -> libkernel::error::Result<usize> {
        self.recv(ctx, buf, count, RecvFlags::empty()).await
    }

    async fn readat(
        &mut self,
        _buf: UA,
        _count: usize,
        _offset: u64,
    ) -> libkernel::error::Result<usize> {
        Err(KernelError::NotSupported)
    }

    async fn write(
        &mut self,
        ctx: &mut FileCtx,
        buf: UA,
        count: usize,
    ) -> libkernel::error::Result<usize> {
        self.send(ctx, buf, count, SendFlags::empty()).await
    }

    async fn writeat(
        &mut self,
        _buf: UA,
        _count: usize,
        _offset: u64,
    ) -> libkernel::error::Result<usize> {
        Err(KernelError::NotSupported)
    }

    fn as_socket(&mut self) -> Option<&mut dyn SocketOps> {
        Some(self)
    }
}
