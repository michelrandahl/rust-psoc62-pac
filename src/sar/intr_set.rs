#[doc = "Register `INTR_SET` reader"]
pub type R = crate::R<INTR_SET_SPEC>;
#[doc = "Register `INTR_SET` writer"]
pub type W = crate::W<INTR_SET_SPEC>;
#[doc = "Field `EOS_SET` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type EOS_SET_R = crate::BitReader;
#[doc = "Field `EOS_SET` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type EOS_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFLOW_SET` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type OVERFLOW_SET_R = crate::BitReader;
#[doc = "Field `OVERFLOW_SET` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type OVERFLOW_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FW_COLLISION_SET` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type FW_COLLISION_SET_R = crate::BitReader;
#[doc = "Field `FW_COLLISION_SET` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type FW_COLLISION_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSI_COLLISION_SET` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type DSI_COLLISION_SET_R = crate::BitReader;
#[doc = "Field `DSI_COLLISION_SET` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type DSI_COLLISION_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INJ_EOC_SET` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type INJ_EOC_SET_R = crate::BitReader;
#[doc = "Field `INJ_EOC_SET` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type INJ_EOC_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INJ_SATURATE_SET` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type INJ_SATURATE_SET_R = crate::BitReader;
#[doc = "Field `INJ_SATURATE_SET` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type INJ_SATURATE_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INJ_RANGE_SET` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type INJ_RANGE_SET_R = crate::BitReader;
#[doc = "Field `INJ_RANGE_SET` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type INJ_RANGE_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INJ_COLLISION_SET` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type INJ_COLLISION_SET_R = crate::BitReader;
#[doc = "Field `INJ_COLLISION_SET` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type INJ_COLLISION_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn eos_set(&self) -> EOS_SET_R {
        EOS_SET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn overflow_set(&self) -> OVERFLOW_SET_R {
        OVERFLOW_SET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn fw_collision_set(&self) -> FW_COLLISION_SET_R {
        FW_COLLISION_SET_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn dsi_collision_set(&self) -> DSI_COLLISION_SET_R {
        DSI_COLLISION_SET_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_eoc_set(&self) -> INJ_EOC_SET_R {
        INJ_EOC_SET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_saturate_set(&self) -> INJ_SATURATE_SET_R {
        INJ_SATURATE_SET_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_range_set(&self) -> INJ_RANGE_SET_R {
        INJ_RANGE_SET_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_collision_set(&self) -> INJ_COLLISION_SET_R {
        INJ_COLLISION_SET_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn eos_set(&mut self) -> EOS_SET_W<INTR_SET_SPEC> {
        EOS_SET_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn overflow_set(&mut self) -> OVERFLOW_SET_W<INTR_SET_SPEC> {
        OVERFLOW_SET_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn fw_collision_set(&mut self) -> FW_COLLISION_SET_W<INTR_SET_SPEC> {
        FW_COLLISION_SET_W::new(self, 2)
    }
    #[doc = "Bit 3 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn dsi_collision_set(&mut self) -> DSI_COLLISION_SET_W<INTR_SET_SPEC> {
        DSI_COLLISION_SET_W::new(self, 3)
    }
    #[doc = "Bit 4 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn inj_eoc_set(&mut self) -> INJ_EOC_SET_W<INTR_SET_SPEC> {
        INJ_EOC_SET_W::new(self, 4)
    }
    #[doc = "Bit 5 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn inj_saturate_set(&mut self) -> INJ_SATURATE_SET_W<INTR_SET_SPEC> {
        INJ_SATURATE_SET_W::new(self, 5)
    }
    #[doc = "Bit 6 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn inj_range_set(&mut self) -> INJ_RANGE_SET_W<INTR_SET_SPEC> {
        INJ_RANGE_SET_W::new(self, 6)
    }
    #[doc = "Bit 7 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn inj_collision_set(&mut self) -> INJ_COLLISION_SET_W<INTR_SET_SPEC> {
        INJ_COLLISION_SET_W::new(self, 7)
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
#[doc = "Interrupt set request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_SET_SPEC;
impl crate::RegisterSpec for INTR_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_set::R`](R) reader structure"]
impl crate::Readable for INTR_SET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr_set::W`](W) writer structure"]
impl crate::Writable for INTR_SET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_SET to value 0"]
impl crate::Resettable for INTR_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
