#[doc = "Register `SRSS_INTR_SET` reader"]
pub type R = crate::R<SRSS_INTR_SET_SPEC>;
#[doc = "Register `SRSS_INTR_SET` writer"]
pub type W = crate::W<SRSS_INTR_SET_SPEC>;
#[doc = "Field `WDT_MATCH` reader - Set interrupt for low voltage detector WDT_MATCH"]
pub type WDT_MATCH_R = crate::BitReader;
#[doc = "Field `WDT_MATCH` writer - Set interrupt for low voltage detector WDT_MATCH"]
pub type WDT_MATCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HVLVD1` reader - Set interrupt for low voltage detector HVLVD1"]
pub type HVLVD1_R = crate::BitReader;
#[doc = "Field `HVLVD1` writer - Set interrupt for low voltage detector HVLVD1"]
pub type HVLVD1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_CAL` reader - Set interrupt for clock calibration counter done. This field is reset during DEEPSLEEP mode."]
pub type CLK_CAL_R = crate::BitReader;
#[doc = "Field `CLK_CAL` writer - Set interrupt for clock calibration counter done. This field is reset during DEEPSLEEP mode."]
pub type CLK_CAL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set interrupt for low voltage detector WDT_MATCH"]
    #[inline(always)]
    pub fn wdt_match(&self) -> WDT_MATCH_R {
        WDT_MATCH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set interrupt for low voltage detector HVLVD1"]
    #[inline(always)]
    pub fn hvlvd1(&self) -> HVLVD1_R {
        HVLVD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Set interrupt for clock calibration counter done. This field is reset during DEEPSLEEP mode."]
    #[inline(always)]
    pub fn clk_cal(&self) -> CLK_CAL_R {
        CLK_CAL_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set interrupt for low voltage detector WDT_MATCH"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_match(&mut self) -> WDT_MATCH_W<SRSS_INTR_SET_SPEC> {
        WDT_MATCH_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set interrupt for low voltage detector HVLVD1"]
    #[inline(always)]
    #[must_use]
    pub fn hvlvd1(&mut self) -> HVLVD1_W<SRSS_INTR_SET_SPEC> {
        HVLVD1_W::new(self, 1)
    }
    #[doc = "Bit 5 - Set interrupt for clock calibration counter done. This field is reset during DEEPSLEEP mode."]
    #[inline(always)]
    #[must_use]
    pub fn clk_cal(&mut self) -> CLK_CAL_W<SRSS_INTR_SET_SPEC> {
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
#[doc = "SRSS Interrupt Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srss_intr_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srss_intr_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRSS_INTR_SET_SPEC;
impl crate::RegisterSpec for SRSS_INTR_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srss_intr_set::R`](R) reader structure"]
impl crate::Readable for SRSS_INTR_SET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`srss_intr_set::W`](W) writer structure"]
impl crate::Writable for SRSS_INTR_SET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRSS_INTR_SET to value 0"]
impl crate::Resettable for SRSS_INTR_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
