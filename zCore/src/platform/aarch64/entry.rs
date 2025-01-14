use kernel_hal::KernelConfig;

core::arch::global_asm!(include_str!("space.s"));

#[naked]
#[no_mangle]
#[link_section = ".text.entry"]
unsafe extern "C" fn _start() -> ! {
    core::arch::asm!(
        "
        adrp    x19, boot_stack_top
        mov     sp, x19
        b rust_main",
        options(noreturn),
    )
}

#[no_mangle]
extern "C" fn rust_main() -> ! {
    let config = KernelConfig {
        rt_services_addr: 0,
        rsdp_addr: 0,
        phys_to_virt_offset: 0xffff_0000_0000_0000,
    };
    crate::primary_main(config);
    unreachable!()
}
