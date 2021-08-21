// cherrykernel's apic module
// apic thing or something, doesn't work right now

/*
use x2apic::lapic::{LocalApic, LocalApicBuilder, xapic_base};

pub fn apic() {
    let apic_physical_address: u64 = unsafe { xapic_base() };
    let apic_virtual_address: u64 =

    let lapic = LocalApicBuilder::new()
        .timer_vector(timer_index)
        .error_vector(error_index)
        .spurious_vector(spurious_index)
        .set_xapic_base(apic_virtual_address)
        .build()
        .unwrap_or_else(|err| panic!("{}", err));

    unsafe {
        lapic.enable();
    }
}
*/
