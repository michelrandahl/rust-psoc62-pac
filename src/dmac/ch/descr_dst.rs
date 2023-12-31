#[doc = "Register `DESCR_DST` reader"]
pub type R = crate::R<DESCR_DST_SPEC>;
#[doc = "Field `ADDR` reader - N/A"]
pub type ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - N/A"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
#[doc = "Channel descriptor destination\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`descr_dst::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DESCR_DST_SPEC;
impl crate::RegisterSpec for DESCR_DST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`descr_dst::R`](R) reader structure"]
impl crate::Readable for DESCR_DST_SPEC {}
#[doc = "`reset()` method sets DESCR_DST to value 0"]
impl crate::Resettable for DESCR_DST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
