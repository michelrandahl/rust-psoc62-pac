#[doc = "Register `PENDING2` reader"]
pub type R = crate::R<PENDING2_SPEC>;
#[doc = "Field `SOURCE` reader - This field specifies the following sources: Bit 0 - 31: See STATUS register."]
pub type SOURCE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field specifies the following sources: Bit 0 - 31: See STATUS register."]
    #[inline(always)]
    pub fn source(&self) -> SOURCE_R {
        SOURCE_R::new(self.bits)
    }
}
#[doc = "Fault pending 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pending2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PENDING2_SPEC;
impl crate::RegisterSpec for PENDING2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pending2::R`](R) reader structure"]
impl crate::Readable for PENDING2_SPEC {}
#[doc = "`reset()` method sets PENDING2 to value 0"]
impl crate::Resettable for PENDING2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
