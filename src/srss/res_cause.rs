#[doc = "Register `RES_CAUSE` reader"]
pub type R = crate::R<RES_CAUSE_SPEC>;
#[doc = "Register `RES_CAUSE` writer"]
pub type W = crate::W<RES_CAUSE_SPEC>;
#[doc = "Field `RESET_WDT` reader - A basic WatchDog Timer (WDT) reset has occurred since last power cycle."]
pub type RESET_WDT_R = crate::BitReader;
#[doc = "Field `RESET_WDT` writer - A basic WatchDog Timer (WDT) reset has occurred since last power cycle."]
pub type RESET_WDT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET_ACT_FAULT` reader - Fault logging system requested a reset from its Active logic."]
pub type RESET_ACT_FAULT_R = crate::BitReader;
#[doc = "Field `RESET_ACT_FAULT` writer - Fault logging system requested a reset from its Active logic."]
pub type RESET_ACT_FAULT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET_DPSLP_FAULT` reader - Fault logging system requested a reset from its DeepSleep logic."]
pub type RESET_DPSLP_FAULT_R = crate::BitReader;
#[doc = "Field `RESET_DPSLP_FAULT` writer - Fault logging system requested a reset from its DeepSleep logic."]
pub type RESET_DPSLP_FAULT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET_CSV_WCO_LOSS` reader - Clock supervision logic requested a reset due to loss of a watch-crystal clock."]
pub type RESET_CSV_WCO_LOSS_R = crate::BitReader;
#[doc = "Field `RESET_CSV_WCO_LOSS` writer - Clock supervision logic requested a reset due to loss of a watch-crystal clock."]
pub type RESET_CSV_WCO_LOSS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET_SOFT` reader - A CPU requested a system reset through it's SYSRESETREQ. This can be done via a debugger probe or in firmware."]
pub type RESET_SOFT_R = crate::BitReader;
#[doc = "Field `RESET_SOFT` writer - A CPU requested a system reset through it's SYSRESETREQ. This can be done via a debugger probe or in firmware."]
pub type RESET_SOFT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET_MCWDT0` reader - Multi-Counter Watchdog timer reset #0 has occurred since last power cycle."]
pub type RESET_MCWDT0_R = crate::BitReader;
#[doc = "Field `RESET_MCWDT0` writer - Multi-Counter Watchdog timer reset #0 has occurred since last power cycle."]
pub type RESET_MCWDT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET_MCWDT1` reader - Multi-Counter Watchdog timer reset #1 has occurred since last power cycle."]
pub type RESET_MCWDT1_R = crate::BitReader;
#[doc = "Field `RESET_MCWDT1` writer - Multi-Counter Watchdog timer reset #1 has occurred since last power cycle."]
pub type RESET_MCWDT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET_MCWDT2` reader - Multi-Counter Watchdog timer reset #2 has occurred since last power cycle."]
pub type RESET_MCWDT2_R = crate::BitReader;
#[doc = "Field `RESET_MCWDT2` writer - Multi-Counter Watchdog timer reset #2 has occurred since last power cycle."]
pub type RESET_MCWDT2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET_MCWDT3` reader - Multi-Counter Watchdog timer reset #3 has occurred since last power cycle."]
pub type RESET_MCWDT3_R = crate::BitReader;
#[doc = "Field `RESET_MCWDT3` writer - Multi-Counter Watchdog timer reset #3 has occurred since last power cycle."]
pub type RESET_MCWDT3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - A basic WatchDog Timer (WDT) reset has occurred since last power cycle."]
    #[inline(always)]
    pub fn reset_wdt(&self) -> RESET_WDT_R {
        RESET_WDT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fault logging system requested a reset from its Active logic."]
    #[inline(always)]
    pub fn reset_act_fault(&self) -> RESET_ACT_FAULT_R {
        RESET_ACT_FAULT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fault logging system requested a reset from its DeepSleep logic."]
    #[inline(always)]
    pub fn reset_dpslp_fault(&self) -> RESET_DPSLP_FAULT_R {
        RESET_DPSLP_FAULT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clock supervision logic requested a reset due to loss of a watch-crystal clock."]
    #[inline(always)]
    pub fn reset_csv_wco_loss(&self) -> RESET_CSV_WCO_LOSS_R {
        RESET_CSV_WCO_LOSS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - A CPU requested a system reset through it's SYSRESETREQ. This can be done via a debugger probe or in firmware."]
    #[inline(always)]
    pub fn reset_soft(&self) -> RESET_SOFT_R {
        RESET_SOFT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Multi-Counter Watchdog timer reset #0 has occurred since last power cycle."]
    #[inline(always)]
    pub fn reset_mcwdt0(&self) -> RESET_MCWDT0_R {
        RESET_MCWDT0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Multi-Counter Watchdog timer reset #1 has occurred since last power cycle."]
    #[inline(always)]
    pub fn reset_mcwdt1(&self) -> RESET_MCWDT1_R {
        RESET_MCWDT1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Multi-Counter Watchdog timer reset #2 has occurred since last power cycle."]
    #[inline(always)]
    pub fn reset_mcwdt2(&self) -> RESET_MCWDT2_R {
        RESET_MCWDT2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Multi-Counter Watchdog timer reset #3 has occurred since last power cycle."]
    #[inline(always)]
    pub fn reset_mcwdt3(&self) -> RESET_MCWDT3_R {
        RESET_MCWDT3_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A basic WatchDog Timer (WDT) reset has occurred since last power cycle."]
    #[inline(always)]
    #[must_use]
    pub fn reset_wdt(&mut self) -> RESET_WDT_W<RES_CAUSE_SPEC> {
        RESET_WDT_W::new(self, 0)
    }
    #[doc = "Bit 1 - Fault logging system requested a reset from its Active logic."]
    #[inline(always)]
    #[must_use]
    pub fn reset_act_fault(&mut self) -> RESET_ACT_FAULT_W<RES_CAUSE_SPEC> {
        RESET_ACT_FAULT_W::new(self, 1)
    }
    #[doc = "Bit 2 - Fault logging system requested a reset from its DeepSleep logic."]
    #[inline(always)]
    #[must_use]
    pub fn reset_dpslp_fault(&mut self) -> RESET_DPSLP_FAULT_W<RES_CAUSE_SPEC> {
        RESET_DPSLP_FAULT_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clock supervision logic requested a reset due to loss of a watch-crystal clock."]
    #[inline(always)]
    #[must_use]
    pub fn reset_csv_wco_loss(&mut self) -> RESET_CSV_WCO_LOSS_W<RES_CAUSE_SPEC> {
        RESET_CSV_WCO_LOSS_W::new(self, 3)
    }
    #[doc = "Bit 4 - A CPU requested a system reset through it's SYSRESETREQ. This can be done via a debugger probe or in firmware."]
    #[inline(always)]
    #[must_use]
    pub fn reset_soft(&mut self) -> RESET_SOFT_W<RES_CAUSE_SPEC> {
        RESET_SOFT_W::new(self, 4)
    }
    #[doc = "Bit 5 - Multi-Counter Watchdog timer reset #0 has occurred since last power cycle."]
    #[inline(always)]
    #[must_use]
    pub fn reset_mcwdt0(&mut self) -> RESET_MCWDT0_W<RES_CAUSE_SPEC> {
        RESET_MCWDT0_W::new(self, 5)
    }
    #[doc = "Bit 6 - Multi-Counter Watchdog timer reset #1 has occurred since last power cycle."]
    #[inline(always)]
    #[must_use]
    pub fn reset_mcwdt1(&mut self) -> RESET_MCWDT1_W<RES_CAUSE_SPEC> {
        RESET_MCWDT1_W::new(self, 6)
    }
    #[doc = "Bit 7 - Multi-Counter Watchdog timer reset #2 has occurred since last power cycle."]
    #[inline(always)]
    #[must_use]
    pub fn reset_mcwdt2(&mut self) -> RESET_MCWDT2_W<RES_CAUSE_SPEC> {
        RESET_MCWDT2_W::new(self, 7)
    }
    #[doc = "Bit 8 - Multi-Counter Watchdog timer reset #3 has occurred since last power cycle."]
    #[inline(always)]
    #[must_use]
    pub fn reset_mcwdt3(&mut self) -> RESET_MCWDT3_W<RES_CAUSE_SPEC> {
        RESET_MCWDT3_W::new(self, 8)
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
#[doc = "Reset Cause Observation Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`res_cause::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`res_cause::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RES_CAUSE_SPEC;
impl crate::RegisterSpec for RES_CAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`res_cause::R`](R) reader structure"]
impl crate::Readable for RES_CAUSE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`res_cause::W`](W) writer structure"]
impl crate::Writable for RES_CAUSE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RES_CAUSE to value 0"]
impl crate::Resettable for RES_CAUSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
