#[doc = "Register `MS_CTL_READ_MIR[%s]` reader"]
pub type R = crate::R<MS_CTL_READ_MIR_SPEC>;
#[doc = "Field `PC` reader - Read-only mirror of MS_CTL.PC"]
pub type PC_R = crate::FieldReader;
#[doc = "Field `PC_SAVED` reader - Read-only mirror of MS_CTL.PC_SAVED"]
pub type PC_SAVED_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Read-only mirror of MS_CTL.PC"]
    #[inline(always)]
    pub fn pc(&self) -> PC_R {
        PC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Read-only mirror of MS_CTL.PC_SAVED"]
    #[inline(always)]
    pub fn pc_saved(&self) -> PC_SAVED_R {
        PC_SAVED_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "Master control read mirror\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ms_ctl_read_mir::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MS_CTL_READ_MIR_SPEC;
impl crate::RegisterSpec for MS_CTL_READ_MIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ms_ctl_read_mir::R`](R) reader structure"]
impl crate::Readable for MS_CTL_READ_MIR_SPEC {}
#[doc = "`reset()` method sets MS_CTL_READ_MIR[%s]
to value 0"]
impl crate::Resettable for MS_CTL_READ_MIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
