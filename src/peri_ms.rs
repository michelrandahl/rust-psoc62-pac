#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ppu_pr: [PPU_PR; 8],
    _reserved1: [u8; 0x0600],
    ppu_fx: [PPU_FX; 229],
}
impl RegisterBlock {
    #[doc = "0x00..0x200 - Programmable protection structure pair"]
    #[inline(always)]
    pub const fn ppu_pr(&self, n: usize) -> &PPU_PR {
        &self.ppu_pr[n]
    }
    #[doc = "0x800..0x4140 - Fixed protection structure pair"]
    #[inline(always)]
    pub const fn ppu_fx(&self, n: usize) -> &PPU_FX {
        &self.ppu_fx[n]
    }
}
#[doc = "Programmable protection structure pair"]
pub use self::ppu_pr::PPU_PR;
#[doc = r"Cluster"]
#[doc = "Programmable protection structure pair"]
pub mod ppu_pr;
#[doc = "Fixed protection structure pair"]
pub use self::ppu_fx::PPU_FX;
#[doc = r"Cluster"]
#[doc = "Fixed protection structure pair"]
pub mod ppu_fx;
