#[doc = "Register `INTR_MASKED` reader"]
pub type R = crate::R<INTR_MASKED_SPEC>;
#[doc = "Field `ALARM1` reader - Logical and of corresponding request and mask bits."]
pub type ALARM1_R = crate::BitReader;
#[doc = "Field `ALARM2` reader - Logical and of corresponding request and mask bits."]
pub type ALARM2_R = crate::BitReader;
#[doc = "Field `CENTURY` reader - Logical and of corresponding request and mask bits."]
pub type CENTURY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn alarm1(&self) -> ALARM1_R {
        ALARM1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn alarm2(&self) -> ALARM2_R {
        ALARM2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn century(&self) -> CENTURY_R {
        CENTURY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Interrupt masked request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_masked::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
