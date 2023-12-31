#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    wrap: WRAP,
    _reserved1: [u8; 0x0ffc],
    core: CORE,
}
impl RegisterBlock {
    #[doc = "0x00 - MMIO at SDHC wrapper level"]
    #[inline(always)]
    pub const fn wrap(&self) -> &WRAP {
        &self.wrap
    }
    #[doc = "0x1000..0x1538 - MMIO for Synopsys Mobile Storage Host Controller IP"]
    #[inline(always)]
    pub const fn core(&self) -> &CORE {
        &self.core
    }
}
#[doc = "MMIO at SDHC wrapper level"]
pub use self::wrap::WRAP;
#[doc = r"Cluster"]
#[doc = "MMIO at SDHC wrapper level"]
pub mod wrap;
#[doc = "MMIO for Synopsys Mobile Storage Host Controller IP"]
pub use self::core::CORE;
#[doc = r"Cluster"]
#[doc = "MMIO for Synopsys Mobile Storage Host Controller IP"]
pub mod core;
