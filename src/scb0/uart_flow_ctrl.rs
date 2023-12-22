#[doc = "Register `UART_FLOW_CTRL` reader"]
pub type R = crate::R<UART_FLOW_CTRL_SPEC>;
#[doc = "Register `UART_FLOW_CTRL` writer"]
pub type W = crate::W<UART_FLOW_CTRL_SPEC>;
#[doc = "Field `TRIGGER_LEVEL` reader - Trigger level. When the receiver FIFO has less entries than the amount of this field, a Ready To Send (RTS) output signal is activated. By setting this field to '0', flow control is disabled"]
pub type TRIGGER_LEVEL_R = crate::FieldReader;
#[doc = "Field `TRIGGER_LEVEL` writer - Trigger level. When the receiver FIFO has less entries than the amount of this field, a Ready To Send (RTS) output signal is activated. By setting this field to '0', flow control is disabled"]
pub type TRIGGER_LEVEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RTS_POLARITY` reader - Polarity of the RTS output signal: '0': RTS is active low; '1': RTS is active high; During SCB reset (Hibernate system power mode), RTS output signal is '1'. This represents an inactive state assuming an active low polarity."]
pub type RTS_POLARITY_R = crate::BitReader;
#[doc = "Field `RTS_POLARITY` writer - Polarity of the RTS output signal: '0': RTS is active low; '1': RTS is active high; During SCB reset (Hibernate system power mode), RTS output signal is '1'. This represents an inactive state assuming an active low polarity."]
pub type RTS_POLARITY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTS_POLARITY` reader - Polarity of the CTS input signal '0': CTS is active low ; '1': CTS is active high;"]
pub type CTS_POLARITY_R = crate::BitReader;
#[doc = "Field `CTS_POLARITY` writer - Polarity of the CTS input signal '0': CTS is active low ; '1': CTS is active high;"]
pub type CTS_POLARITY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTS_ENABLED` reader - Enable use of CTS input signal by the UART transmitter: '0': Disabled. The UART transmitter ignores the CTS input signal and transmits when a data frame is available for transmission in the TX FIFO or the TX shift register. '1': Enabled. The UART transmitter uses CTS input signal to qualify the transmission of data. It transmits when CTS input signal is active and a data frame is available for transmission in the TX FIFO or the TX shift register. If UART_CTRL.LOOPBACK is '1', the CTS input signal is driven by the RTS output signal locally in SCB (both signals are subjected to signal polarity changes as indicated by RTS_POLARITY and CTS_POLARITY)."]
pub type CTS_ENABLED_R = crate::BitReader;
#[doc = "Field `CTS_ENABLED` writer - Enable use of CTS input signal by the UART transmitter: '0': Disabled. The UART transmitter ignores the CTS input signal and transmits when a data frame is available for transmission in the TX FIFO or the TX shift register. '1': Enabled. The UART transmitter uses CTS input signal to qualify the transmission of data. It transmits when CTS input signal is active and a data frame is available for transmission in the TX FIFO or the TX shift register. If UART_CTRL.LOOPBACK is '1', the CTS input signal is driven by the RTS output signal locally in SCB (both signals are subjected to signal polarity changes as indicated by RTS_POLARITY and CTS_POLARITY)."]
pub type CTS_ENABLED_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Trigger level. When the receiver FIFO has less entries than the amount of this field, a Ready To Send (RTS) output signal is activated. By setting this field to '0', flow control is disabled"]
    #[inline(always)]
    pub fn trigger_level(&self) -> TRIGGER_LEVEL_R {
        TRIGGER_LEVEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - Polarity of the RTS output signal: '0': RTS is active low; '1': RTS is active high; During SCB reset (Hibernate system power mode), RTS output signal is '1'. This represents an inactive state assuming an active low polarity."]
    #[inline(always)]
    pub fn rts_polarity(&self) -> RTS_POLARITY_R {
        RTS_POLARITY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Polarity of the CTS input signal '0': CTS is active low ; '1': CTS is active high;"]
    #[inline(always)]
    pub fn cts_polarity(&self) -> CTS_POLARITY_R {
        CTS_POLARITY_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable use of CTS input signal by the UART transmitter: '0': Disabled. The UART transmitter ignores the CTS input signal and transmits when a data frame is available for transmission in the TX FIFO or the TX shift register. '1': Enabled. The UART transmitter uses CTS input signal to qualify the transmission of data. It transmits when CTS input signal is active and a data frame is available for transmission in the TX FIFO or the TX shift register. If UART_CTRL.LOOPBACK is '1', the CTS input signal is driven by the RTS output signal locally in SCB (both signals are subjected to signal polarity changes as indicated by RTS_POLARITY and CTS_POLARITY)."]
    #[inline(always)]
    pub fn cts_enabled(&self) -> CTS_ENABLED_R {
        CTS_ENABLED_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Trigger level. When the receiver FIFO has less entries than the amount of this field, a Ready To Send (RTS) output signal is activated. By setting this field to '0', flow control is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn trigger_level(&mut self) -> TRIGGER_LEVEL_W<UART_FLOW_CTRL_SPEC> {
        TRIGGER_LEVEL_W::new(self, 0)
    }
    #[doc = "Bit 16 - Polarity of the RTS output signal: '0': RTS is active low; '1': RTS is active high; During SCB reset (Hibernate system power mode), RTS output signal is '1'. This represents an inactive state assuming an active low polarity."]
    #[inline(always)]
    #[must_use]
    pub fn rts_polarity(&mut self) -> RTS_POLARITY_W<UART_FLOW_CTRL_SPEC> {
        RTS_POLARITY_W::new(self, 16)
    }
    #[doc = "Bit 24 - Polarity of the CTS input signal '0': CTS is active low ; '1': CTS is active high;"]
    #[inline(always)]
    #[must_use]
    pub fn cts_polarity(&mut self) -> CTS_POLARITY_W<UART_FLOW_CTRL_SPEC> {
        CTS_POLARITY_W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable use of CTS input signal by the UART transmitter: '0': Disabled. The UART transmitter ignores the CTS input signal and transmits when a data frame is available for transmission in the TX FIFO or the TX shift register. '1': Enabled. The UART transmitter uses CTS input signal to qualify the transmission of data. It transmits when CTS input signal is active and a data frame is available for transmission in the TX FIFO or the TX shift register. If UART_CTRL.LOOPBACK is '1', the CTS input signal is driven by the RTS output signal locally in SCB (both signals are subjected to signal polarity changes as indicated by RTS_POLARITY and CTS_POLARITY)."]
    #[inline(always)]
    #[must_use]
    pub fn cts_enabled(&mut self) -> CTS_ENABLED_W<UART_FLOW_CTRL_SPEC> {
        CTS_ENABLED_W::new(self, 25)
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
#[doc = "UART flow control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_flow_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_flow_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UART_FLOW_CTRL_SPEC;
impl crate::RegisterSpec for UART_FLOW_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_flow_ctrl::R`](R) reader structure"]
impl crate::Readable for UART_FLOW_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uart_flow_ctrl::W`](W) writer structure"]
impl crate::Writable for UART_FLOW_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UART_FLOW_CTRL to value 0"]
impl crate::Resettable for UART_FLOW_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
