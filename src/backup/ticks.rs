#[doc = "Register `TICKS` reader"]
pub type R = crate::R<TICKS_SPEC>;
#[doc = "Field `CNT128HZ` reader - 128Hz counter (msb=2Hz) When SECONDS is written this field will be reset."]
pub type CNT128HZ_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - 128Hz counter (msb=2Hz) When SECONDS is written this field will be reset."]
    #[inline(always)]
    pub fn cnt128hz(&self) -> CNT128HZ_R {
        CNT128HZ_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "128Hz tick counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ticks::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TICKS_SPEC;
impl crate::RegisterSpec for TICKS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ticks::R`](R) reader structure"]
impl crate::Readable for TICKS_SPEC {}
#[doc = "`reset()` method sets TICKS to value 0"]
impl crate::Resettable for TICKS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
