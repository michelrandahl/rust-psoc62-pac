#[doc = "Register `RX_FIFO_RD_SILENT` reader"]
pub type R = crate::R<RX_FIFO_RD_SILENT_SPEC>;
#[doc = "Field `DATA` reader - Data read from the RX FIFO. Reading a data frame will NOT remove the data frame from the RX FIFO; i.e. behavior is similar to that of a PEEK operation. This field is used only for debugging purposes. Note: Don't access to this bit while RX_FIFO_CTL.CLEAR is '1'."]
pub type DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data read from the RX FIFO. Reading a data frame will NOT remove the data frame from the RX FIFO; i.e. behavior is similar to that of a PEEK operation. This field is used only for debugging purposes. Note: Don't access to this bit while RX_FIFO_CTL.CLEAR is '1'."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
#[doc = "RX FIFO silent read\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_fifo_rd_silent::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_FIFO_RD_SILENT_SPEC;
impl crate::RegisterSpec for RX_FIFO_RD_SILENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_fifo_rd_silent::R`](R) reader structure"]
impl crate::Readable for RX_FIFO_RD_SILENT_SPEC {}
#[doc = "`reset()` method sets RX_FIFO_RD_SILENT to value 0"]
impl crate::Resettable for RX_FIFO_RD_SILENT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
