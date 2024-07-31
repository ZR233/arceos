use core::{alloc::Layout, ptr::NonNull};

use allocator::{AllocError, AllocResult, BaseAllocator as _, ByteAllocator as _};
use axalloc::{global_allocator, DefaultByteAllocator};
use axhal::{mem::virt_to_phys, paging::MappingFlags};
use kspin::SpinNoIrq;
use log::{debug, error};
use memory_addr::{align_up, VirtAddr, PAGE_SIZE_4K};
use os_dma::DMAInfo;

use crate::phys_to_bus;

pub(crate) static ALLOCATOR: SpinNoIrq<DmaAllocator> = SpinNoIrq::new(DmaAllocator::new());

pub(crate) struct DmaAllocator {
    alloc: DefaultByteAllocator,
}

impl DmaAllocator {
    pub const fn new() -> Self {
        Self {
            alloc: DefaultByteAllocator::new(),
        }
    }

    pub unsafe fn alloc_coherent(&mut self, layout: Layout) -> AllocResult<DMAInfo> {
        loop {
            if let Ok(data) = self.alloc.alloc(layout) {
                let cpu_addr = data.as_ptr() as usize;
                let paddr = virt_to_phys(VirtAddr::from(cpu_addr));
                let bus_addr = phys_to_bus(paddr);
                return Ok(DMAInfo {
                    cpu_addr,
                    bus_addr: bus_addr.as_u64(),
                });
            } else {
                let align = PAGE_SIZE_4K.max(layout.align());
                let want_pages = align_up(layout.size() + 1, PAGE_SIZE_4K) / PAGE_SIZE_4K;
                let available_pages = global_allocator().available_pages();
                if want_pages > available_pages {
                    return Err(AllocError::NoMemory);
                }
                // at least 32 pages or available pages.
                let num_pages = want_pages.max((32).min(available_pages));
                let expand_size = num_pages * PAGE_SIZE_4K;
                let vaddr = global_allocator().alloc_pages(num_pages, align)?;
                let want_flags = MappingFlags::READ | MappingFlags::WRITE | MappingFlags::UNCACHED;
                self.table_change_flag(VirtAddr::from(vaddr), expand_size, want_flags)?;

                self.alloc
                    .add_memory(vaddr, expand_size)
                    .inspect_err(|e| error!("add memory fail: {e:?}"))?;
                debug!("expand memory @{vaddr:#X}, size: {expand_size:#X} bytes");
            }
        }
    }

    pub unsafe fn dealloc_coherent(&mut self, dma: DMAInfo, layout: Layout) {
        self.alloc
            .dealloc(NonNull::new_unchecked(dma.cpu_addr as *mut u8), layout)
    }

    fn table_change_flag(
        &self,
        vaddr: VirtAddr,
        size: usize,
        flags: MappingFlags,
    ) -> AllocResult<()> {
        let mut aspace = axmm::kernel_aspace().lock();
        aspace.protect(vaddr, size, flags).map_err(|e| {
            error!("change table flag fail: {e:?}");
            AllocError::MemoryOverlap
        })?;
        Ok(())
    }
}
