#[doc = "Register `DESCR_X_SIZE` reader"]
pub type R = crate::R<DESCR_X_SIZE_SPEC>;
#[doc = "Field `X_COUNT` reader - N/A"]
pub type X_COUNT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - N/A"]
    #[inline(always)]
    pub fn x_count(&self) -> X_COUNT_R {
        X_COUNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Channel descriptor X size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`descr_x_size::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DESCR_X_SIZE_SPEC;
impl crate::RegisterSpec for DESCR_X_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`descr_x_size::R`](R) reader structure"]
impl crate::Readable for DESCR_X_SIZE_SPEC {}
#[doc = "`reset()` method sets DESCR_X_SIZE to value 0"]
impl crate::Resettable for DESCR_X_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
