#[doc = "Register `UART_RX_STATUS` reader"]
pub type R = crate::R<UART_RX_STATUS_SPEC>;
#[doc = "Field `BR_COUNTER` reader - For LIN: Amount of clk_scb periods that constitute the transmission of a 0x55 data frame (sent least signficant bit first) as determined by the receiver. BR_COUNTER / 8 is the amount of clk_scb periods that constitute a bit period. This field has valid data when INTR_RX.BAUD_DETECT is set to '1'."]
pub type BR_COUNTER_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - For LIN: Amount of clk_scb periods that constitute the transmission of a 0x55 data frame (sent least signficant bit first) as determined by the receiver. BR_COUNTER / 8 is the amount of clk_scb periods that constitute a bit period. This field has valid data when INTR_RX.BAUD_DETECT is set to '1'."]
    #[inline(always)]
    pub fn br_counter(&self) -> BR_COUNTER_R {
        BR_COUNTER_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "UART receiver status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_rx_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UART_RX_STATUS_SPEC;
impl crate::RegisterSpec for UART_RX_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_rx_status::R`](R) reader structure"]
impl crate::Readable for UART_RX_STATUS_SPEC {}
#[doc = "`reset()` method sets UART_RX_STATUS to value 0"]
impl crate::Resettable for UART_RX_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
