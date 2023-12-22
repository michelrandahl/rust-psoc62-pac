#[doc = "Register `CLK_PLL_STATUS[%s]` reader"]
pub type R = crate::R<CLK_PLL_STATUS_SPEC>;
#[doc = "Register `CLK_PLL_STATUS[%s]` writer"]
pub type W = crate::W<CLK_PLL_STATUS_SPEC>;
#[doc = "Field `LOCKED` reader - PLL Lock Indicator"]
pub type LOCKED_R = crate::BitReader;
#[doc = "Field `UNLOCK_OCCURRED` reader - This bit sets whenever the PLL Lock bit goes low, and stays set until cleared by firmware."]
pub type UNLOCK_OCCURRED_R = crate::BitReader;
#[doc = "Field `UNLOCK_OCCURRED` writer - This bit sets whenever the PLL Lock bit goes low, and stays set until cleared by firmware."]
pub type UNLOCK_OCCURRED_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PLL Lock Indicator"]
    #[inline(always)]
    pub fn locked(&self) -> LOCKED_R {
        LOCKED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit sets whenever the PLL Lock bit goes low, and stays set until cleared by firmware."]
    #[inline(always)]
    pub fn unlock_occurred(&self) -> UNLOCK_OCCURRED_R {
        UNLOCK_OCCURRED_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - This bit sets whenever the PLL Lock bit goes low, and stays set until cleared by firmware."]
    #[inline(always)]
    #[must_use]
    pub fn unlock_occurred(&mut self) -> UNLOCK_OCCURRED_W<CLK_PLL_STATUS_SPEC> {
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
#[doc = "PLL Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_pll_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_pll_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_PLL_STATUS_SPEC;
impl crate::RegisterSpec for CLK_PLL_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_pll_status::R`](R) reader structure"]
impl crate::Readable for CLK_PLL_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_pll_status::W`](W) writer structure"]
impl crate::Writable for CLK_PLL_STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_PLL_STATUS[%s]
to value 0"]
impl crate::Resettable for CLK_PLL_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
