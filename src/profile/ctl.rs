#[doc = "Register `CTL` reader"]
pub type R = crate::R<CTL_SPEC>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Field `WIN_MODE` reader - Specifies the profiling time window mode: '0': Start / stop mode. The profiling time window is started when a rising edge of the start trigger signal occurs and stopped when a rising edge of the stop trigger signal occurs. In case both rising edges (of start and stop trigger signals) occur in the same cycle, the profiling time window is stopped. '1': Enable mode. The profiling time window is active as long as the start 'trigger' signal is active. The stop trigger signal has no effect."]
pub type WIN_MODE_R = crate::BitReader;
#[doc = "Field `WIN_MODE` writer - Specifies the profiling time window mode: '0': Start / stop mode. The profiling time window is started when a rising edge of the start trigger signal occurs and stopped when a rising edge of the stop trigger signal occurs. In case both rising edges (of start and stop trigger signals) occur in the same cycle, the profiling time window is stopped. '1': Enable mode. The profiling time window is active as long as the start 'trigger' signal is active. The stop trigger signal has no effect."]
pub type WIN_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLED` reader - Enables the profiling block: '0': Disabled. '1': Enabled."]
pub type ENABLED_R = crate::BitReader;
#[doc = "Field `ENABLED` writer - Enables the profiling block: '0': Disabled. '1': Enabled."]
pub type ENABLED_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Specifies the profiling time window mode: '0': Start / stop mode. The profiling time window is started when a rising edge of the start trigger signal occurs and stopped when a rising edge of the stop trigger signal occurs. In case both rising edges (of start and stop trigger signals) occur in the same cycle, the profiling time window is stopped. '1': Enable mode. The profiling time window is active as long as the start 'trigger' signal is active. The stop trigger signal has no effect."]
    #[inline(always)]
    pub fn win_mode(&self) -> WIN_MODE_R {
        WIN_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 31 - Enables the profiling block: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Specifies the profiling time window mode: '0': Start / stop mode. The profiling time window is started when a rising edge of the start trigger signal occurs and stopped when a rising edge of the stop trigger signal occurs. In case both rising edges (of start and stop trigger signals) occur in the same cycle, the profiling time window is stopped. '1': Enable mode. The profiling time window is active as long as the start 'trigger' signal is active. The stop trigger signal has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn win_mode(&mut self) -> WIN_MODE_W<CTL_SPEC> {
        WIN_MODE_W::new(self, 0)
    }
    #[doc = "Bit 31 - Enables the profiling block: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> ENABLED_W<CTL_SPEC> {
        ENABLED_W::new(self, 31)
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
#[doc = "Profile control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
