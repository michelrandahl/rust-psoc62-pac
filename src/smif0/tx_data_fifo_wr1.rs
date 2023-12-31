#[doc = "Register `TX_DATA_FIFO_WR1` writer"]
pub type W = crate::W<TX_DATA_FIFO_WR1_SPEC>;
#[doc = "Field `DATA0` writer - TX data (written to TX data FIFO)."]
pub type DATA0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - TX data (written to TX data FIFO)."]
    #[inline(always)]
    #[must_use]
    pub fn data0(&mut self) -> DATA0_W<TX_DATA_FIFO_WR1_SPEC> {
        DATA0_W::new(self, 0)
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
#[doc = "Transmitter data FIFO write\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_data_fifo_wr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_DATA_FIFO_WR1_SPEC;
impl crate::RegisterSpec for TX_DATA_FIFO_WR1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tx_data_fifo_wr1::W`](W) writer structure"]
impl crate::Writable for TX_DATA_FIFO_WR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_DATA_FIFO_WR1 to value 0"]
impl crate::Resettable for TX_DATA_FIFO_WR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
