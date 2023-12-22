#[doc = "Register `CTL` reader"]
pub type R = crate::R<CTL_SPEC>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Field `ENABLE` reader - IP Enable: 0: IP disabled, RAM in DeepSleep, SDHC_CORE regs are inaccessible (any attempts to access will result in AHB Error responses), IP is NOT held in reset but the clocks are gated 1: IP enabled, normal operation"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - IP Enable: 0: IP disabled, RAM in DeepSleep, SDHC_CORE regs are inaccessible (any attempts to access will result in AHB Error responses), IP is NOT held in reset but the clocks are gated 1: IP enabled, normal operation"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - IP Enable: 0: IP disabled, RAM in DeepSleep, SDHC_CORE regs are inaccessible (any attempts to access will result in AHB Error responses), IP is NOT held in reset but the clocks are gated 1: IP enabled, normal operation"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - IP Enable: 0: IP disabled, RAM in DeepSleep, SDHC_CORE regs are inaccessible (any attempts to access will result in AHB Error responses), IP is NOT held in reset but the clocks are gated 1: IP enabled, normal operation"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<CTL_SPEC> {
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
#[doc = "Top level wrapper control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
