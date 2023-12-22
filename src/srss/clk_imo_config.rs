#[doc = "Register `CLK_IMO_CONFIG` reader"]
pub type R = crate::R<CLK_IMO_CONFIG_SPEC>;
#[doc = "Register `CLK_IMO_CONFIG` writer"]
pub type W = crate::W<CLK_IMO_CONFIG_SPEC>;
#[doc = "Field `ENABLE` reader - Master enable for IMO oscillator. This bit must be high at all times for all functions to work properly. Hardware will automatically disable the IMO during HIBERNATE and XRES. It will automatically disable during DEEPSLEEP if DPSLP_ENABLE==0."]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Master enable for IMO oscillator. This bit must be high at all times for all functions to work properly. Hardware will automatically disable the IMO during HIBERNATE and XRES. It will automatically disable during DEEPSLEEP if DPSLP_ENABLE==0."]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - Master enable for IMO oscillator. This bit must be high at all times for all functions to work properly. Hardware will automatically disable the IMO during HIBERNATE and XRES. It will automatically disable during DEEPSLEEP if DPSLP_ENABLE==0."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Master enable for IMO oscillator. This bit must be high at all times for all functions to work properly. Hardware will automatically disable the IMO during HIBERNATE and XRES. It will automatically disable during DEEPSLEEP if DPSLP_ENABLE==0."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<CLK_IMO_CONFIG_SPEC> {
        ENABLE_W::new(self, 31)
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
#[doc = "IMO Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_imo_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_imo_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_IMO_CONFIG_SPEC;
impl crate::RegisterSpec for CLK_IMO_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_imo_config::R`](R) reader structure"]
impl crate::Readable for CLK_IMO_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_imo_config::W`](W) writer structure"]
impl crate::Writable for CLK_IMO_CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_IMO_CONFIG to value 0x8000_0000"]
impl crate::Resettable for CLK_IMO_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
