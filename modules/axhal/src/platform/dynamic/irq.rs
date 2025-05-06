pub use crate::arch::dispatch_irq;
use crate::irq::IrqHandler;
use alloc::boxed::Box;
use somehal::{
    driver::{
        DeviceId,
        intc::{Hardware, HardwareCPU, IrqConfig},
    },
    mem::cpu_id,
};
/// The maximum number of IRQs.
pub const MAX_IRQ_COUNT: usize = 2048;

static mut IRQ_CHIP: u64 = 0;

pub(crate) unsafe fn init() {
    let ls = somehal::driver::read(|m| m.intc.all());
    let (id, chip) = ls.first().unwrap();

    unsafe { IRQ_CHIP = (*id).into() };

    crate::platform::irq::set_enable(somehal::systime::get().irq(), true);
}

pub(crate) fn cpu_interface() -> &'static HardwareCPU {
    somehal::irq::interface(unsafe { IRQ_CHIP }.into()).expect("no cpu interface")
}

fn modify_chip<F: Fn(&mut Hardware)>(f: F) {
    let mut g = somehal::driver::intc_get(unsafe { IRQ_CHIP.into() })
        .as_ref()
        .unwrap()
        .upgrade()
        .unwrap()
        .spin_try_borrow_by(0.into());

    (f)(&mut g);
}

/// Enables or disables the given IRQ.
pub fn set_enable(irq: IrqConfig, enabled: bool) {
    trace!("GICD set enable: {:?} {}", irq, enabled);
    modify_chip(|c| {
        c.set_target_cpu(irq.irq, cpu_id().raw().into());
        c.set_trigger(irq.irq, irq.trigger);
        if enabled {
            c.irq_enable(irq.irq);
        } else {
            c.irq_disable(irq.irq);
        }
    });
}

/// Registers an IRQ handler for the given IRQ.
///
/// It also enables the IRQ if the registration succeeds. It returns `false` if
/// the registration failed.
pub fn register_handler(irq_config: IrqConfig, handler: IrqHandler) -> bool {
    trace!("register handler irq {:?}", irq_config);
    crate::irq::register_handler_common(irq_config, handler)
}
