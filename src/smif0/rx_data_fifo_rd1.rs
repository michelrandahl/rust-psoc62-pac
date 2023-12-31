#[doc = "Register `RX_DATA_FIFO_RD1` reader"]
pub type R = crate::R<RX_DATA_FIFO_RD1_SPEC>;
#[doc = "Field `DATA0` reader - RX data (read from RX data FIFO)."]
pub type DATA0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - RX data (read from RX data FIFO)."]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Receiver data FIFO read\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_data_fifo_rd1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_DATA_FIFO_RD1_SPEC;
impl crate::RegisterSpec for RX_DATA_FIFO_RD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_data_fifo_rd1::R`](R) reader structure"]
impl crate::Readable for RX_DATA_FIFO_RD1_SPEC {}
#[doc = "`reset()` method sets RX_DATA_FIFO_RD1 to value 0"]
impl crate::Resettable for RX_DATA_FIFO_RD1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
