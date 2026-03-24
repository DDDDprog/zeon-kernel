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

pub struct Ext4FsDriver {}

impl Ext4FsDriver {
    pub fn new() -> Self {
        Self {}
    }
}

impl Driver for Ext4FsDriver {
    fn name(&self) -> &'static str {
        "ext4fs"
    }

    fn as_filesystem_driver(self: Arc<Self>) -> Option<Arc<dyn FilesystemDriver>> {
        Some(self)
    }
}

#[async_trait]
impl FilesystemDriver for Ext4FsDriver {
    async fn construct(
        &self,
        fs_id: u64,
        device: Option<Box<dyn BlockDevice>>,
    ) -> Result<Arc<dyn Filesystem>> {
        match device {
            Some(dev) => Ok(Ext4Filesystem::<ArchImpl>::new(BlockBuffer::new(dev), fs_id).await?),
            None => {
                warn!("Could not mount fat32 fs with no block device");
                Err(KernelError::InvalidValue)
            }
        }
    }
}
