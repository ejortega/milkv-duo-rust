use std::fs::{File, OpenOptions};
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::AsRawFd;
use std::ptr;

use anyhow::{anyhow, Context, Result};
use libc::{c_void, mmap, munmap, off_t, MAP_FAILED, MAP_SHARED, PROT_READ, PROT_WRITE};

pub struct DevMem {
    dev_mem: File,
    page_size: usize,
}

impl DevMem {
    pub fn new() -> Result<Self> {
        let dev_mem = OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(libc::O_SYNC)
            .open("/dev/mem")
            .context("Error opening /dev/mem")?;

        let page_size = unsafe { libc::sysconf(libc::_SC_PAGE_SIZE) } as usize;

        Ok(Self { dev_mem, page_size })
    }

    pub fn mem_write(&self, addr: usize, val: u32) -> Result<()> {
        let virt_addr =
            self.dev_mmap(addr, 4).context(format!("Failed to write {val} at address {addr}"))?;

        unsafe {
            ptr::write_volatile(virt_addr as *mut u32, val);
        }

        self.dev_munmap(virt_addr, 4)
    }

    pub fn mem_read(&self, addr: usize) -> Result<u32> {
        let virt_addr =
            self.dev_mmap(addr, 4).context(format!("Failed to read from address {addr}"))?;

        let val = unsafe { ptr::read_volatile(virt_addr as *mut u32) };

        self.dev_munmap(virt_addr, 4)?;

        Ok(val)
    }

    fn dev_mmap(&self, addr: usize, len: usize) -> Result<*mut c_void> {
        let offset = addr & !(self.page_size - 1);
        let map_len = len + addr - offset;

        let map_base = unsafe {
            mmap(
                ptr::null_mut(),
                map_len,
                PROT_READ | PROT_WRITE,
                MAP_SHARED,
                self.dev_mem.as_raw_fd(),
                offset as off_t,
            )
        };

        if map_base == MAP_FAILED || map_base.is_null() {
            let err = std::io::Error::last_os_error();
            Err(anyhow!("Unable to allocate memory: {err}"))
        } else {
            let mapped_addr = map_base as usize + addr - offset;
            Ok(mapped_addr as *mut c_void)
        }
    }

    fn dev_munmap(&self, virt_addr: *mut c_void, len: usize) -> Result<()> {
        let addr = (virt_addr as usize) & !(self.page_size - 1);
        let unmap_len = len + (virt_addr as usize) - addr;

        let result = unsafe { munmap(addr as *mut c_void, unmap_len) };

        if result == -1 {
            let err = std::io::Error::last_os_error();
            Err(anyhow!("Error running munmap: {err}"))
        } else {
            Ok(())
        }
    }
}

impl Drop for DevMem {
    fn drop(&mut self) {
        unsafe {
            libc::close(self.dev_mem.as_raw_fd());
        }
    }
}
