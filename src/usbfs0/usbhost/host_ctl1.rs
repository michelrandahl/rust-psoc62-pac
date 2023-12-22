#[doc = "Register `HOST_CTL1` reader"]
pub type R = crate::R<HOST_CTL1_SPEC>;
#[doc = "Register `HOST_CTL1` writer"]
pub type W = crate::W<HOST_CTL1_SPEC>;
#[doc = "Field `CLKSEL` reader - This bit selects the operating clock of USB Host. '0' : Low-speed clock '1' : Full-speed clock Notes: - This bit is initialized if ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'. - This bit must always be set to '1' in the USB Device mode."]
pub type CLKSEL_R = crate::BitReader;
#[doc = "Field `CLKSEL` writer - This bit selects the operating clock of USB Host. '0' : Low-speed clock '1' : Full-speed clock Notes: - This bit is initialized if ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'. - This bit must always be set to '1' in the USB Device mode."]
pub type CLKSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USTP` reader - This bit stops the clock for the USB Host operating unit. When this bit is '1', power consumption can be reduced by configuring this bit. '0' : Normal mode. '1' : Stops the clock for the USB Host operating unit. Notes: - If this bit is set to '1', the function of USB Host can't be used because internal clock is stopped. - This bit is initialized if ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'."]
pub type USTP_R = crate::BitReader;
#[doc = "Field `USTP` writer - This bit stops the clock for the USB Host operating unit. When this bit is '1', power consumption can be reduced by configuring this bit. '0' : Normal mode. '1' : Stops the clock for the USB Host operating unit. Notes: - If this bit is set to '1', the function of USB Host can't be used because internal clock is stopped. - This bit is initialized if ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'."]
pub type USTP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST` reader - This bit resets this IP. '0' : Releases the reset for USB Host. '1' : Resets USB Host. Notes: - This bit is initialized if ENABLE bit of the Host Control 0 Register changes from '1' to '0'. - If this bit is set to '1', both the BFINI bits of the Host Endpoint 1 Control Register (HOST_EP1_CTL) and Host Endpoint 2 Control Register (HOST_EP2_CTL) are set to '1'."]
pub type RST_R = crate::BitReader;
#[doc = "Field `RST` writer - This bit resets this IP. '0' : Releases the reset for USB Host. '1' : Resets USB Host. Notes: - This bit is initialized if ENABLE bit of the Host Control 0 Register changes from '1' to '0'. - If this bit is set to '1', both the BFINI bits of the Host Endpoint 1 Control Register (HOST_EP1_CTL) and Host Endpoint 2 Control Register (HOST_EP2_CTL) are set to '1'."]
pub type RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit selects the operating clock of USB Host. '0' : Low-speed clock '1' : Full-speed clock Notes: - This bit is initialized if ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'. - This bit must always be set to '1' in the USB Device mode."]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit stops the clock for the USB Host operating unit. When this bit is '1', power consumption can be reduced by configuring this bit. '0' : Normal mode. '1' : Stops the clock for the USB Host operating unit. Notes: - If this bit is set to '1', the function of USB Host can't be used because internal clock is stopped. - This bit is initialized if ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'."]
    #[inline(always)]
    pub fn ustp(&self) -> USTP_R {
        USTP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit resets this IP. '0' : Releases the reset for USB Host. '1' : Resets USB Host. Notes: - This bit is initialized if ENABLE bit of the Host Control 0 Register changes from '1' to '0'. - If this bit is set to '1', both the BFINI bits of the Host Endpoint 1 Control Register (HOST_EP1_CTL) and Host Endpoint 2 Control Register (HOST_EP2_CTL) are set to '1'."]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit selects the operating clock of USB Host. '0' : Low-speed clock '1' : Full-speed clock Notes: - This bit is initialized if ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'. - This bit must always be set to '1' in the USB Device mode."]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> CLKSEL_W<HOST_CTL1_SPEC> {
        CLKSEL_W::new(self, 0)
    }
    #[doc = "Bit 1 - This bit stops the clock for the USB Host operating unit. When this bit is '1', power consumption can be reduced by configuring this bit. '0' : Normal mode. '1' : Stops the clock for the USB Host operating unit. Notes: - If this bit is set to '1', the function of USB Host can't be used because internal clock is stopped. - This bit is initialized if ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'."]
    #[inline(always)]
    #[must_use]
    pub fn ustp(&mut self) -> USTP_W<HOST_CTL1_SPEC> {
        USTP_W::new(self, 1)
    }
    #[doc = "Bit 7 - This bit resets this IP. '0' : Releases the reset for USB Host. '1' : Resets USB Host. Notes: - This bit is initialized if ENABLE bit of the Host Control 0 Register changes from '1' to '0'. - If this bit is set to '1', both the BFINI bits of the Host Endpoint 1 Control Register (HOST_EP1_CTL) and Host Endpoint 2 Control Register (HOST_EP2_CTL) are set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RST_W<HOST_CTL1_SPEC> {
        RST_W::new(self, 7)
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
#[doc = "Host Control 1 Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_ctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_ctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_CTL1_SPEC;
impl crate::RegisterSpec for HOST_CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_ctl1::R`](R) reader structure"]
impl crate::Readable for HOST_CTL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_ctl1::W`](W) writer structure"]
impl crate::Writable for HOST_CTL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOST_CTL1 to value 0x83"]
impl crate::Resettable for HOST_CTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0x83;
}
