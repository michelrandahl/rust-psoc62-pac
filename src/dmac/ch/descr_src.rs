#[doc = "Register `DESCR_SRC` reader"]
pub type R = crate::R<DESCR_SRC_SPEC>;
#[doc = "Field `ADDR` reader - N/A"]
pub type ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - N/A"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
#[doc = "Channel descriptor source\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`descr_src::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DESCR_SRC_SPEC;
impl crate::RegisterSpec for DESCR_SRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`descr_src::R`](R) reader structure"]
impl crate::Readable for DESCR_SRC_SPEC {}
#[doc = "`reset()` method sets DESCR_SRC to value 0"]
impl crate::Resettable for DESCR_SRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
