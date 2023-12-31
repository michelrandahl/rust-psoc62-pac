#[doc = "Register `RX_DATA_FIFO_RD2` reader"]
pub type R = crate::R<RX_DATA_FIFO_RD2_SPEC>;
#[doc = "Field `DATA0` reader - RX data (read from RX data FIFO, first byte)."]
pub type DATA0_R = crate::FieldReader;
#[doc = "Field `DATA1` reader - RX data (read from RX data FIFO, second byte)."]
pub type DATA1_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - RX data (read from RX data FIFO, first byte)."]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - RX data (read from RX data FIFO, second byte)."]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Receiver data FIFO read\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_data_fifo_rd2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_DATA_FIFO_RD2_SPEC;
impl crate::RegisterSpec for RX_DATA_FIFO_RD2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_data_fifo_rd2::R`](R) reader structure"]
impl crate::Readable for RX_DATA_FIFO_RD2_SPEC {}
#[doc = "`reset()` method sets RX_DATA_FIFO_RD2 to value 0"]
impl crate::Resettable for RX_DATA_FIFO_RD2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
