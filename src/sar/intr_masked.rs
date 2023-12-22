#[doc = "Register `INTR_MASKED` reader"]
pub type R = crate::R<INTR_MASKED_SPEC>;
#[doc = "Field `EOS_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type EOS_MASKED_R = crate::BitReader;
#[doc = "Field `OVERFLOW_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type OVERFLOW_MASKED_R = crate::BitReader;
#[doc = "Field `FW_COLLISION_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type FW_COLLISION_MASKED_R = crate::BitReader;
#[doc = "Field `DSI_COLLISION_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type DSI_COLLISION_MASKED_R = crate::BitReader;
#[doc = "Field `INJ_EOC_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type INJ_EOC_MASKED_R = crate::BitReader;
#[doc = "Field `INJ_SATURATE_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type INJ_SATURATE_MASKED_R = crate::BitReader;
#[doc = "Field `INJ_RANGE_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type INJ_RANGE_MASKED_R = crate::BitReader;
#[doc = "Field `INJ_COLLISION_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type INJ_COLLISION_MASKED_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn eos_masked(&self) -> EOS_MASKED_R {
        EOS_MASKED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn overflow_masked(&self) -> OVERFLOW_MASKED_R {
        OVERFLOW_MASKED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn fw_collision_masked(&self) -> FW_COLLISION_MASKED_R {
        FW_COLLISION_MASKED_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn dsi_collision_masked(&self) -> DSI_COLLISION_MASKED_R {
        DSI_COLLISION_MASKED_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn inj_eoc_masked(&self) -> INJ_EOC_MASKED_R {
        INJ_EOC_MASKED_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn inj_saturate_masked(&self) -> INJ_SATURATE_MASKED_R {
        INJ_SATURATE_MASKED_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn inj_range_masked(&self) -> INJ_RANGE_MASKED_R {
        INJ_RANGE_MASKED_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn inj_collision_masked(&self) -> INJ_COLLISION_MASKED_R {
        INJ_COLLISION_MASKED_R::new(((self.bits >> 7) & 1) != 0)
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
