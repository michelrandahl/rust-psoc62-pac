#[doc = "Register `INTR_MASK` reader"]
pub type R = crate::R<INTR_MASK_SPEC>;
#[doc = "Register `INTR_MASK` writer"]
pub type W = crate::W<INTR_MASK_SPEC>;
#[doc = "Field `EOS_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub type EOS_MASK_R = crate::BitReader;
#[doc = "Field `EOS_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub type EOS_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFLOW_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub type OVERFLOW_MASK_R = crate::BitReader;
#[doc = "Field `OVERFLOW_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub type OVERFLOW_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FW_COLLISION_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub type FW_COLLISION_MASK_R = crate::BitReader;
#[doc = "Field `FW_COLLISION_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub type FW_COLLISION_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSI_COLLISION_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub type DSI_COLLISION_MASK_R = crate::BitReader;
#[doc = "Field `DSI_COLLISION_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub type DSI_COLLISION_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INJ_EOC_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub type INJ_EOC_MASK_R = crate::BitReader;
#[doc = "Field `INJ_EOC_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub type INJ_EOC_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INJ_SATURATE_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub type INJ_SATURATE_MASK_R = crate::BitReader;
#[doc = "Field `INJ_SATURATE_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub type INJ_SATURATE_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INJ_RANGE_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub type INJ_RANGE_MASK_R = crate::BitReader;
#[doc = "Field `INJ_RANGE_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub type INJ_RANGE_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INJ_COLLISION_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub type INJ_COLLISION_MASK_R = crate::BitReader;
#[doc = "Field `INJ_COLLISION_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub type INJ_COLLISION_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn eos_mask(&self) -> EOS_MASK_R {
        EOS_MASK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn overflow_mask(&self) -> OVERFLOW_MASK_R {
        OVERFLOW_MASK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn fw_collision_mask(&self) -> FW_COLLISION_MASK_R {
        FW_COLLISION_MASK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn dsi_collision_mask(&self) -> DSI_COLLISION_MASK_R {
        DSI_COLLISION_MASK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_eoc_mask(&self) -> INJ_EOC_MASK_R {
        INJ_EOC_MASK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_saturate_mask(&self) -> INJ_SATURATE_MASK_R {
        INJ_SATURATE_MASK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_range_mask(&self) -> INJ_RANGE_MASK_R {
        INJ_RANGE_MASK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_collision_mask(&self) -> INJ_COLLISION_MASK_R {
        INJ_COLLISION_MASK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn eos_mask(&mut self) -> EOS_MASK_W<INTR_MASK_SPEC> {
        EOS_MASK_W::new(self, 0)
    }
    #[doc = "Bit 1 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn overflow_mask(&mut self) -> OVERFLOW_MASK_W<INTR_MASK_SPEC> {
        OVERFLOW_MASK_W::new(self, 1)
    }
    #[doc = "Bit 2 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn fw_collision_mask(&mut self) -> FW_COLLISION_MASK_W<INTR_MASK_SPEC> {
        FW_COLLISION_MASK_W::new(self, 2)
    }
    #[doc = "Bit 3 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn dsi_collision_mask(&mut self) -> DSI_COLLISION_MASK_W<INTR_MASK_SPEC> {
        DSI_COLLISION_MASK_W::new(self, 3)
    }
    #[doc = "Bit 4 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn inj_eoc_mask(&mut self) -> INJ_EOC_MASK_W<INTR_MASK_SPEC> {
        INJ_EOC_MASK_W::new(self, 4)
    }
    #[doc = "Bit 5 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn inj_saturate_mask(&mut self) -> INJ_SATURATE_MASK_W<INTR_MASK_SPEC> {
        INJ_SATURATE_MASK_W::new(self, 5)
    }
    #[doc = "Bit 6 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn inj_range_mask(&mut self) -> INJ_RANGE_MASK_W<INTR_MASK_SPEC> {
        INJ_RANGE_MASK_W::new(self, 6)
    }
    #[doc = "Bit 7 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn inj_collision_mask(&mut self) -> INJ_COLLISION_MASK_W<INTR_MASK_SPEC> {
        INJ_COLLISION_MASK_W::new(self, 7)
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
#[doc = "Interrupt mask register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
