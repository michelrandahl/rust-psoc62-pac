#[doc = "Register `INTR` reader"]
pub type R = crate::R<INTR_SPEC>;
#[doc = "Register `INTR` writer"]
pub type W = crate::W<INTR_SPEC>;
#[doc = "Field `EOS_INTR` reader - End Of Scan Interrupt: hardware sets this interrupt after completing a scan of all the enabled channels. Write with '1' to clear bit."]
pub type EOS_INTR_R = crate::BitReader;
#[doc = "Field `EOS_INTR` writer - End Of Scan Interrupt: hardware sets this interrupt after completing a scan of all the enabled channels. Write with '1' to clear bit."]
pub type EOS_INTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFLOW_INTR` reader - Overflow Interrupt: hardware sets this interrupt when it sets a new EOS_INTR while that bit was not yet cleared by the firmware. Write with '1' to clear bit."]
pub type OVERFLOW_INTR_R = crate::BitReader;
#[doc = "Field `OVERFLOW_INTR` writer - Overflow Interrupt: hardware sets this interrupt when it sets a new EOS_INTR while that bit was not yet cleared by the firmware. Write with '1' to clear bit."]
pub type OVERFLOW_INTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FW_COLLISION_INTR` reader - Firmware Collision Interrupt: hardware sets this interrupt when FW_TRIGGER is asserted while the SAR is BUSY. Raising this interrupt is delayed to when the scan caused by the FW_TRIGGER has been completed, i.e. not when the preceding scan with which this trigger collided is completed. When this interrupt is set it implies that the channels were sampled later than was intended (jitter). Write with '1' to clear bit."]
pub type FW_COLLISION_INTR_R = crate::BitReader;
#[doc = "Field `FW_COLLISION_INTR` writer - Firmware Collision Interrupt: hardware sets this interrupt when FW_TRIGGER is asserted while the SAR is BUSY. Raising this interrupt is delayed to when the scan caused by the FW_TRIGGER has been completed, i.e. not when the preceding scan with which this trigger collided is completed. When this interrupt is set it implies that the channels were sampled later than was intended (jitter). Write with '1' to clear bit."]
pub type FW_COLLISION_INTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSI_COLLISION_INTR` reader - DSI Collision Interrupt: hardware sets this interrupt when the DSI trigger signal is asserted while the SAR is BUSY. Raising this interrupt is delayed to when the scan caused by the DSI trigger has been completed, i.e. not when the preceding scan with which this trigger collided is completed. When this interrupt is set it implies that the channels were sampled later than was intended (jitter). Write with '1' to clear bit."]
pub type DSI_COLLISION_INTR_R = crate::BitReader;
#[doc = "Field `DSI_COLLISION_INTR` writer - DSI Collision Interrupt: hardware sets this interrupt when the DSI trigger signal is asserted while the SAR is BUSY. Raising this interrupt is delayed to when the scan caused by the DSI trigger has been completed, i.e. not when the preceding scan with which this trigger collided is completed. When this interrupt is set it implies that the channels were sampled later than was intended (jitter). Write with '1' to clear bit."]
pub type DSI_COLLISION_INTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INJ_EOC_INTR` reader - Injection End of Conversion Interrupt: hardware sets this interrupt after completing the conversion for the injection channel (irrespective of if tailgating was used). Write with '1' to clear bit."]
pub type INJ_EOC_INTR_R = crate::BitReader;
#[doc = "Field `INJ_EOC_INTR` writer - Injection End of Conversion Interrupt: hardware sets this interrupt after completing the conversion for the injection channel (irrespective of if tailgating was used). Write with '1' to clear bit."]
pub type INJ_EOC_INTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INJ_SATURATE_INTR` reader - Injection Saturation Interrupt: hardware sets this interrupt if an injection conversion result (before averaging) is either 0x000 or 0xFFF, this is an indication that the ADC likely saturated. Write with '1' to clear bit."]
pub type INJ_SATURATE_INTR_R = crate::BitReader;
#[doc = "Field `INJ_SATURATE_INTR` writer - Injection Saturation Interrupt: hardware sets this interrupt if an injection conversion result (before averaging) is either 0x000 or 0xFFF, this is an indication that the ADC likely saturated. Write with '1' to clear bit."]
pub type INJ_SATURATE_INTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INJ_RANGE_INTR` reader - Injection Range detect Interrupt: hardware sets this interrupt if the injection conversion result (after averaging) met the condition specified by the SAR_RANGE registers. Write with '1' to clear bit."]
pub type INJ_RANGE_INTR_R = crate::BitReader;
#[doc = "Field `INJ_RANGE_INTR` writer - Injection Range detect Interrupt: hardware sets this interrupt if the injection conversion result (after averaging) met the condition specified by the SAR_RANGE registers. Write with '1' to clear bit."]
pub type INJ_RANGE_INTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INJ_COLLISION_INTR` reader - Injection Collision Interrupt: hardware sets this interrupt when the injection trigger signal is asserted (INJ_START_EN==1 &amp;&amp; INJ_TAILGATING==0) while the SAR is BUSY. Raising this interrupt is delayed to when the sampling of the injection channel has been completed, i.e. not when the preceding scan with which this trigger collided is completed. When this interrupt is set it implies that the injection channel was sampled later than was intended. Write with '1' to clear bit."]
pub type INJ_COLLISION_INTR_R = crate::BitReader;
#[doc = "Field `INJ_COLLISION_INTR` writer - Injection Collision Interrupt: hardware sets this interrupt when the injection trigger signal is asserted (INJ_START_EN==1 &amp;&amp; INJ_TAILGATING==0) while the SAR is BUSY. Raising this interrupt is delayed to when the sampling of the injection channel has been completed, i.e. not when the preceding scan with which this trigger collided is completed. When this interrupt is set it implies that the injection channel was sampled later than was intended. Write with '1' to clear bit."]
pub type INJ_COLLISION_INTR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - End Of Scan Interrupt: hardware sets this interrupt after completing a scan of all the enabled channels. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn eos_intr(&self) -> EOS_INTR_R {
        EOS_INTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overflow Interrupt: hardware sets this interrupt when it sets a new EOS_INTR while that bit was not yet cleared by the firmware. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn overflow_intr(&self) -> OVERFLOW_INTR_R {
        OVERFLOW_INTR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Firmware Collision Interrupt: hardware sets this interrupt when FW_TRIGGER is asserted while the SAR is BUSY. Raising this interrupt is delayed to when the scan caused by the FW_TRIGGER has been completed, i.e. not when the preceding scan with which this trigger collided is completed. When this interrupt is set it implies that the channels were sampled later than was intended (jitter). Write with '1' to clear bit."]
    #[inline(always)]
    pub fn fw_collision_intr(&self) -> FW_COLLISION_INTR_R {
        FW_COLLISION_INTR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DSI Collision Interrupt: hardware sets this interrupt when the DSI trigger signal is asserted while the SAR is BUSY. Raising this interrupt is delayed to when the scan caused by the DSI trigger has been completed, i.e. not when the preceding scan with which this trigger collided is completed. When this interrupt is set it implies that the channels were sampled later than was intended (jitter). Write with '1' to clear bit."]
    #[inline(always)]
    pub fn dsi_collision_intr(&self) -> DSI_COLLISION_INTR_R {
        DSI_COLLISION_INTR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Injection End of Conversion Interrupt: hardware sets this interrupt after completing the conversion for the injection channel (irrespective of if tailgating was used). Write with '1' to clear bit."]
    #[inline(always)]
    pub fn inj_eoc_intr(&self) -> INJ_EOC_INTR_R {
        INJ_EOC_INTR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Injection Saturation Interrupt: hardware sets this interrupt if an injection conversion result (before averaging) is either 0x000 or 0xFFF, this is an indication that the ADC likely saturated. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn inj_saturate_intr(&self) -> INJ_SATURATE_INTR_R {
        INJ_SATURATE_INTR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Injection Range detect Interrupt: hardware sets this interrupt if the injection conversion result (after averaging) met the condition specified by the SAR_RANGE registers. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn inj_range_intr(&self) -> INJ_RANGE_INTR_R {
        INJ_RANGE_INTR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Injection Collision Interrupt: hardware sets this interrupt when the injection trigger signal is asserted (INJ_START_EN==1 &amp;&amp; INJ_TAILGATING==0) while the SAR is BUSY. Raising this interrupt is delayed to when the sampling of the injection channel has been completed, i.e. not when the preceding scan with which this trigger collided is completed. When this interrupt is set it implies that the injection channel was sampled later than was intended. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn inj_collision_intr(&self) -> INJ_COLLISION_INTR_R {
        INJ_COLLISION_INTR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End Of Scan Interrupt: hardware sets this interrupt after completing a scan of all the enabled channels. Write with '1' to clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn eos_intr(&mut self) -> EOS_INTR_W<INTR_SPEC> {
        EOS_INTR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Overflow Interrupt: hardware sets this interrupt when it sets a new EOS_INTR while that bit was not yet cleared by the firmware. Write with '1' to clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn overflow_intr(&mut self) -> OVERFLOW_INTR_W<INTR_SPEC> {
        OVERFLOW_INTR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Firmware Collision Interrupt: hardware sets this interrupt when FW_TRIGGER is asserted while the SAR is BUSY. Raising this interrupt is delayed to when the scan caused by the FW_TRIGGER has been completed, i.e. not when the preceding scan with which this trigger collided is completed. When this interrupt is set it implies that the channels were sampled later than was intended (jitter). Write with '1' to clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn fw_collision_intr(&mut self) -> FW_COLLISION_INTR_W<INTR_SPEC> {
        FW_COLLISION_INTR_W::new(self, 2)
    }
    #[doc = "Bit 3 - DSI Collision Interrupt: hardware sets this interrupt when the DSI trigger signal is asserted while the SAR is BUSY. Raising this interrupt is delayed to when the scan caused by the DSI trigger has been completed, i.e. not when the preceding scan with which this trigger collided is completed. When this interrupt is set it implies that the channels were sampled later than was intended (jitter). Write with '1' to clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn dsi_collision_intr(&mut self) -> DSI_COLLISION_INTR_W<INTR_SPEC> {
        DSI_COLLISION_INTR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Injection End of Conversion Interrupt: hardware sets this interrupt after completing the conversion for the injection channel (irrespective of if tailgating was used). Write with '1' to clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn inj_eoc_intr(&mut self) -> INJ_EOC_INTR_W<INTR_SPEC> {
        INJ_EOC_INTR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Injection Saturation Interrupt: hardware sets this interrupt if an injection conversion result (before averaging) is either 0x000 or 0xFFF, this is an indication that the ADC likely saturated. Write with '1' to clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn inj_saturate_intr(&mut self) -> INJ_SATURATE_INTR_W<INTR_SPEC> {
        INJ_SATURATE_INTR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Injection Range detect Interrupt: hardware sets this interrupt if the injection conversion result (after averaging) met the condition specified by the SAR_RANGE registers. Write with '1' to clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn inj_range_intr(&mut self) -> INJ_RANGE_INTR_W<INTR_SPEC> {
        INJ_RANGE_INTR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Injection Collision Interrupt: hardware sets this interrupt when the injection trigger signal is asserted (INJ_START_EN==1 &amp;&amp; INJ_TAILGATING==0) while the SAR is BUSY. Raising this interrupt is delayed to when the sampling of the injection channel has been completed, i.e. not when the preceding scan with which this trigger collided is completed. When this interrupt is set it implies that the injection channel was sampled later than was intended. Write with '1' to clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn inj_collision_intr(&mut self) -> INJ_COLLISION_INTR_W<INTR_SPEC> {
        INJ_COLLISION_INTR_W::new(self, 7)
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
#[doc = "Interrupt request register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_SPEC;
impl crate::RegisterSpec for INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr::R`](R) reader structure"]
impl crate::Readable for INTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr::W`](W) writer structure"]
impl crate::Writable for INTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for INTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
