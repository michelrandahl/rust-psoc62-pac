#[doc = "Register `MCWDT_INTR_MASKED` reader"]
pub type R = crate::R<MCWDT_INTR_MASKED_SPEC>;
#[doc = "Field `MCWDT_INT0` reader - Logical and of corresponding request and mask bits."]
pub type MCWDT_INT0_R = crate::BitReader;
#[doc = "Field `MCWDT_INT1` reader - Logical and of corresponding request and mask bits."]
pub type MCWDT_INT1_R = crate::BitReader;
#[doc = "Field `MCWDT_INT2` reader - Logical and of corresponding request and mask bits."]
pub type MCWDT_INT2_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn mcwdt_int0(&self) -> MCWDT_INT0_R {
        MCWDT_INT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn mcwdt_int1(&self) -> MCWDT_INT1_R {
        MCWDT_INT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn mcwdt_int2(&self) -> MCWDT_INT2_R {
        MCWDT_INT2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Multi-Counter Watchdog Counter Interrupt Masked Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcwdt_intr_masked::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCWDT_INTR_MASKED_SPEC;
impl crate::RegisterSpec for MCWDT_INTR_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcwdt_intr_masked::R`](R) reader structure"]
impl crate::Readable for MCWDT_INTR_MASKED_SPEC {}
#[doc = "`reset()` method sets MCWDT_INTR_MASKED to value 0"]
impl crate::Resettable for MCWDT_INTR_MASKED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
