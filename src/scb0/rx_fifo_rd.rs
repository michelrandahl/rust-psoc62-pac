#[doc = "Register `RX_FIFO_RD` reader"]
pub type R = crate::R<RX_FIFO_RD_SPEC>;
#[doc = "Field `DATA` reader - Data read from the receiver FIFO. Reading a data frame will remove the data frame from the FIFO; i.e. behavior is similar to that of a POP operation. Note that when CTRL.BYTE_MODE is '1', only DATA\\[7:0\\]
are used. When in debug mode a read from this register behaves as a read from the SCB_RX_FIFO_RD_SILENT register. That is data will not be removed from the FIFO A read from an empty RX FIFO sets INTR_RX.UNDERFLOW to '1'."]
pub type DATA_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Data read from the receiver FIFO. Reading a data frame will remove the data frame from the FIFO; i.e. behavior is similar to that of a POP operation. Note that when CTRL.BYTE_MODE is '1', only DATA\\[7:0\\]
are used. When in debug mode a read from this register behaves as a read from the SCB_RX_FIFO_RD_SILENT register. That is data will not be removed from the FIFO A read from an empty RX FIFO sets INTR_RX.UNDERFLOW to '1'."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Receiver FIFO read\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_fifo_rd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_FIFO_RD_SPEC;
impl crate::RegisterSpec for RX_FIFO_RD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_fifo_rd::R`](R) reader structure"]
impl crate::Readable for RX_FIFO_RD_SPEC {}
#[doc = "`reset()` method sets RX_FIFO_RD to value 0"]
impl crate::Resettable for RX_FIFO_RD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
