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

pub struct ProcTaskDirInode {
    id: InodeId,
    attr: FileAttr,
    tid: Tid,
}

impl ProcTaskDirInode {
    pub fn new(tid: Tid, inode_id: InodeId) -> Self {
        Self {
            id: inode_id,
            attr: FileAttr {
                file_type: FileType::Directory,
                // Define appropriate file attributes for fdinfo.
                ..FileAttr::default()
            },
            tid,
        }
    }
}

#[async_trait]
impl Inode for ProcTaskDirInode {
    fn id(&self) -> InodeId {
        self.id
    }

    async fn getattr(&self) -> libkernel::error::Result<FileAttr> {
        Ok(self.attr.clone())
    }

    async fn lookup(&self, name: &str) -> libkernel::error::Result<Arc<dyn Inode>> {
        let tid = match name.parse::<u32>() {
            Ok(tid) => Tid(tid),
            Err(_) => return Err(FsError::NotFound.into()),
        };
        let fs = procfs();
        let inode_id = InodeId::from_fsid_and_inodeid(
            fs.id(),
            get_inode_id(&[&self.tid.value().to_string(), &tid.value().to_string()]),
        );
        find_task_by_tid(self.tid).ok_or(FsError::NotFound)?;
        Ok(Arc::new(ProcTaskInode::new(self.tid, true, inode_id)))
    }

    async fn readdir(&self, start_offset: u64) -> libkernel::error::Result<Box<dyn DirStream>> {
        let process = &find_task_by_tid(self.tid).ok_or(FsError::NotFound)?.process;
        let tasks = process.tasks.lock_save_irq();
        let mut entries = Vec::new();
        for (i, (_tid, task)) in tasks.iter().enumerate().skip(start_offset as usize) {
            let Some(task) = task.upgrade() else {
                continue;
            };
            let id = InodeId::from_fsid_and_inodeid(
                procfs().id(),
                get_inode_id(&[&self.tid.value().to_string(), &task.tid.value().to_string()]),
            );
            entries.push(Dirent {
                id,
                offset: (i + 1) as u64,
                file_type: FileType::Directory,
                name: task.tid.value().to_string(),
            });
        }
        Ok(Box::new(SimpleDirStream::new(entries, start_offset)))
    }
}
