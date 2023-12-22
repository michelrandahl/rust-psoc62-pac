#[doc = "Register `INTR_SIE_MASKED` reader"]
pub type R = crate::R<INTR_SIE_MASKED_SPEC>;
#[doc = "Field `SOF_INTR_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type SOF_INTR_MASKED_R = crate::BitReader;
#[doc = "Field `BUS_RESET_INTR_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type BUS_RESET_INTR_MASKED_R = crate::BitReader;
#[doc = "Field `EP0_INTR_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type EP0_INTR_MASKED_R = crate::BitReader;
#[doc = "Field `LPM_INTR_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type LPM_INTR_MASKED_R = crate::BitReader;
#[doc = "Field `RESUME_INTR_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type RESUME_INTR_MASKED_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn sof_intr_masked(&self) -> SOF_INTR_MASKED_R {
        SOF_INTR_MASKED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn bus_reset_intr_masked(&self) -> BUS_RESET_INTR_MASKED_R {
        BUS_RESET_INTR_MASKED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn ep0_intr_masked(&self) -> EP0_INTR_MASKED_R {
        EP0_INTR_MASKED_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn lpm_intr_masked(&self) -> LPM_INTR_MASKED_R {
        LPM_INTR_MASKED_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn resume_intr_masked(&self) -> RESUME_INTR_MASKED_R {
        RESUME_INTR_MASKED_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "USB SOF, BUS RESET and EP0 Interrupt Masked\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_sie_masked::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_SIE_MASKED_SPEC;
impl crate::RegisterSpec for INTR_SIE_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_sie_masked::R`](R) reader structure"]
impl crate::Readable for INTR_SIE_MASKED_SPEC {}
#[doc = "`reset()` method sets INTR_SIE_MASKED to value 0"]
impl crate::Resettable for INTR_SIE_MASKED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
