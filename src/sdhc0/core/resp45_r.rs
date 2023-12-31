#[doc = "Register `RESP45_R` reader"]
pub type R = crate::R<RESP45_R_SPEC>;
#[doc = "Field `RESP45` reader - Command Response These bits reflect 103-72 bits of the Response Field."]
pub type RESP45_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Command Response These bits reflect 103-72 bits of the Response Field."]
    #[inline(always)]
    pub fn resp45(&self) -> RESP45_R {
        RESP45_R::new(self.bits)
    }
}
#[doc = "Response Register 4/5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp45_r::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESP45_R_SPEC;
impl crate::RegisterSpec for RESP45_R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp45_r::R`](R) reader structure"]
impl crate::Readable for RESP45_R_SPEC {}
#[doc = "`reset()` method sets RESP45_R to value 0"]
impl crate::Resettable for RESP45_R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
