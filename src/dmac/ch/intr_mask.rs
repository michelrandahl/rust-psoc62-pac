#[doc = "Register `INTR_MASK` reader"]
pub type R = crate::R<INTR_MASK_SPEC>;
#[doc = "Register `INTR_MASK` writer"]
pub type W = crate::W<INTR_MASK_SPEC>;
#[doc = "Field `COMPLETION` reader - Mask for corresponding field in INTR register."]
pub type COMPLETION_R = crate::BitReader;
#[doc = "Field `COMPLETION` writer - Mask for corresponding field in INTR register."]
pub type COMPLETION_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRC_BUS_ERROR` reader - N/A"]
pub type SRC_BUS_ERROR_R = crate::BitReader;
#[doc = "Field `SRC_BUS_ERROR` writer - N/A"]
pub type SRC_BUS_ERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DST_BUS_ERROR` reader - N/A"]
pub type DST_BUS_ERROR_R = crate::BitReader;
#[doc = "Field `DST_BUS_ERROR` writer - N/A"]
pub type DST_BUS_ERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRC_MISAL` reader - N/A"]
pub type SRC_MISAL_R = crate::BitReader;
#[doc = "Field `SRC_MISAL` writer - N/A"]
pub type SRC_MISAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DST_MISAL` reader - N/A"]
pub type DST_MISAL_R = crate::BitReader;
#[doc = "Field `DST_MISAL` writer - N/A"]
pub type DST_MISAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CURR_PTR_NULL` reader - N/A"]
pub type CURR_PTR_NULL_R = crate::BitReader;
#[doc = "Field `CURR_PTR_NULL` writer - N/A"]
pub type CURR_PTR_NULL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE_CH_DISABLED` reader - N/A"]
pub type ACTIVE_CH_DISABLED_R = crate::BitReader;
#[doc = "Field `ACTIVE_CH_DISABLED` writer - N/A"]
pub type ACTIVE_CH_DISABLED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DESCR_BUS_ERROR` reader - N/A"]
pub type DESCR_BUS_ERROR_R = crate::BitReader;
#[doc = "Field `DESCR_BUS_ERROR` writer - N/A"]
pub type DESCR_BUS_ERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Mask for corresponding field in INTR register."]
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
impl W {
    #[doc = "Bit 0 - Mask for corresponding field in INTR register."]
    #[inline(always)]
    #[must_use]
    pub fn completion(&mut self) -> COMPLETION_W<INTR_MASK_SPEC> {
        COMPLETION_W::new(self, 0)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn src_bus_error(&mut self) -> SRC_BUS_ERROR_W<INTR_MASK_SPEC> {
        SRC_BUS_ERROR_W::new(self, 1)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn dst_bus_error(&mut self) -> DST_BUS_ERROR_W<INTR_MASK_SPEC> {
        DST_BUS_ERROR_W::new(self, 2)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn src_misal(&mut self) -> SRC_MISAL_W<INTR_MASK_SPEC> {
        SRC_MISAL_W::new(self, 3)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn dst_misal(&mut self) -> DST_MISAL_W<INTR_MASK_SPEC> {
        DST_MISAL_W::new(self, 4)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn curr_ptr_null(&mut self) -> CURR_PTR_NULL_W<INTR_MASK_SPEC> {
        CURR_PTR_NULL_W::new(self, 5)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn active_ch_disabled(&mut self) -> ACTIVE_CH_DISABLED_W<INTR_MASK_SPEC> {
        ACTIVE_CH_DISABLED_W::new(self, 6)
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn descr_bus_error(&mut self) -> DESCR_BUS_ERROR_W<INTR_MASK_SPEC> {
        DESCR_BUS_ERROR_W::new(self, 7)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_MASK_SPEC;
impl crate::RegisterSpec for INTR_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_mask::R`](R) reader structure"]
impl crate::Readable for INTR_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr_mask::W`](W) writer structure"]
impl crate::Writable for INTR_MASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_MASK to value 0"]
impl crate::Resettable for INTR_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
