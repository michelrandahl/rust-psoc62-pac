#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    smpu: SMPU,
    _reserved1: [u8; 0x1c18],
    mpu: (),
}
impl RegisterBlock {
    #[doc = "0x00..0x23e8 - SMPU"]
    #[inline(always)]
    pub const fn smpu(&self) -> &SMPU {
        &self.smpu
    }
    #[doc = "0x4000..0x6e80 - MPU"]
    #[inline(always)]
    pub const fn mpu(&self, n: usize) -> &MPU {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(16384)
                .add(1024 * n)
                .cast()
        }
    }
}
#[doc = "SMPU"]
pub use self::smpu::SMPU;
#[doc = r"Cluster"]
#[doc = "SMPU"]
pub mod smpu;
#[doc = "MPU"]
pub use self::mpu::MPU;
#[doc = r"Cluster"]
#[doc = "MPU"]
pub mod mpu;
