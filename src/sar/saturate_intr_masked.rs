#[doc = "Register `SATURATE_INTR_MASKED` reader"]
pub type R = crate::R<SATURATE_INTR_MASKED_SPEC>;
#[doc = "Field `SATURATE_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type SATURATE_MASKED_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn saturate_masked(&self) -> SATURATE_MASKED_R {
        SATURATE_MASKED_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Saturate interrupt masked request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`saturate_intr_masked::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SATURATE_INTR_MASKED_SPEC;
impl crate::RegisterSpec for SATURATE_INTR_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`saturate_intr_masked::R`](R) reader structure"]
impl crate::Readable for SATURATE_INTR_MASKED_SPEC {}
#[doc = "`reset()` method sets SATURATE_INTR_MASKED to value 0"]
impl crate::Resettable for SATURATE_INTR_MASKED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
