#[doc = "Register `TX_WATCHDOG` reader"]
pub type R = crate::R<TX_WATCHDOG_SPEC>;
#[doc = "Register `TX_WATCHDOG` writer"]
pub type W = crate::W<TX_WATCHDOG_SPEC>;
#[doc = "Field `WD_COUNTER` reader - Start value of the TX watchdog. With the reset value of 0x0000:0000 the counter is disabled. This is clocked by the AHB-Lite system clock 'clk_sys'."]
pub type WD_COUNTER_R = crate::FieldReader<u32>;
#[doc = "Field `WD_COUNTER` writer - Start value of the TX watchdog. With the reset value of 0x0000:0000 the counter is disabled. This is clocked by the AHB-Lite system clock 'clk_sys'."]
pub type WD_COUNTER_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Start value of the TX watchdog. With the reset value of 0x0000:0000 the counter is disabled. This is clocked by the AHB-Lite system clock 'clk_sys'."]
    #[inline(always)]
    pub fn wd_counter(&self) -> WD_COUNTER_R {
        WD_COUNTER_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start value of the TX watchdog. With the reset value of 0x0000:0000 the counter is disabled. This is clocked by the AHB-Lite system clock 'clk_sys'."]
    #[inline(always)]
    #[must_use]
    pub fn wd_counter(&mut self) -> WD_COUNTER_W<TX_WATCHDOG_SPEC> {
        WD_COUNTER_W::new(self, 0)
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
#[doc = "Transmitter watchdog\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_watchdog::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_watchdog::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_WATCHDOG_SPEC;
impl crate::RegisterSpec for TX_WATCHDOG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_watchdog::R`](R) reader structure"]
impl crate::Readable for TX_WATCHDOG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_watchdog::W`](W) writer structure"]
impl crate::Writable for TX_WATCHDOG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_WATCHDOG to value 0"]
impl crate::Resettable for TX_WATCHDOG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
