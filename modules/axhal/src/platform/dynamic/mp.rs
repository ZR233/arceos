use axplat_dyn::mem::cpu_idx_to_id;

pub fn start_secondary_cpu(cpu_idx: usize) {
    let cpu_id = cpu_idx_to_id(cpu_idx.into());
    axplat_dyn::mp::cpu_on(cpu_id);
}
