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

pub mod cgroup;
pub mod dev;
pub mod ext4;
pub mod fat32;
pub mod proc;
pub mod sys;
pub mod tmpfs;

pub fn register_fs_drivers() {
    let mut dm = DM.lock_save_irq();

    dm.insert_driver(Arc::new(Ext4FsDriver::new()));
    dm.insert_driver(Arc::new(Fat32FsDriver::new()));
    dm.insert_driver(Arc::new(DevFsDriver::new()));
    dm.insert_driver(Arc::new(ProcFsDriver::new()));
    dm.insert_driver(Arc::new(SysFsDriver::new()));
    dm.insert_driver(Arc::new(TmpFsDriver::new()));
    dm.insert_driver(Arc::new(CgroupFsDriver::new()));
}
