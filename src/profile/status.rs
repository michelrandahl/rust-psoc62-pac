#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `WIN_ACTIVE` reader - Indicates if the profiling time window is active. '0': Not active. '1': Active."]
pub type WIN_ACTIVE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Indicates if the profiling time window is active. '0': Not active. '1': Active."]
    #[inline(always)]
    pub fn win_active(&self) -> WIN_ACTIVE_R {
        WIN_ACTIVE_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Profile status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
