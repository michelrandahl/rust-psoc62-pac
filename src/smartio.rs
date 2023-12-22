#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    prt: (),
}
impl RegisterBlock {
    #[doc = "0x00..0x988 - Programmable IO port registers"]
    #[inline(always)]
    pub const fn prt(&self, n: usize) -> &PRT {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(0)
                .add(256 * n)
                .cast()
        }
    }
}
#[doc = "Programmable IO port registers"]
pub use self::prt::PRT;
#[doc = r"Cluster"]
#[doc = "Programmable IO port registers"]
pub mod prt;
