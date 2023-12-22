#[doc = "Register `SRSS_INTR_MASK` reader"]
pub type R = crate::R<SRSS_INTR_MASK_SPEC>;
#[doc = "Register `SRSS_INTR_MASK` writer"]
pub type W = crate::W<SRSS_INTR_MASK_SPEC>;
#[doc = "Field `WDT_MATCH` reader - Mask for watchdog timer. Clearing this bit will not forward the interrupt to the CPU. It will not, however, disable the WDT reset generation on 2 missed interrupts. When WDT resets the chip, it also internally pends an interrupt that survives the reset. To prevent unintended ISR execution, clear SRSS_INTR.WDT_MATCH before setting this bit."]
pub type WDT_MATCH_R = crate::BitReader;
#[doc = "Field `WDT_MATCH` writer - Mask for watchdog timer. Clearing this bit will not forward the interrupt to the CPU. It will not, however, disable the WDT reset generation on 2 missed interrupts. When WDT resets the chip, it also internally pends an interrupt that survives the reset. To prevent unintended ISR execution, clear SRSS_INTR.WDT_MATCH before setting this bit."]
pub type WDT_MATCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HVLVD1` reader - Mask for low voltage detector HVLVD1"]
pub type HVLVD1_R = crate::BitReader;
#[doc = "Field `HVLVD1` writer - Mask for low voltage detector HVLVD1"]
pub type HVLVD1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_CAL` reader - Mask for clock calibration done"]
pub type CLK_CAL_R = crate::BitReader;
#[doc = "Field `CLK_CAL` writer - Mask for clock calibration done"]
pub type CLK_CAL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Mask for watchdog timer. Clearing this bit will not forward the interrupt to the CPU. It will not, however, disable the WDT reset generation on 2 missed interrupts. When WDT resets the chip, it also internally pends an interrupt that survives the reset. To prevent unintended ISR execution, clear SRSS_INTR.WDT_MATCH before setting this bit."]
    #[inline(always)]
    pub fn wdt_match(&self) -> WDT_MATCH_R {
        WDT_MATCH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask for low voltage detector HVLVD1"]
    #[inline(always)]
    pub fn hvlvd1(&self) -> HVLVD1_R {
        HVLVD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Mask for clock calibration done"]
    #[inline(always)]
    pub fn clk_cal(&self) -> CLK_CAL_R {
        CLK_CAL_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask for watchdog timer. Clearing this bit will not forward the interrupt to the CPU. It will not, however, disable the WDT reset generation on 2 missed interrupts. When WDT resets the chip, it also internally pends an interrupt that survives the reset. To prevent unintended ISR execution, clear SRSS_INTR.WDT_MATCH before setting this bit."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_match(&mut self) -> WDT_MATCH_W<SRSS_INTR_MASK_SPEC> {
        WDT_MATCH_W::new(self, 0)
    }
    #[doc = "Bit 1 - Mask for low voltage detector HVLVD1"]
    #[inline(always)]
    #[must_use]
    pub fn hvlvd1(&mut self) -> HVLVD1_W<SRSS_INTR_MASK_SPEC> {
        HVLVD1_W::new(self, 1)
    }
    #[doc = "Bit 5 - Mask for clock calibration done"]
    #[inline(always)]
    #[must_use]
    pub fn clk_cal(&mut self) -> CLK_CAL_W<SRSS_INTR_MASK_SPEC> {
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
#[doc = "SRSS Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srss_intr_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srss_intr_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRSS_INTR_MASK_SPEC;
impl crate::RegisterSpec for SRSS_INTR_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srss_intr_mask::R`](R) reader structure"]
impl crate::Readable for SRSS_INTR_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`srss_intr_mask::W`](W) writer structure"]
impl crate::Writable for SRSS_INTR_MASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRSS_INTR_MASK to value 0"]
impl crate::Resettable for SRSS_INTR_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
