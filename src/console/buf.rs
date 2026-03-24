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

impl BufConsole {
    pub const fn new() -> Self {
        Self {
            data: [0; BUF_CONSOLE_SZ],
            ptr: 0,
        }
    }

    pub fn data(&self) -> &[u8] {
        &self.data[..self.ptr]
    }
}

impl Write for BufConsole {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let available = BUF_CONSOLE_SZ.saturating_sub(self.ptr);
        let len = core::cmp::min(available, s.len());
        self.data[self.ptr..self.ptr + len].copy_from_slice(&s.as_bytes()[..len]);
        self.ptr += len;
        Ok(())
    }
}
