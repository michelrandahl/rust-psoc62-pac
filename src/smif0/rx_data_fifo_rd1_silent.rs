#[doc = "Register `RX_DATA_FIFO_RD1_SILENT` reader"]
pub type R = crate::R<RX_DATA_FIFO_RD1_SILENT_SPEC>;
#[doc = "Field `DATA0` reader - RX data (read from RX data FIFO)."]
pub type DATA0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - RX data (read from RX data FIFO)."]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Receiver data FIFO silent read\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_data_fifo_rd1_silent::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_DATA_FIFO_RD1_SILENT_SPEC;
impl crate::RegisterSpec for RX_DATA_FIFO_RD1_SILENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_data_fifo_rd1_silent::R`](R) reader structure"]
impl crate::Readable for RX_DATA_FIFO_RD1_SILENT_SPEC {}
#[doc = "`reset()` method sets RX_DATA_FIFO_RD1_SILENT to value 0"]
impl crate::Resettable for RX_DATA_FIFO_RD1_SILENT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
