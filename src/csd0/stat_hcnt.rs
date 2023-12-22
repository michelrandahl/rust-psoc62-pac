#[doc = "Register `STAT_HCNT` reader"]
pub type R = crate::R<STAT_HCNT_SPEC>;
#[doc = "Field `CNT` reader - Current value of HSCMP counter"]
pub type CNT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Current value of HSCMP counter"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Current count of the HSCMP counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat_hcnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STAT_HCNT_SPEC;
impl crate::RegisterSpec for STAT_HCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat_hcnt::R`](R) reader structure"]
impl crate::Readable for STAT_HCNT_SPEC {}
#[doc = "`reset()` method sets STAT_HCNT to value 0"]
impl crate::Resettable for STAT_HCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
