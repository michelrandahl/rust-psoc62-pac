#[doc = r"Register block"]
#[repr(C)]
pub struct MPU {
    ms_ctl: MS_CTL,
    ms_ctl_read_mir: [MS_CTL_READ_MIR; 127],
    mpu_struct: (),
}
impl MPU {
    #[doc = "0x00 - Master control"]
    #[inline(always)]
    pub const fn ms_ctl(&self) -> &MS_CTL {
        &self.ms_ctl
    }
    #[doc = "0x04..0x200 - Master control read mirror"]
    #[inline(always)]
    pub const fn ms_ctl_read_mir(&self, n: usize) -> &MS_CTL_READ_MIR {
        &self.ms_ctl_read_mir[n]
    }
    #[doc = "0x200..0x240 - MPU structure"]
    #[inline(always)]
    pub const fn mpu_struct(&self, n: usize) -> &MPU_STRUCT {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(512)
                .add(32 * n)
                .cast()
        }
    }
}
#[doc = "MS_CTL (rw) register accessor: Master control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ms_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ms_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms_ctl`]
module"]
pub type MS_CTL = crate::Reg<ms_ctl::MS_CTL_SPEC>;
#[doc = "Master control"]
pub mod ms_ctl;
#[doc = "MS_CTL_READ_MIR (r) register accessor: Master control read mirror\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ms_ctl_read_mir::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms_ctl_read_mir`]
module"]
pub type MS_CTL_READ_MIR = crate::Reg<ms_ctl_read_mir::MS_CTL_READ_MIR_SPEC>;
#[doc = "Master control read mirror"]
pub mod ms_ctl_read_mir;
#[doc = "MPU structure"]
pub use self::mpu_struct::MPU_STRUCT;
#[doc = r"Cluster"]
#[doc = "MPU structure"]
pub mod mpu_struct;
