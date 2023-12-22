#[doc = "Register `INTR_MASKED` reader"]
pub type R = crate::R<INTR_MASKED_SPEC>;
#[doc = "Field `COMPLETION` reader - Logical and of corresponding INTR and INTR_MASK fields."]
pub type COMPLETION_R = crate::BitReader;
#[doc = "Field `SRC_BUS_ERROR` reader - N/A"]
pub type SRC_BUS_ERROR_R = crate::BitReader;
#[doc = "Field `DST_BUS_ERROR` reader - N/A"]
pub type DST_BUS_ERROR_R = crate::BitReader;
#[doc = "Field `SRC_MISAL` reader - N/A"]
pub type SRC_MISAL_R = crate::BitReader;
#[doc = "Field `DST_MISAL` reader - N/A"]
pub type DST_MISAL_R = crate::BitReader;
#[doc = "Field `CURR_PTR_NULL` reader - N/A"]
pub type CURR_PTR_NULL_R = crate::BitReader;
#[doc = "Field `ACTIVE_CH_DISABLED` reader - N/A"]
pub type ACTIVE_CH_DISABLED_R = crate::BitReader;
#[doc = "Field `DESCR_BUS_ERROR` reader - N/A"]
pub type DESCR_BUS_ERROR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn completion(&self) -> COMPLETION_R {
        COMPLETION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn src_bus_error(&self) -> SRC_BUS_ERROR_R {
        SRC_BUS_ERROR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn dst_bus_error(&self) -> DST_BUS_ERROR_R {
        DST_BUS_ERROR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn src_misal(&self) -> SRC_MISAL_R {
        SRC_MISAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn dst_misal(&self) -> DST_MISAL_R {
        DST_MISAL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn curr_ptr_null(&self) -> CURR_PTR_NULL_R {
        CURR_PTR_NULL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn active_ch_disabled(&self) -> ACTIVE_CH_DISABLED_R {
        ACTIVE_CH_DISABLED_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn descr_bus_error(&self) -> DESCR_BUS_ERROR_R {
        DESCR_BUS_ERROR_R::new(((self.bits >> 7) & 1) != 0)
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
