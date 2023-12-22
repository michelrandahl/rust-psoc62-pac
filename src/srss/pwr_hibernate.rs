#[doc = "Register `PWR_HIBERNATE` reader"]
pub type R = crate::R<PWR_HIBERNATE_SPEC>;
#[doc = "Register `PWR_HIBERNATE` writer"]
pub type W = crate::W<PWR_HIBERNATE_SPEC>;
#[doc = "Field `TOKEN` reader - Contains a 8-bit token that is retained through a HIBERNATE/WAKEUP sequence that can be used by firmware to differentiate WAKEUP from a general RESET event. Note that waking up from HIBERNATE using XRES will reset this register."]
pub type TOKEN_R = crate::FieldReader;
#[doc = "Field `TOKEN` writer - Contains a 8-bit token that is retained through a HIBERNATE/WAKEUP sequence that can be used by firmware to differentiate WAKEUP from a general RESET event. Note that waking up from HIBERNATE using XRES will reset this register."]
pub type TOKEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `UNLOCK` reader - This byte must be set to 0x3A for FREEZE or HIBERNATE fields to operate. Any other value in this register will cause FREEZE/HIBERNATE to have no effect, except as noted in the FREEZE description."]
pub type UNLOCK_R = crate::FieldReader;
#[doc = "Field `UNLOCK` writer - This byte must be set to 0x3A for FREEZE or HIBERNATE fields to operate. Any other value in this register will cause FREEZE/HIBERNATE to have no effect, except as noted in the FREEZE description."]
pub type UNLOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FREEZE` reader - Firmware sets this bit to freeze the configuration, mode and state of all GPIOs and SIOs in the system. When entering HIBERNATE mode, the first write instructs DEEPSLEEP peripherals that they cannot ignore the upcoming freeze command. This occurs even in the illegal condition where UNLOCK is not set. If UNLOCK and HIBERNATE are properly set, the IOs actually freeze on the second write."]
pub type FREEZE_R = crate::BitReader;
#[doc = "Field `FREEZE` writer - Firmware sets this bit to freeze the configuration, mode and state of all GPIOs and SIOs in the system. When entering HIBERNATE mode, the first write instructs DEEPSLEEP peripherals that they cannot ignore the upcoming freeze command. This occurs even in the illegal condition where UNLOCK is not set. If UNLOCK and HIBERNATE are properly set, the IOs actually freeze on the second write."]
pub type FREEZE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK_HIBALARM` reader - When set, HIBERNATE will wakeup for a RTC interrupt"]
pub type MASK_HIBALARM_R = crate::BitReader;
#[doc = "Field `MASK_HIBALARM` writer - When set, HIBERNATE will wakeup for a RTC interrupt"]
pub type MASK_HIBALARM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK_HIBWDT` reader - When set, HIBERNATE will wakeup if WDT matches"]
pub type MASK_HIBWDT_R = crate::BitReader;
#[doc = "Field `MASK_HIBWDT` writer - When set, HIBERNATE will wakeup if WDT matches"]
pub type MASK_HIBWDT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLARITY_HIBPIN` reader - Each bit sets the active polarity of the corresponding wakeup pin. 0: Pin input of 0 will wakeup the part from HIBERNATE 1: Pin input of 1 will wakeup the part from HIBERNATE"]
pub type POLARITY_HIBPIN_R = crate::FieldReader;
#[doc = "Field `POLARITY_HIBPIN` writer - Each bit sets the active polarity of the corresponding wakeup pin. 0: Pin input of 0 will wakeup the part from HIBERNATE 1: Pin input of 1 will wakeup the part from HIBERNATE"]
pub type POLARITY_HIBPIN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MASK_HIBPIN` reader - When set, HIBERNATE will wakeup if the corresponding pin input matches the POLARITY_HIBPIN setting. Each bit corresponds to one of the wakeup pins."]
pub type MASK_HIBPIN_R = crate::FieldReader;
#[doc = "Field `MASK_HIBPIN` writer - When set, HIBERNATE will wakeup if the corresponding pin input matches the POLARITY_HIBPIN setting. Each bit corresponds to one of the wakeup pins."]
pub type MASK_HIBPIN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HIBERNATE_DISABLE` reader - Hibernate disable bit. 0: Normal operation, HIBERNATE works as described 1: Further writes to this register are ignored Note: This bit is a write-once bit until the next reset. Avoid changing any other bits in this register while disabling HIBERNATE mode. Also, it is recommended to clear the UNLOCK code, if it was previously written.."]
pub type HIBERNATE_DISABLE_R = crate::BitReader;
#[doc = "Field `HIBERNATE_DISABLE` writer - Hibernate disable bit. 0: Normal operation, HIBERNATE works as described 1: Further writes to this register are ignored Note: This bit is a write-once bit until the next reset. Avoid changing any other bits in this register while disabling HIBERNATE mode. Also, it is recommended to clear the UNLOCK code, if it was previously written.."]
pub type HIBERNATE_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HIBERNATE` reader - Firmware sets this bit to enter HIBERNATE mode. The system will enter HIBERNATE mode immediately after writing to this bit and will wakeup only in response to XRES or WAKEUP event. Both UNLOCK and FREEZE must have been set correctly in a previous write operations. Otherwise, it will not enter HIBERNATE. External supplies must have been stable for 250us before entering HIBERNATE mode."]
pub type HIBERNATE_R = crate::BitReader;
#[doc = "Field `HIBERNATE` writer - Firmware sets this bit to enter HIBERNATE mode. The system will enter HIBERNATE mode immediately after writing to this bit and will wakeup only in response to XRES or WAKEUP event. Both UNLOCK and FREEZE must have been set correctly in a previous write operations. Otherwise, it will not enter HIBERNATE. External supplies must have been stable for 250us before entering HIBERNATE mode."]
pub type HIBERNATE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Contains a 8-bit token that is retained through a HIBERNATE/WAKEUP sequence that can be used by firmware to differentiate WAKEUP from a general RESET event. Note that waking up from HIBERNATE using XRES will reset this register."]
    #[inline(always)]
    pub fn token(&self) -> TOKEN_R {
        TOKEN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - This byte must be set to 0x3A for FREEZE or HIBERNATE fields to operate. Any other value in this register will cause FREEZE/HIBERNATE to have no effect, except as noted in the FREEZE description."]
    #[inline(always)]
    pub fn unlock(&self) -> UNLOCK_R {
        UNLOCK_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 17 - Firmware sets this bit to freeze the configuration, mode and state of all GPIOs and SIOs in the system. When entering HIBERNATE mode, the first write instructs DEEPSLEEP peripherals that they cannot ignore the upcoming freeze command. This occurs even in the illegal condition where UNLOCK is not set. If UNLOCK and HIBERNATE are properly set, the IOs actually freeze on the second write."]
    #[inline(always)]
    pub fn freeze(&self) -> FREEZE_R {
        FREEZE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - When set, HIBERNATE will wakeup for a RTC interrupt"]
    #[inline(always)]
    pub fn mask_hibalarm(&self) -> MASK_HIBALARM_R {
        MASK_HIBALARM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - When set, HIBERNATE will wakeup if WDT matches"]
    #[inline(always)]
    pub fn mask_hibwdt(&self) -> MASK_HIBWDT_R {
        MASK_HIBWDT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - Each bit sets the active polarity of the corresponding wakeup pin. 0: Pin input of 0 will wakeup the part from HIBERNATE 1: Pin input of 1 will wakeup the part from HIBERNATE"]
    #[inline(always)]
    pub fn polarity_hibpin(&self) -> POLARITY_HIBPIN_R {
        POLARITY_HIBPIN_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - When set, HIBERNATE will wakeup if the corresponding pin input matches the POLARITY_HIBPIN setting. Each bit corresponds to one of the wakeup pins."]
    #[inline(always)]
    pub fn mask_hibpin(&self) -> MASK_HIBPIN_R {
        MASK_HIBPIN_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Hibernate disable bit. 0: Normal operation, HIBERNATE works as described 1: Further writes to this register are ignored Note: This bit is a write-once bit until the next reset. Avoid changing any other bits in this register while disabling HIBERNATE mode. Also, it is recommended to clear the UNLOCK code, if it was previously written.."]
    #[inline(always)]
    pub fn hibernate_disable(&self) -> HIBERNATE_DISABLE_R {
        HIBERNATE_DISABLE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Firmware sets this bit to enter HIBERNATE mode. The system will enter HIBERNATE mode immediately after writing to this bit and will wakeup only in response to XRES or WAKEUP event. Both UNLOCK and FREEZE must have been set correctly in a previous write operations. Otherwise, it will not enter HIBERNATE. External supplies must have been stable for 250us before entering HIBERNATE mode."]
    #[inline(always)]
    pub fn hibernate(&self) -> HIBERNATE_R {
        HIBERNATE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Contains a 8-bit token that is retained through a HIBERNATE/WAKEUP sequence that can be used by firmware to differentiate WAKEUP from a general RESET event. Note that waking up from HIBERNATE using XRES will reset this register."]
    #[inline(always)]
    #[must_use]
    pub fn token(&mut self) -> TOKEN_W<PWR_HIBERNATE_SPEC> {
        TOKEN_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - This byte must be set to 0x3A for FREEZE or HIBERNATE fields to operate. Any other value in this register will cause FREEZE/HIBERNATE to have no effect, except as noted in the FREEZE description."]
    #[inline(always)]
    #[must_use]
    pub fn unlock(&mut self) -> UNLOCK_W<PWR_HIBERNATE_SPEC> {
        UNLOCK_W::new(self, 8)
    }
    #[doc = "Bit 17 - Firmware sets this bit to freeze the configuration, mode and state of all GPIOs and SIOs in the system. When entering HIBERNATE mode, the first write instructs DEEPSLEEP peripherals that they cannot ignore the upcoming freeze command. This occurs even in the illegal condition where UNLOCK is not set. If UNLOCK and HIBERNATE are properly set, the IOs actually freeze on the second write."]
    #[inline(always)]
    #[must_use]
    pub fn freeze(&mut self) -> FREEZE_W<PWR_HIBERNATE_SPEC> {
        FREEZE_W::new(self, 17)
    }
    #[doc = "Bit 18 - When set, HIBERNATE will wakeup for a RTC interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn mask_hibalarm(&mut self) -> MASK_HIBALARM_W<PWR_HIBERNATE_SPEC> {
        MASK_HIBALARM_W::new(self, 18)
    }
    #[doc = "Bit 19 - When set, HIBERNATE will wakeup if WDT matches"]
    #[inline(always)]
    #[must_use]
    pub fn mask_hibwdt(&mut self) -> MASK_HIBWDT_W<PWR_HIBERNATE_SPEC> {
        MASK_HIBWDT_W::new(self, 19)
    }
    #[doc = "Bits 20:23 - Each bit sets the active polarity of the corresponding wakeup pin. 0: Pin input of 0 will wakeup the part from HIBERNATE 1: Pin input of 1 will wakeup the part from HIBERNATE"]
    #[inline(always)]
    #[must_use]
    pub fn polarity_hibpin(&mut self) -> POLARITY_HIBPIN_W<PWR_HIBERNATE_SPEC> {
        POLARITY_HIBPIN_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - When set, HIBERNATE will wakeup if the corresponding pin input matches the POLARITY_HIBPIN setting. Each bit corresponds to one of the wakeup pins."]
    #[inline(always)]
    #[must_use]
    pub fn mask_hibpin(&mut self) -> MASK_HIBPIN_W<PWR_HIBERNATE_SPEC> {
        MASK_HIBPIN_W::new(self, 24)
    }
    #[doc = "Bit 30 - Hibernate disable bit. 0: Normal operation, HIBERNATE works as described 1: Further writes to this register are ignored Note: This bit is a write-once bit until the next reset. Avoid changing any other bits in this register while disabling HIBERNATE mode. Also, it is recommended to clear the UNLOCK code, if it was previously written.."]
    #[inline(always)]
    #[must_use]
    pub fn hibernate_disable(&mut self) -> HIBERNATE_DISABLE_W<PWR_HIBERNATE_SPEC> {
        HIBERNATE_DISABLE_W::new(self, 30)
    }
    #[doc = "Bit 31 - Firmware sets this bit to enter HIBERNATE mode. The system will enter HIBERNATE mode immediately after writing to this bit and will wakeup only in response to XRES or WAKEUP event. Both UNLOCK and FREEZE must have been set correctly in a previous write operations. Otherwise, it will not enter HIBERNATE. External supplies must have been stable for 250us before entering HIBERNATE mode."]
    #[inline(always)]
    #[must_use]
    pub fn hibernate(&mut self) -> HIBERNATE_W<PWR_HIBERNATE_SPEC> {
        HIBERNATE_W::new(self, 31)
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
#[doc = "HIBERNATE Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_hibernate::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_hibernate::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWR_HIBERNATE_SPEC;
impl crate::RegisterSpec for PWR_HIBERNATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_hibernate::R`](R) reader structure"]
impl crate::Readable for PWR_HIBERNATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwr_hibernate::W`](W) writer structure"]
impl crate::Writable for PWR_HIBERNATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWR_HIBERNATE to value 0"]
impl crate::Resettable for PWR_HIBERNATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
