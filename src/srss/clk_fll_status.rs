#[doc = "Register `CLK_FLL_STATUS` reader"]
pub type R = crate::R<CLK_FLL_STATUS_SPEC>;
#[doc = "Register `CLK_FLL_STATUS` writer"]
pub type W = crate::W<CLK_FLL_STATUS_SPEC>;
#[doc = "Field `LOCKED` reader - FLL Lock Indicator. LOCKED is high when FLL is within CLK_FLL_CONFIG2.LOCK_TOL. If FLL is outside LOCK_TOL, LOCKED goes low. Note that this can happen during normal operation, if FLL needs to recalculate due to a change in the reference clock, change in voltage, or change in temperature."]
pub type LOCKED_R = crate::BitReader;
#[doc = "Field `UNLOCK_OCCURRED` reader - N/A"]
pub type UNLOCK_OCCURRED_R = crate::BitReader;
#[doc = "Field `UNLOCK_OCCURRED` writer - N/A"]
pub type UNLOCK_OCCURRED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCO_READY` reader - This indicates that the CCO is internally settled and ready to use."]
pub type CCO_READY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - FLL Lock Indicator. LOCKED is high when FLL is within CLK_FLL_CONFIG2.LOCK_TOL. If FLL is outside LOCK_TOL, LOCKED goes low. Note that this can happen during normal operation, if FLL needs to recalculate due to a change in the reference clock, change in voltage, or change in temperature."]
    #[inline(always)]
    pub fn locked(&self) -> LOCKED_R {
        LOCKED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn unlock_occurred(&self) -> UNLOCK_OCCURRED_R {
        UNLOCK_OCCURRED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This indicates that the CCO is internally settled and ready to use."]
    #[inline(always)]
    pub fn cco_ready(&self) -> CCO_READY_R {
        CCO_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn unlock_occurred(&mut self) -> UNLOCK_OCCURRED_W<CLK_FLL_STATUS_SPEC> {
        UNLOCK_OCCURRED_W::new(self, 1)
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
#[doc = "FLL Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_fll_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_fll_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_FLL_STATUS_SPEC;
impl crate::RegisterSpec for CLK_FLL_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_fll_status::R`](R) reader structure"]
impl crate::Readable for CLK_FLL_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_fll_status::W`](W) writer structure"]
impl crate::Writable for CLK_FLL_STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_FLL_STATUS to value 0"]
impl crate::Resettable for CLK_FLL_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
