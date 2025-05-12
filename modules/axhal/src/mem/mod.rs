//! Physical memory management.

use core::fmt;

cfg_if::cfg_if! {
    if #[cfg(feature = "plat-dyn")]{
        mod r#dyn;
        pub use r#dyn::*;
    }else{
        mod r#static;
        pub use r#static::*;
    }
}

#[doc(no_inline)]
pub use memory_addr::{MemoryAddr, PAGE_SIZE_4K, PhysAddr, VirtAddr};

bitflags::bitflags! {
    /// The flags of a physical memory region.
    pub struct MemRegionFlags: usize {
        /// Readable.
        const READ          = 1 << 0;
        /// Writable.
        const WRITE         = 1 << 1;
        /// Executable.
        const EXECUTE       = 1 << 2;
        /// Device memory. (e.g., MMIO regions)
        const DEVICE        = 1 << 4;
        /// Uncachable memory. (e.g., framebuffer)
        const UNCACHED      = 1 << 5;
        /// Reserved memory, do not use for allocation.
        const RESERVED      = 1 << 6;
        /// Free memory for allocation.
        const FREE          = 1 << 7;
    }
}

impl fmt::Debug for MemRegionFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

/// A physical memory region.
#[derive(Debug)]
pub struct MemRegion {
    /// The start physical address of the region.
    pub paddr: PhysAddr,
    /// The size in bytes of the region.
    pub size: usize,
    /// The region flags, see [`MemRegionFlags`].
    pub flags: MemRegionFlags,
    /// The region name, used for identification.
    pub name: &'static str,
}

pub fn get_kernel_aspace_start() -> VirtAddr {
    cfg_if::cfg_if! {
        if #[cfg(feature = "plat-dyn")] {
            somehal::mem::KERNEL_ADDR_SPACE_START.into()
        } else {
            axconfig::plat::KERNEL_ASPACE_BASE.into()
        }
    }
}

pub fn get_kernel_aspace_size() -> usize {
    cfg_if::cfg_if! {
        if #[cfg(feature = "plat-dyn")] {
            somehal::mem::KERNEL_ADDR_SPACE_SIZE
        } else {
            axconfig::plat::KERNEL_ASPACE_SIZE
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn _percpu_start_addr() -> usize {
    percpu_all().as_ptr() as usize
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn _percpu_end_addr() -> usize {
    percpu_all().as_ptr_range().end as usize
}
