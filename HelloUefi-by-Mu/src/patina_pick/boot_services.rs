// use std::sync::OnceLock;

use r_efi::efi;
use spin::Once;

pub struct RawBootServices {
    efi_boot_services: Once<*mut efi::BootServices>,
}

unsafe impl Sync for RawBootServices {}
unsafe impl Send for RawBootServices {}

impl RawBootServices {
    pub fn new(efi_boot_services: *mut efi::BootServices) -> Self {
        let this = Self::new_uninit();
        this.init(efi_boot_services);
        this
    }

    pub const fn new_uninit() -> Self {
        RawBootServices { efi_boot_services: Once::new() }
    }
    pub fn init(&self, efi_boot_services: *mut efi::BootServices) {
        // This struct never mutate the efi_boot_services.
        self.efi_boot_services.call_once(|| efi_boot_services);
    }

    pub fn is_init(&self) -> bool {
        self.efi_boot_services.is_completed()
    }

    // Returns the boot services pointer if initialized, panics otherwise.
    fn as_mut_ptr(&self) -> *mut efi::BootServices {
        *self.efi_boot_services.get().expect("BOOT_SERVICES is not initialized!")
    }
}
