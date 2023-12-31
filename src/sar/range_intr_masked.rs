#[doc = "Register `RANGE_INTR_MASKED` reader"]
pub type R = crate::R<RANGE_INTR_MASKED_SPEC>;
#[doc = "Field `RANGE_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type RANGE_MASKED_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn range_masked(&self) -> RANGE_MASKED_R {
        RANGE_MASKED_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Range interrupt masked request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`range_intr_masked::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RANGE_INTR_MASKED_SPEC;
impl crate::RegisterSpec for RANGE_INTR_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`range_intr_masked::R`](R) reader structure"]
impl crate::Readable for RANGE_INTR_MASKED_SPEC {}
#[doc = "`reset()` method sets RANGE_INTR_MASKED to value 0"]
impl crate::Resettable for RANGE_INTR_MASKED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
