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

pub fn sys_name_to_handle_at() -> Result<usize> {
    Err(KernelError::OpNotSupported)
}
