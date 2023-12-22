#[doc = "Register `DST` reader"]
pub type R = crate::R<DST_SPEC>;
#[doc = "Field `ADDR` reader - Current address of destination location."]
pub type ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Current address of destination location."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
#[doc = "Channel current destination address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DST_SPEC;
impl crate::RegisterSpec for DST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dst::R`](R) reader structure"]
impl crate::Readable for DST_SPEC {}
#[doc = "`reset()` method sets DST to value 0"]
impl crate::Resettable for DST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
