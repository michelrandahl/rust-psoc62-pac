#[doc = "Register `INTR_MASKED` reader"]
pub type R = crate::R<INTR_MASKED_SPEC>;
#[doc = "Field `FAULT` reader - Logical and of corresponding INTR and INTR_MASK fields."]
pub type FAULT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn fault(&self) -> FAULT_R {
        FAULT_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Interrupt masked\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_masked::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_MASKED_SPEC;
impl crate::RegisterSpec for INTR_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_masked::R`](R) reader structure"]
impl crate::Readable for INTR_MASKED_SPEC {}
#[doc = "`reset()` method sets INTR_MASKED to value 0"]
impl crate::Resettable for INTR_MASKED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
