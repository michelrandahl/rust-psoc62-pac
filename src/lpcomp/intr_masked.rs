#[doc = "Register `INTR_MASKED` reader"]
pub type R = crate::R<INTR_MASKED_SPEC>;
#[doc = "Field `COMP0_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type COMP0_MASKED_R = crate::BitReader;
#[doc = "Field `COMP1_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type COMP1_MASKED_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn comp0_masked(&self) -> COMP0_MASKED_R {
        COMP0_MASKED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn comp1_masked(&self) -> COMP1_MASKED_R {
        COMP1_MASKED_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "LPCOMP Interrupt request masked\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_masked::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
