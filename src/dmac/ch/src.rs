#[doc = "Register `SRC` reader"]
pub type R = crate::R<SRC_SPEC>;
#[doc = "Field `ADDR` reader - Current address of source location."]
pub type ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Current address of source location."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
#[doc = "Channel current source address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRC_SPEC;
impl crate::RegisterSpec for SRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`src::R`](R) reader structure"]
impl crate::Readable for SRC_SPEC {}
#[doc = "`reset()` method sets SRC to value 0"]
impl crate::Resettable for SRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
