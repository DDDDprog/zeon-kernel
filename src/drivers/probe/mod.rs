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

pub enum DeviceMatchType {
    FdtCompatible(&'static str),
}

#[derive(Clone)]
pub enum DeviceDescriptor {
    Fdt(fdt_parser::Node<'static>, FdtFlags),
}

impl Display for DeviceDescriptor {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DeviceDescriptor::Fdt(node, _) => f.write_str(node.name),
        }
    }
}

pub type ProbeFn =
    Box<dyn Fn(&mut DriverManager, DeviceDescriptor) -> Result<Arc<dyn Driver>> + Send>;
