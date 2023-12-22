#[doc = "Register `USBIO_CR1` reader"]
pub type R = crate::R<USBIO_CR1_SPEC>;
#[doc = "Register `USBIO_CR1` writer"]
pub type W = crate::W<USBIO_CR1_SPEC>;
#[doc = "Field `DMO` reader - This read only bit gives the state of the D- pin when IOMODE bit is '0' and USB doesn't transmit. This bit is '0' when USB transmits SE0, and this bit is '1' when USB transmits other than SE0. This bit is valid if USB Device."]
pub type DMO_R = crate::BitReader;
#[doc = "Field `DPO` reader - This read only bit gives the state of the D+ pin when IOMODE bit is '0' and USB doesn't transmit. This bit displays the output value of D+ pin when USB transmits SE0 or data. This bit is valid if USB Device."]
pub type DPO_R = crate::BitReader;
#[doc = "Field `RSVD_2` reader - N/A"]
pub type RSVD_2_R = crate::BitReader;
#[doc = "Field `RSVD_2` writer - N/A"]
pub type RSVD_2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOMODE` reader - This bit allows the D+ and D- pins to be configured for either USB mode or bit-banged modes. If this bit is set the DMI and DPI bits are used to drive the D- and D+ pins."]
pub type IOMODE_R = crate::BitReader;
#[doc = "Field `IOMODE` writer - This bit allows the D+ and D- pins to be configured for either USB mode or bit-banged modes. If this bit is set the DMI and DPI bits are used to drive the D- and D+ pins."]
pub type IOMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This read only bit gives the state of the D- pin when IOMODE bit is '0' and USB doesn't transmit. This bit is '0' when USB transmits SE0, and this bit is '1' when USB transmits other than SE0. This bit is valid if USB Device."]
    #[inline(always)]
    pub fn dmo(&self) -> DMO_R {
        DMO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This read only bit gives the state of the D+ pin when IOMODE bit is '0' and USB doesn't transmit. This bit displays the output value of D+ pin when USB transmits SE0 or data. This bit is valid if USB Device."]
    #[inline(always)]
    pub fn dpo(&self) -> DPO_R {
        DPO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn rsvd_2(&self) -> RSVD_2_R {
        RSVD_2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit allows the D+ and D- pins to be configured for either USB mode or bit-banged modes. If this bit is set the DMI and DPI bits are used to drive the D- and D+ pins."]
    #[inline(always)]
    pub fn iomode(&self) -> IOMODE_R {
        IOMODE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn rsvd_2(&mut self) -> RSVD_2_W<USBIO_CR1_SPEC> {
        RSVD_2_W::new(self, 2)
    }
    #[doc = "Bit 5 - This bit allows the D+ and D- pins to be configured for either USB mode or bit-banged modes. If this bit is set the DMI and DPI bits are used to drive the D- and D+ pins."]
    #[inline(always)]
    #[must_use]
    pub fn iomode(&mut self) -> IOMODE_W<USBIO_CR1_SPEC> {
        IOMODE_W::new(self, 5)
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
#[doc = "USBIO control 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbio_cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbio_cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBIO_CR1_SPEC;
impl crate::RegisterSpec for USBIO_CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbio_cr1::R`](R) reader structure"]
impl crate::Readable for USBIO_CR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbio_cr1::W`](W) writer structure"]
impl crate::Writable for USBIO_CR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBIO_CR1 to value 0x20"]
impl crate::Resettable for USBIO_CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
