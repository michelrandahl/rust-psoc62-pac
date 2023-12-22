#[doc = "Register `INTR_RX` reader"]
pub type R = crate::R<INTR_RX_SPEC>;
#[doc = "Register `INTR_RX` writer"]
pub type W = crate::W<INTR_RX_SPEC>;
#[doc = "Field `TRIGGER` reader - N/A"]
pub type TRIGGER_R = crate::BitReader;
#[doc = "Field `TRIGGER` writer - N/A"]
pub type TRIGGER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOT_EMPTY` reader - N/A"]
pub type NOT_EMPTY_R = crate::BitReader;
#[doc = "Field `NOT_EMPTY` writer - N/A"]
pub type NOT_EMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FULL` reader - N/A"]
pub type FULL_R = crate::BitReader;
#[doc = "Field `FULL` writer - N/A"]
pub type FULL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFLOW` reader - N/A"]
pub type OVERFLOW_R = crate::BitReader;
#[doc = "Field `OVERFLOW` writer - N/A"]
pub type OVERFLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERFLOW` reader - N/A"]
pub type UNDERFLOW_R = crate::BitReader;
#[doc = "Field `UNDERFLOW` writer - N/A"]
pub type UNDERFLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLOCKED` reader - CPU read transfer can not get access to the EZ memory (EZ_DATA accesses), due to an externally clocked EZ access."]
pub type BLOCKED_R = crate::BitReader;
#[doc = "Field `BLOCKED` writer - CPU read transfer can not get access to the EZ memory (EZ_DATA accesses), due to an externally clocked EZ access."]
pub type BLOCKED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME_ERROR` reader - UART Frame error in received data frame. This can be either a start or stop bit(s) error: Start bit error: after the detection of the beginning of a start bit period (RX line changes from '1' to '0'), the middle of the start bit period is sampled erroneously (RX line is '1'). Note: a start bit error is detected BEFORE a data frame is received. Stop bit error: the RX line is sampled as '0', but a '1' was expected. Note: a stop bit error may result in failure to receive successive data frame(s). Note: a stop bit error is detected AFTER a data frame is received. A stop bit error is detected after a data frame is received, and the UART_RX_CTL.DROP_ON_FRAME_ERROR field specifies whether the received frame is dropped or send to the RX FIFO. If UART_RX_CTL.DROP_ON_FRAME_ERROR is '1', the received data frame is dropped. If UART_RX_CTL.DROP_ON_FRAME_ERROR is '0', the received data frame is send to the RX FIFO. Note that Firmware can only identify the erroneous data frame in the RX FIFO if it is fast enough to read the data frame before the hardware writes a next data frame into the RX FIFO; i.e. the RX FIFO does not have error flags to tag erroneous data frames."]
pub type FRAME_ERROR_R = crate::BitReader;
#[doc = "Field `FRAME_ERROR` writer - UART Frame error in received data frame. This can be either a start or stop bit(s) error: Start bit error: after the detection of the beginning of a start bit period (RX line changes from '1' to '0'), the middle of the start bit period is sampled erroneously (RX line is '1'). Note: a start bit error is detected BEFORE a data frame is received. Stop bit error: the RX line is sampled as '0', but a '1' was expected. Note: a stop bit error may result in failure to receive successive data frame(s). Note: a stop bit error is detected AFTER a data frame is received. A stop bit error is detected after a data frame is received, and the UART_RX_CTL.DROP_ON_FRAME_ERROR field specifies whether the received frame is dropped or send to the RX FIFO. If UART_RX_CTL.DROP_ON_FRAME_ERROR is '1', the received data frame is dropped. If UART_RX_CTL.DROP_ON_FRAME_ERROR is '0', the received data frame is send to the RX FIFO. Note that Firmware can only identify the erroneous data frame in the RX FIFO if it is fast enough to read the data frame before the hardware writes a next data frame into the RX FIFO; i.e. the RX FIFO does not have error flags to tag erroneous data frames."]
pub type FRAME_ERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARITY_ERROR` reader - UART Parity error in received data frame. If UART_RX_CTL.DROP_ON_PARITY_ERROR is '1', the received frame is dropped. If UART_RX_CTL.DROP_ON_PARITY_ERROR is '0', the received frame is send to the RX FIFO. In SmartCard submode, negatively acknowledged data frames generate a parity error. Note that Firmware can only identify the erroneous data frame in the RX FIFO if it is fast enough to read the data frame before the hardware writes a next data frame into the RX FIFO."]
pub type PARITY_ERROR_R = crate::BitReader;
#[doc = "Field `PARITY_ERROR` writer - UART Parity error in received data frame. If UART_RX_CTL.DROP_ON_PARITY_ERROR is '1', the received frame is dropped. If UART_RX_CTL.DROP_ON_PARITY_ERROR is '0', the received frame is send to the RX FIFO. In SmartCard submode, negatively acknowledged data frames generate a parity error. Note that Firmware can only identify the erroneous data frame in the RX FIFO if it is fast enough to read the data frame before the hardware writes a next data frame into the RX FIFO."]
pub type PARITY_ERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BAUD_DETECT` reader - LIN baudrate detection is completed. The receiver software uses the UART_RX_STATUS.BR_COUNTER value to set the clk_scb to guarantee successful receipt of the first LIN data frame (Protected Identifier Field) after the synchronization byte."]
pub type BAUD_DETECT_R = crate::BitReader;
#[doc = "Field `BAUD_DETECT` writer - LIN baudrate detection is completed. The receiver software uses the UART_RX_STATUS.BR_COUNTER value to set the clk_scb to guarantee successful receipt of the first LIN data frame (Protected Identifier Field) after the synchronization byte."]
pub type BAUD_DETECT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BREAK_DETECT` reader - Break detection is successful: the line is '0' for UART_RX_CTRL.BREAK_WIDTH + 1 bit period. Can occur at any time to address unanticipated break fields; i.e. 'break-in-data' is supported. This feature is supported for the UART standard and LIN submodes. For the UART standard submodes, ongoing receipt of data frames is NOT affected; i.e. Firmware is expected to take the proper action. For the LIN submode, possible ongoing receipt of a data frame is stopped and the (partially) received data frame is dropped and baud rate detection is started. Set to '1', when event is detected. Write with '1' to clear bit."]
pub type BREAK_DETECT_R = crate::BitReader;
#[doc = "Field `BREAK_DETECT` writer - Break detection is successful: the line is '0' for UART_RX_CTRL.BREAK_WIDTH + 1 bit period. Can occur at any time to address unanticipated break fields; i.e. 'break-in-data' is supported. This feature is supported for the UART standard and LIN submodes. For the UART standard submodes, ongoing receipt of data frames is NOT affected; i.e. Firmware is expected to take the proper action. For the LIN submode, possible ongoing receipt of a data frame is stopped and the (partially) received data frame is dropped and baud rate detection is started. Set to '1', when event is detected. Write with '1' to clear bit."]
pub type BREAK_DETECT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - N/A"]
    #[inline(always)]
    pub fn trigger(&self) -> TRIGGER_R {
        TRIGGER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn not_empty(&self) -> NOT_EMPTY_R {
        NOT_EMPTY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn full(&self) -> FULL_R {
        FULL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn overflow(&self) -> OVERFLOW_R {
        OVERFLOW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn underflow(&self) -> UNDERFLOW_R {
        UNDERFLOW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CPU read transfer can not get access to the EZ memory (EZ_DATA accesses), due to an externally clocked EZ access."]
    #[inline(always)]
    pub fn blocked(&self) -> BLOCKED_R {
        BLOCKED_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UART Frame error in received data frame. This can be either a start or stop bit(s) error: Start bit error: after the detection of the beginning of a start bit period (RX line changes from '1' to '0'), the middle of the start bit period is sampled erroneously (RX line is '1'). Note: a start bit error is detected BEFORE a data frame is received. Stop bit error: the RX line is sampled as '0', but a '1' was expected. Note: a stop bit error may result in failure to receive successive data frame(s). Note: a stop bit error is detected AFTER a data frame is received. A stop bit error is detected after a data frame is received, and the UART_RX_CTL.DROP_ON_FRAME_ERROR field specifies whether the received frame is dropped or send to the RX FIFO. If UART_RX_CTL.DROP_ON_FRAME_ERROR is '1', the received data frame is dropped. If UART_RX_CTL.DROP_ON_FRAME_ERROR is '0', the received data frame is send to the RX FIFO. Note that Firmware can only identify the erroneous data frame in the RX FIFO if it is fast enough to read the data frame before the hardware writes a next data frame into the RX FIFO; i.e. the RX FIFO does not have error flags to tag erroneous data frames."]
    #[inline(always)]
    pub fn frame_error(&self) -> FRAME_ERROR_R {
        FRAME_ERROR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UART Parity error in received data frame. If UART_RX_CTL.DROP_ON_PARITY_ERROR is '1', the received frame is dropped. If UART_RX_CTL.DROP_ON_PARITY_ERROR is '0', the received frame is send to the RX FIFO. In SmartCard submode, negatively acknowledged data frames generate a parity error. Note that Firmware can only identify the erroneous data frame in the RX FIFO if it is fast enough to read the data frame before the hardware writes a next data frame into the RX FIFO."]
    #[inline(always)]
    pub fn parity_error(&self) -> PARITY_ERROR_R {
        PARITY_ERROR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LIN baudrate detection is completed. The receiver software uses the UART_RX_STATUS.BR_COUNTER value to set the clk_scb to guarantee successful receipt of the first LIN data frame (Protected Identifier Field) after the synchronization byte."]
    #[inline(always)]
    pub fn baud_detect(&self) -> BAUD_DETECT_R {
        BAUD_DETECT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Break detection is successful: the line is '0' for UART_RX_CTRL.BREAK_WIDTH + 1 bit period. Can occur at any time to address unanticipated break fields; i.e. 'break-in-data' is supported. This feature is supported for the UART standard and LIN submodes. For the UART standard submodes, ongoing receipt of data frames is NOT affected; i.e. Firmware is expected to take the proper action. For the LIN submode, possible ongoing receipt of a data frame is stopped and the (partially) received data frame is dropped and baud rate detection is started. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn break_detect(&self) -> BREAK_DETECT_R {
        BREAK_DETECT_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn trigger(&mut self) -> TRIGGER_W<INTR_RX_SPEC> {
        TRIGGER_W::new(self, 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn not_empty(&mut self) -> NOT_EMPTY_W<INTR_RX_SPEC> {
        NOT_EMPTY_W::new(self, 2)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn full(&mut self) -> FULL_W<INTR_RX_SPEC> {
        FULL_W::new(self, 3)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn overflow(&mut self) -> OVERFLOW_W<INTR_RX_SPEC> {
        OVERFLOW_W::new(self, 5)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn underflow(&mut self) -> UNDERFLOW_W<INTR_RX_SPEC> {
        UNDERFLOW_W::new(self, 6)
    }
    #[doc = "Bit 7 - CPU read transfer can not get access to the EZ memory (EZ_DATA accesses), due to an externally clocked EZ access."]
    #[inline(always)]
    #[must_use]
    pub fn blocked(&mut self) -> BLOCKED_W<INTR_RX_SPEC> {
        BLOCKED_W::new(self, 7)
    }
    #[doc = "Bit 8 - UART Frame error in received data frame. This can be either a start or stop bit(s) error: Start bit error: after the detection of the beginning of a start bit period (RX line changes from '1' to '0'), the middle of the start bit period is sampled erroneously (RX line is '1'). Note: a start bit error is detected BEFORE a data frame is received. Stop bit error: the RX line is sampled as '0', but a '1' was expected. Note: a stop bit error may result in failure to receive successive data frame(s). Note: a stop bit error is detected AFTER a data frame is received. A stop bit error is detected after a data frame is received, and the UART_RX_CTL.DROP_ON_FRAME_ERROR field specifies whether the received frame is dropped or send to the RX FIFO. If UART_RX_CTL.DROP_ON_FRAME_ERROR is '1', the received data frame is dropped. If UART_RX_CTL.DROP_ON_FRAME_ERROR is '0', the received data frame is send to the RX FIFO. Note that Firmware can only identify the erroneous data frame in the RX FIFO if it is fast enough to read the data frame before the hardware writes a next data frame into the RX FIFO; i.e. the RX FIFO does not have error flags to tag erroneous data frames."]
    #[inline(always)]
    #[must_use]
    pub fn frame_error(&mut self) -> FRAME_ERROR_W<INTR_RX_SPEC> {
        FRAME_ERROR_W::new(self, 8)
    }
    #[doc = "Bit 9 - UART Parity error in received data frame. If UART_RX_CTL.DROP_ON_PARITY_ERROR is '1', the received frame is dropped. If UART_RX_CTL.DROP_ON_PARITY_ERROR is '0', the received frame is send to the RX FIFO. In SmartCard submode, negatively acknowledged data frames generate a parity error. Note that Firmware can only identify the erroneous data frame in the RX FIFO if it is fast enough to read the data frame before the hardware writes a next data frame into the RX FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn parity_error(&mut self) -> PARITY_ERROR_W<INTR_RX_SPEC> {
        PARITY_ERROR_W::new(self, 9)
    }
    #[doc = "Bit 10 - LIN baudrate detection is completed. The receiver software uses the UART_RX_STATUS.BR_COUNTER value to set the clk_scb to guarantee successful receipt of the first LIN data frame (Protected Identifier Field) after the synchronization byte."]
    #[inline(always)]
    #[must_use]
    pub fn baud_detect(&mut self) -> BAUD_DETECT_W<INTR_RX_SPEC> {
        BAUD_DETECT_W::new(self, 10)
    }
    #[doc = "Bit 11 - Break detection is successful: the line is '0' for UART_RX_CTRL.BREAK_WIDTH + 1 bit period. Can occur at any time to address unanticipated break fields; i.e. 'break-in-data' is supported. This feature is supported for the UART standard and LIN submodes. For the UART standard submodes, ongoing receipt of data frames is NOT affected; i.e. Firmware is expected to take the proper action. For the LIN submode, possible ongoing receipt of a data frame is stopped and the (partially) received data frame is dropped and baud rate detection is started. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn break_detect(&mut self) -> BREAK_DETECT_W<INTR_RX_SPEC> {
        BREAK_DETECT_W::new(self, 11)
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
#[doc = "Receiver interrupt request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_rx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_rx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_RX_SPEC;
impl crate::RegisterSpec for INTR_RX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_rx::R`](R) reader structure"]
impl crate::Readable for INTR_RX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr_rx::W`](W) writer structure"]
impl crate::Writable for INTR_RX_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_RX to value 0"]
impl crate::Resettable for INTR_RX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
