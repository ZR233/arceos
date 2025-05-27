pub fn start_secondary_cpu(cpu_idx: usize, second_cpu_idx: usize) {
    #[cfg(plat_dyn)]
    start_secondary_cpu_dyn(cpu_idx, second_cpu_idx);
    #[cfg(not(plat_dyn))]
    start_secondary_cpu_static(cpu_idx, second_cpu_idx);
}

#[cfg(plat_dyn)]
fn start_secondary_cpu_dyn(cpu_idx: usize, _second_cpu_idx: usize) {
    let cpu_id = axplat_dyn::mem::cpu_idx_to_id(cpu_idx.into());
    axplat_dyn::mp::cpu_on(cpu_id);
}

#[cfg(not(plat_dyn))]
fn start_secondary_cpu_static(cpu_idx: usize, second_cpu_idx: usize) {
    use axconfig::{SMP, TASK_STACK_SIZE};
    use memory_addr::VirtAddr;

    use crate::mem::virt_to_phys;

    #[allow(unused)]
    #[unsafe(link_section = ".bss.stack")]
    static mut SECONDARY_BOOT_STACK: [[u8; TASK_STACK_SIZE]; SMP - 1] =
        [[0; TASK_STACK_SIZE]; SMP - 1];

    let stack_top = virt_to_phys(VirtAddr::from(unsafe {
        SECONDARY_BOOT_STACK[second_cpu_idx].as_ptr_range().end as usize
    }));

    crate::platform::mp::start_secondary_cpu(cpu_idx, stack_top);
}
