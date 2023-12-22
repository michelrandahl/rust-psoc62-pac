#[doc = "Register `USBIO_CR0` reader"]
pub type R = crate::R<USBIO_CR0_SPEC>;
#[doc = "Register `USBIO_CR0` writer"]
pub type W = crate::W<USBIO_CR0_SPEC>;
#[doc = "Field `RD` reader - Received Data. This read only bit gives the state of the USB differential receiver when IOMODE bit is '0' and USB doesn't transmit. This bit is valid if USB Device. If D+=D- (SE0), this value is undefined."]
pub type RD_R = crate::BitReader<RD_A>;
#[doc = "Received Data. This read only bit gives the state of the USB differential receiver when IOMODE bit is '0' and USB doesn't transmit. This bit is valid if USB Device. If D+=D- (SE0), this value is undefined.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RD_A {
    #[doc = "0: D+ &lt; D- (K state)"]
    DIFF_LOW = 0,
    #[doc = "1: D+ > D- (J state)"]
    DIFF_HIGH = 1,
}
impl From<RD_A> for bool {
    #[inline(always)]
    fn from(variant: RD_A) -> Self {
        variant as u8 != 0
    }
}
impl RD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RD_A {
        match self.bits {
            false => RD_A::DIFF_LOW,
            true => RD_A::DIFF_HIGH,
        }
    }
    #[doc = "D+ &lt; D- (K state)"]
    #[inline(always)]
    pub fn is_diff_low(&self) -> bool {
        *self == RD_A::DIFF_LOW
    }
    #[doc = "D+ > D- (J state)"]
    #[inline(always)]
    pub fn is_diff_high(&self) -> bool {
        *self == RD_A::DIFF_HIGH
    }
}
#[doc = "Field `TD` reader - Transmit Data. Transmit a USB J or K state on the USB bus. No effect if TEN=0 or TSE0=1."]
pub type TD_R = crate::BitReader<TD_A>;
#[doc = "Transmit Data. Transmit a USB J or K state on the USB bus. No effect if TEN=0 or TSE0=1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TD_A {
    #[doc = "0: Force USB K state (D+ is low D- is high)."]
    DIFF_K = 0,
    #[doc = "1: Force USB J state (D+ is high D- is low)."]
    DIFF_J = 1,
}
impl From<TD_A> for bool {
    #[inline(always)]
    fn from(variant: TD_A) -> Self {
        variant as u8 != 0
    }
}
impl TD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TD_A {
        match self.bits {
            false => TD_A::DIFF_K,
            true => TD_A::DIFF_J,
        }
    }
    #[doc = "Force USB K state (D+ is low D- is high)."]
    #[inline(always)]
    pub fn is_diff_k(&self) -> bool {
        *self == TD_A::DIFF_K
    }
    #[doc = "Force USB J state (D+ is high D- is low)."]
    #[inline(always)]
    pub fn is_diff_j(&self) -> bool {
        *self == TD_A::DIFF_J
    }
}
#[doc = "Field `TD` writer - Transmit Data. Transmit a USB J or K state on the USB bus. No effect if TEN=0 or TSE0=1."]
pub type TD_W<'a, REG> = crate::BitWriter<'a, REG, TD_A>;
impl<'a, REG> TD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Force USB K state (D+ is low D- is high)."]
    #[inline(always)]
    pub fn diff_k(self) -> &'a mut crate::W<REG> {
        self.variant(TD_A::DIFF_K)
    }
    #[doc = "Force USB J state (D+ is high D- is low)."]
    #[inline(always)]
    pub fn diff_j(self) -> &'a mut crate::W<REG> {
        self.variant(TD_A::DIFF_J)
    }
}
#[doc = "Field `TSE0` reader - Transmit Single-Ended Zero. SE0: both D+ and D- low. No effect if TEN=0."]
pub type TSE0_R = crate::BitReader;
#[doc = "Field `TSE0` writer - Transmit Single-Ended Zero. SE0: both D+ and D- low. No effect if TEN=0."]
pub type TSE0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEN` reader - USB Transmit Enable. This is used to manually transmit on the D+ and D- pins. Normally this bit should be cleared to allow the internal SIE to drive the pins. The most common reason for manually transmitting is to force a resume state on the bus."]
pub type TEN_R = crate::BitReader;
#[doc = "Field `TEN` writer - USB Transmit Enable. This is used to manually transmit on the D+ and D- pins. Normally this bit should be cleared to allow the internal SIE to drive the pins. The most common reason for manually transmitting is to force a resume state on the bus."]
pub type TEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Received Data. This read only bit gives the state of the USB differential receiver when IOMODE bit is '0' and USB doesn't transmit. This bit is valid if USB Device. If D+=D- (SE0), this value is undefined."]
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Data. Transmit a USB J or K state on the USB bus. No effect if TEN=0 or TSE0=1."]
    #[inline(always)]
    pub fn td(&self) -> TD_R {
        TD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit Single-Ended Zero. SE0: both D+ and D- low. No effect if TEN=0."]
    #[inline(always)]
    pub fn tse0(&self) -> TSE0_R {
        TSE0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USB Transmit Enable. This is used to manually transmit on the D+ and D- pins. Normally this bit should be cleared to allow the internal SIE to drive the pins. The most common reason for manually transmitting is to force a resume state on the bus."]
    #[inline(always)]
    pub fn ten(&self) -> TEN_R {
        TEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Transmit Data. Transmit a USB J or K state on the USB bus. No effect if TEN=0 or TSE0=1."]
    #[inline(always)]
    #[must_use]
    pub fn td(&mut self) -> TD_W<USBIO_CR0_SPEC> {
        TD_W::new(self, 5)
    }
    #[doc = "Bit 6 - Transmit Single-Ended Zero. SE0: both D+ and D- low. No effect if TEN=0."]
    #[inline(always)]
    #[must_use]
    pub fn tse0(&mut self) -> TSE0_W<USBIO_CR0_SPEC> {
        TSE0_W::new(self, 6)
    }
    #[doc = "Bit 7 - USB Transmit Enable. This is used to manually transmit on the D+ and D- pins. Normally this bit should be cleared to allow the internal SIE to drive the pins. The most common reason for manually transmitting is to force a resume state on the bus."]
    #[inline(always)]
    #[must_use]
    pub fn ten(&mut self) -> TEN_W<USBIO_CR0_SPEC> {
        TEN_W::new(self, 7)
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
#[doc = "USBIO Control 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbio_cr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbio_cr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBIO_CR0_SPEC;
impl crate::RegisterSpec for USBIO_CR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbio_cr0::R`](R) reader structure"]
impl crate::Readable for USBIO_CR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbio_cr0::W`](W) writer structure"]
impl crate::Writable for USBIO_CR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBIO_CR0 to value 0"]
impl crate::Resettable for USBIO_CR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
