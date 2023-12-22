#[doc = "Register `MCWDT_CTL` reader"]
pub type R = crate::R<MCWDT_CTL_SPEC>;
#[doc = "Register `MCWDT_CTL` writer"]
pub type W = crate::W<MCWDT_CTL_SPEC>;
#[doc = "Field `WDT_ENABLE0` reader - Enable subcounter 0. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
pub type WDT_ENABLE0_R = crate::BitReader;
#[doc = "Field `WDT_ENABLE0` writer - Enable subcounter 0. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
pub type WDT_ENABLE0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_ENABLED0` reader - Indicates actual state of counter. May lag WDT_ENABLE0 by up to two LFCLK cycles."]
pub type WDT_ENABLED0_R = crate::BitReader;
#[doc = "Field `WDT_RESET0` reader - Resets counter 0 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
pub type WDT_RESET0_R = crate::BitReader;
#[doc = "Field `WDT_RESET0` writer - Resets counter 0 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
pub type WDT_RESET0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_ENABLE1` reader - Enable subcounter 1. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
pub type WDT_ENABLE1_R = crate::BitReader;
#[doc = "Field `WDT_ENABLE1` writer - Enable subcounter 1. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
pub type WDT_ENABLE1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_ENABLED1` reader - Indicates actual state of counter. May lag WDT_ENABLE1 by up to two LFCLK cycles."]
pub type WDT_ENABLED1_R = crate::BitReader;
#[doc = "Field `WDT_RESET1` reader - Resets counter 0 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
pub type WDT_RESET1_R = crate::BitReader;
#[doc = "Field `WDT_RESET1` writer - Resets counter 0 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
pub type WDT_RESET1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_ENABLE2` reader - Enable subcounter 2. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
pub type WDT_ENABLE2_R = crate::BitReader;
#[doc = "Field `WDT_ENABLE2` writer - Enable subcounter 2. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
pub type WDT_ENABLE2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_ENABLED2` reader - Indicates actual state of counter. May lag WDT_ENABLE2 by up to two LFCLK cycles."]
pub type WDT_ENABLED2_R = crate::BitReader;
#[doc = "Field `WDT_RESET2` reader - Resets counter 0 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
pub type WDT_RESET2_R = crate::BitReader;
#[doc = "Field `WDT_RESET2` writer - Resets counter 0 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
pub type WDT_RESET2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable subcounter 0. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
    #[inline(always)]
    pub fn wdt_enable0(&self) -> WDT_ENABLE0_R {
        WDT_ENABLE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates actual state of counter. May lag WDT_ENABLE0 by up to two LFCLK cycles."]
    #[inline(always)]
    pub fn wdt_enabled0(&self) -> WDT_ENABLED0_R {
        WDT_ENABLED0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Resets counter 0 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
    #[inline(always)]
    pub fn wdt_reset0(&self) -> WDT_RESET0_R {
        WDT_RESET0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable subcounter 1. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
    #[inline(always)]
    pub fn wdt_enable1(&self) -> WDT_ENABLE1_R {
        WDT_ENABLE1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Indicates actual state of counter. May lag WDT_ENABLE1 by up to two LFCLK cycles."]
    #[inline(always)]
    pub fn wdt_enabled1(&self) -> WDT_ENABLED1_R {
        WDT_ENABLED1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Resets counter 0 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
    #[inline(always)]
    pub fn wdt_reset1(&self) -> WDT_RESET1_R {
        WDT_RESET1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable subcounter 2. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
    #[inline(always)]
    pub fn wdt_enable2(&self) -> WDT_ENABLE2_R {
        WDT_ENABLE2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Indicates actual state of counter. May lag WDT_ENABLE2 by up to two LFCLK cycles."]
    #[inline(always)]
    pub fn wdt_enabled2(&self) -> WDT_ENABLED2_R {
        WDT_ENABLED2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Resets counter 0 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
    #[inline(always)]
    pub fn wdt_reset2(&self) -> WDT_RESET2_R {
        WDT_RESET2_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable subcounter 0. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_enable0(&mut self) -> WDT_ENABLE0_W<MCWDT_CTL_SPEC> {
        WDT_ENABLE0_W::new(self, 0)
    }
    #[doc = "Bit 3 - Resets counter 0 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_reset0(&mut self) -> WDT_RESET0_W<MCWDT_CTL_SPEC> {
        WDT_RESET0_W::new(self, 3)
    }
    #[doc = "Bit 8 - Enable subcounter 1. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_enable1(&mut self) -> WDT_ENABLE1_W<MCWDT_CTL_SPEC> {
        WDT_ENABLE1_W::new(self, 8)
    }
    #[doc = "Bit 11 - Resets counter 0 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_reset1(&mut self) -> WDT_RESET1_W<MCWDT_CTL_SPEC> {
        WDT_RESET1_W::new(self, 11)
    }
    #[doc = "Bit 16 - Enable subcounter 2. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_enable2(&mut self) -> WDT_ENABLE2_W<MCWDT_CTL_SPEC> {
        WDT_ENABLE2_W::new(self, 16)
    }
    #[doc = "Bit 19 - Resets counter 0 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_reset2(&mut self) -> WDT_RESET2_W<MCWDT_CTL_SPEC> {
        WDT_RESET2_W::new(self, 19)
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
#[doc = "Multi-Counter Watchdog Counter Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcwdt_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcwdt_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCWDT_CTL_SPEC;
impl crate::RegisterSpec for MCWDT_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcwdt_ctl::R`](R) reader structure"]
impl crate::Readable for MCWDT_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcwdt_ctl::W`](W) writer structure"]
impl crate::Writable for MCWDT_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCWDT_CTL to value 0"]
impl crate::Resettable for MCWDT_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
