#[doc = "Register `DESCR_Y_SIZE` reader"]
pub type R = crate::R<DESCR_Y_SIZE_SPEC>;
#[doc = "Field `Y_COUNT` reader - N/A"]
pub type Y_COUNT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - N/A"]
    #[inline(always)]
    pub fn y_count(&self) -> Y_COUNT_R {
        Y_COUNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Channel descriptor Y size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`descr_y_size::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DESCR_Y_SIZE_SPEC;
impl crate::RegisterSpec for DESCR_Y_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`descr_y_size::R`](R) reader structure"]
impl crate::Readable for DESCR_Y_SIZE_SPEC {}
#[doc = "`reset()` method sets DESCR_Y_SIZE to value 0"]
impl crate::Resettable for DESCR_Y_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
