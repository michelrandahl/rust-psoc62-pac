#[doc = "Register `ACTIVE` reader"]
pub type R = crate::R<ACTIVE_SPEC>;
#[doc = "Field `ACTIVE` reader - Specifies active channels; i.e. enabled channels whose trigger got activated."]
pub type ACTIVE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Specifies active channels; i.e. enabled channels whose trigger got activated."]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Active channels\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`active::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACTIVE_SPEC;
impl crate::RegisterSpec for ACTIVE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`active::R`](R) reader structure"]
impl crate::Readable for ACTIVE_SPEC {}
#[doc = "`reset()` method sets ACTIVE to value 0"]
impl crate::Resettable for ACTIVE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
