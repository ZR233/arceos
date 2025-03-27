use core::ptr::NonNull;

use crate::mem::phys_to_virt;

pub fn test(dtb: usize) -> Option<()> {
    let dtb = NonNull::new(phys_to_virt(dtb.into()).as_mut_ptr())?;

    let mut uart = any_uart::init(dtb, |phys| phys_to_virt(phys.into()).as_mut_ptr())?;

    uart.tx.as_mut()?.write_str_blocking("uart test\r\n");

    Some(())
}
