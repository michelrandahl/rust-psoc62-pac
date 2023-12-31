#[doc = "Register `UART_TX_CTRL` reader"]
pub type R = crate::R<UART_TX_CTRL_SPEC>;
#[doc = "Register `UART_TX_CTRL` writer"]
pub type W = crate::W<UART_TX_CTRL_SPEC>;
#[doc = "Field `STOP_BITS` reader - Stop bits. STOP_BITS + 1 is the duration of the stop period in terms of half bit periods. Valid range is \\[1, 7\\]; i.e. a stop period should last at least one bit period."]
pub type STOP_BITS_R = crate::FieldReader;
#[doc = "Field `STOP_BITS` writer - Stop bits. STOP_BITS + 1 is the duration of the stop period in terms of half bit periods. Valid range is \\[1, 7\\]; i.e. a stop period should last at least one bit period."]
pub type STOP_BITS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PARITY` reader - N/A"]
pub type PARITY_R = crate::BitReader;
#[doc = "Field `PARITY` writer - N/A"]
pub type PARITY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARITY_ENABLED` reader - N/A"]
pub type PARITY_ENABLED_R = crate::BitReader;
#[doc = "Field `PARITY_ENABLED` writer - N/A"]
pub type PARITY_ENABLED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETRY_ON_NACK` reader - N/A"]
pub type RETRY_ON_NACK_R = crate::BitReader;
#[doc = "Field `RETRY_ON_NACK` writer - N/A"]
pub type RETRY_ON_NACK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Stop bits. STOP_BITS + 1 is the duration of the stop period in terms of half bit periods. Valid range is \\[1, 7\\]; i.e. a stop period should last at least one bit period."]
    #[inline(always)]
    pub fn stop_bits(&self) -> STOP_BITS_R {
        STOP_BITS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn parity_enabled(&self) -> PARITY_ENABLED_R {
        PARITY_ENABLED_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - N/A"]
    #[inline(always)]
    pub fn retry_on_nack(&self) -> RETRY_ON_NACK_R {
        RETRY_ON_NACK_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Stop bits. STOP_BITS + 1 is the duration of the stop period in terms of half bit periods. Valid range is \\[1, 7\\]; i.e. a stop period should last at least one bit period."]
    #[inline(always)]
    #[must_use]
    pub fn stop_bits(&mut self) -> STOP_BITS_W<UART_TX_CTRL_SPEC> {
        STOP_BITS_W::new(self, 0)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn parity(&mut self) -> PARITY_W<UART_TX_CTRL_SPEC> {
        PARITY_W::new(self, 4)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn parity_enabled(&mut self) -> PARITY_ENABLED_W<UART_TX_CTRL_SPEC> {
        PARITY_ENABLED_W::new(self, 5)
    }
    #[doc = "Bit 8 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn retry_on_nack(&mut self) -> RETRY_ON_NACK_W<UART_TX_CTRL_SPEC> {
        RETRY_ON_NACK_W::new(self, 8)
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
#[doc = "UART transmitter control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_tx_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_tx_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UART_TX_CTRL_SPEC;
impl crate::RegisterSpec for UART_TX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_tx_ctrl::R`](R) reader structure"]
impl crate::Readable for UART_TX_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uart_tx_ctrl::W`](W) writer structure"]
impl crate::Writable for UART_TX_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UART_TX_CTRL to value 0x02"]
impl crate::Resettable for UART_TX_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
