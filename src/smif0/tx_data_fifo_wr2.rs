#[doc = "Register `TX_DATA_FIFO_WR2` writer"]
pub type W = crate::W<TX_DATA_FIFO_WR2_SPEC>;
#[doc = "Field `DATA0` writer - TX data (written to TX data FIFO, first byte)."]
pub type DATA0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA1` writer - TX data (written to TX data FIFO, second byte)."]
pub type DATA1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - TX data (written to TX data FIFO, first byte)."]
    #[inline(always)]
    #[must_use]
    pub fn data0(&mut self) -> DATA0_W<TX_DATA_FIFO_WR2_SPEC> {
        DATA0_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - TX data (written to TX data FIFO, second byte)."]
    #[inline(always)]
    #[must_use]
    pub fn data1(&mut self) -> DATA1_W<TX_DATA_FIFO_WR2_SPEC> {
        DATA1_W::new(self, 8)
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
#[doc = "Transmitter data FIFO write\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_data_fifo_wr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_DATA_FIFO_WR2_SPEC;
impl crate::RegisterSpec for TX_DATA_FIFO_WR2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tx_data_fifo_wr2::W`](W) writer structure"]
impl crate::Writable for TX_DATA_FIFO_WR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_DATA_FIFO_WR2 to value 0"]
impl crate::Resettable for TX_DATA_FIFO_WR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
