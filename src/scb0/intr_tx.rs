#[doc = "Register `INTR_TX` reader"]
pub type R = crate::R<INTR_TX_SPEC>;
#[doc = "Register `INTR_TX` writer"]
pub type W = crate::W<INTR_TX_SPEC>;
#[doc = "Field `TRIGGER` reader - N/A"]
pub type TRIGGER_R = crate::BitReader;
#[doc = "Field `TRIGGER` writer - N/A"]
pub type TRIGGER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOT_FULL` reader - N/A"]
pub type NOT_FULL_R = crate::BitReader;
#[doc = "Field `NOT_FULL` writer - N/A"]
pub type NOT_FULL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMPTY` reader - N/A"]
pub type EMPTY_R = crate::BitReader;
#[doc = "Field `EMPTY` writer - N/A"]
pub type EMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFLOW` reader - N/A"]
pub type OVERFLOW_R = crate::BitReader;
#[doc = "Field `OVERFLOW` writer - N/A"]
pub type OVERFLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERFLOW` reader - Attempt to read from an empty TX FIFO. This happens when the SCB is ready to transfer data and EMPTY is '1'. Only used in FIFO mode."]
pub type UNDERFLOW_R = crate::BitReader;
#[doc = "Field `UNDERFLOW` writer - Attempt to read from an empty TX FIFO. This happens when the SCB is ready to transfer data and EMPTY is '1'. Only used in FIFO mode."]
pub type UNDERFLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLOCKED` reader - CPU write can not get access to the EZ memory (EZ data access), due to an externally clocked EZ access."]
pub type BLOCKED_R = crate::BitReader;
#[doc = "Field `BLOCKED` writer - CPU write can not get access to the EZ memory (EZ data access), due to an externally clocked EZ access."]
pub type BLOCKED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_NACK` reader - N/A"]
pub type UART_NACK_R = crate::BitReader;
#[doc = "Field `UART_NACK` writer - N/A"]
pub type UART_NACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_DONE` reader - UART transmitter done event. This happens when the SCB is done transferring all data in the TX FIFO, and the last stop field is transmitted (both TX FIFO and transmit shifter register are empty)."]
pub type UART_DONE_R = crate::BitReader;
#[doc = "Field `UART_DONE` writer - UART transmitter done event. This happens when the SCB is done transferring all data in the TX FIFO, and the last stop field is transmitted (both TX FIFO and transmit shifter register are empty)."]
pub type UART_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_ARB_LOST` reader - UART lost arbitration: the value driven on the TX line is not the same as the value observed on the RX line. This condition event is usefull when transmitter and receiver share a TX/RX line. This is the case in LIN or SmartCard modes."]
pub type UART_ARB_LOST_R = crate::BitReader;
#[doc = "Field `UART_ARB_LOST` writer - UART lost arbitration: the value driven on the TX line is not the same as the value observed on the RX line. This condition event is usefull when transmitter and receiver share a TX/RX line. This is the case in LIN or SmartCard modes."]
pub type UART_ARB_LOST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - N/A"]
    #[inline(always)]
    pub fn trigger(&self) -> TRIGGER_R {
        TRIGGER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn not_full(&self) -> NOT_FULL_R {
        NOT_FULL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn empty(&self) -> EMPTY_R {
        EMPTY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn overflow(&self) -> OVERFLOW_R {
        OVERFLOW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Attempt to read from an empty TX FIFO. This happens when the SCB is ready to transfer data and EMPTY is '1'. Only used in FIFO mode."]
    #[inline(always)]
    pub fn underflow(&self) -> UNDERFLOW_R {
        UNDERFLOW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CPU write can not get access to the EZ memory (EZ data access), due to an externally clocked EZ access."]
    #[inline(always)]
    pub fn blocked(&self) -> BLOCKED_R {
        BLOCKED_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - N/A"]
    #[inline(always)]
    pub fn uart_nack(&self) -> UART_NACK_R {
        UART_NACK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UART transmitter done event. This happens when the SCB is done transferring all data in the TX FIFO, and the last stop field is transmitted (both TX FIFO and transmit shifter register are empty)."]
    #[inline(always)]
    pub fn uart_done(&self) -> UART_DONE_R {
        UART_DONE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - UART lost arbitration: the value driven on the TX line is not the same as the value observed on the RX line. This condition event is usefull when transmitter and receiver share a TX/RX line. This is the case in LIN or SmartCard modes."]
    #[inline(always)]
    pub fn uart_arb_lost(&self) -> UART_ARB_LOST_R {
        UART_ARB_LOST_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn trigger(&mut self) -> TRIGGER_W<INTR_TX_SPEC> {
        TRIGGER_W::new(self, 0)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn not_full(&mut self) -> NOT_FULL_W<INTR_TX_SPEC> {
        NOT_FULL_W::new(self, 1)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn empty(&mut self) -> EMPTY_W<INTR_TX_SPEC> {
        EMPTY_W::new(self, 4)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn overflow(&mut self) -> OVERFLOW_W<INTR_TX_SPEC> {
        OVERFLOW_W::new(self, 5)
    }
    #[doc = "Bit 6 - Attempt to read from an empty TX FIFO. This happens when the SCB is ready to transfer data and EMPTY is '1'. Only used in FIFO mode."]
    #[inline(always)]
    #[must_use]
    pub fn underflow(&mut self) -> UNDERFLOW_W<INTR_TX_SPEC> {
        UNDERFLOW_W::new(self, 6)
    }
    #[doc = "Bit 7 - CPU write can not get access to the EZ memory (EZ data access), due to an externally clocked EZ access."]
    #[inline(always)]
    #[must_use]
    pub fn blocked(&mut self) -> BLOCKED_W<INTR_TX_SPEC> {
        BLOCKED_W::new(self, 7)
    }
    #[doc = "Bit 8 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn uart_nack(&mut self) -> UART_NACK_W<INTR_TX_SPEC> {
        UART_NACK_W::new(self, 8)
    }
    #[doc = "Bit 9 - UART transmitter done event. This happens when the SCB is done transferring all data in the TX FIFO, and the last stop field is transmitted (both TX FIFO and transmit shifter register are empty)."]
    #[inline(always)]
    #[must_use]
    pub fn uart_done(&mut self) -> UART_DONE_W<INTR_TX_SPEC> {
        UART_DONE_W::new(self, 9)
    }
    #[doc = "Bit 10 - UART lost arbitration: the value driven on the TX line is not the same as the value observed on the RX line. This condition event is usefull when transmitter and receiver share a TX/RX line. This is the case in LIN or SmartCard modes."]
    #[inline(always)]
    #[must_use]
    pub fn uart_arb_lost(&mut self) -> UART_ARB_LOST_W<INTR_TX_SPEC> {
        UART_ARB_LOST_W::new(self, 10)
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
#[doc = "Transmitter interrupt request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_tx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_tx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_TX_SPEC;
impl crate::RegisterSpec for INTR_TX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_tx::R`](R) reader structure"]
impl crate::Readable for INTR_TX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr_tx::W`](W) writer structure"]
impl crate::Writable for INTR_TX_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_TX to value 0"]
impl crate::Resettable for INTR_TX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
