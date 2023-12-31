#[doc = "Register `SRSS_INTR` reader"]
pub type R = crate::R<SRSS_INTR_SPEC>;
#[doc = "Register `SRSS_INTR` writer"]
pub type W = crate::W<SRSS_INTR_SPEC>;
#[doc = "Field `WDT_MATCH` reader - WDT Interrupt Request. This bit is set each time WDT_COUNTR==WDT_MATCH. W1C also feeds the watch dog. Missing 2 interrupts in a row will generate a reset. Due to internal synchronization, it takes 2 SYSCLK cycles to update after a W1C."]
pub type WDT_MATCH_R = crate::BitReader;
#[doc = "Field `WDT_MATCH` writer - WDT Interrupt Request. This bit is set each time WDT_COUNTR==WDT_MATCH. W1C also feeds the watch dog. Missing 2 interrupts in a row will generate a reset. Due to internal synchronization, it takes 2 SYSCLK cycles to update after a W1C."]
pub type WDT_MATCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HVLVD1` reader - Interrupt for low voltage detector HVLVD1"]
pub type HVLVD1_R = crate::BitReader;
#[doc = "Field `HVLVD1` writer - Interrupt for low voltage detector HVLVD1"]
pub type HVLVD1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_CAL` reader - Clock calibration counter is done. This field is reset during DEEPSLEEP mode."]
pub type CLK_CAL_R = crate::BitReader;
#[doc = "Field `CLK_CAL` writer - Clock calibration counter is done. This field is reset during DEEPSLEEP mode."]
pub type CLK_CAL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - WDT Interrupt Request. This bit is set each time WDT_COUNTR==WDT_MATCH. W1C also feeds the watch dog. Missing 2 interrupts in a row will generate a reset. Due to internal synchronization, it takes 2 SYSCLK cycles to update after a W1C."]
    #[inline(always)]
    pub fn wdt_match(&self) -> WDT_MATCH_R {
        WDT_MATCH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt for low voltage detector HVLVD1"]
    #[inline(always)]
    pub fn hvlvd1(&self) -> HVLVD1_R {
        HVLVD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock calibration counter is done. This field is reset during DEEPSLEEP mode."]
    #[inline(always)]
    pub fn clk_cal(&self) -> CLK_CAL_R {
        CLK_CAL_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WDT Interrupt Request. This bit is set each time WDT_COUNTR==WDT_MATCH. W1C also feeds the watch dog. Missing 2 interrupts in a row will generate a reset. Due to internal synchronization, it takes 2 SYSCLK cycles to update after a W1C."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_match(&mut self) -> WDT_MATCH_W<SRSS_INTR_SPEC> {
        WDT_MATCH_W::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt for low voltage detector HVLVD1"]
    #[inline(always)]
    #[must_use]
    pub fn hvlvd1(&mut self) -> HVLVD1_W<SRSS_INTR_SPEC> {
        HVLVD1_W::new(self, 1)
    }
    #[doc = "Bit 5 - Clock calibration counter is done. This field is reset during DEEPSLEEP mode."]
    #[inline(always)]
    #[must_use]
    pub fn clk_cal(&mut self) -> CLK_CAL_W<SRSS_INTR_SPEC> {
        CLK_CAL_W::new(self, 5)
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
#[doc = "SRSS Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srss_intr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srss_intr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRSS_INTR_SPEC;
impl crate::RegisterSpec for SRSS_INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srss_intr::R`](R) reader structure"]
impl crate::Readable for SRSS_INTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`srss_intr::W`](W) writer structure"]
impl crate::Writable for SRSS_INTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRSS_INTR to value 0"]
impl crate::Resettable for SRSS_INTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
