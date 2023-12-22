#[doc = "Register `TX_FIFO_WR` writer"]
pub type W = crate::W<TX_FIFO_WR_SPEC>;
#[doc = "Field `DATA` writer - N/A"]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<TX_FIFO_WR_SPEC> {
        DATA_W::new(self, 0)
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
#[doc = "Transmitter FIFO write\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_fifo_wr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_FIFO_WR_SPEC;
impl crate::RegisterSpec for TX_FIFO_WR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tx_fifo_wr::W`](W) writer structure"]
impl crate::Writable for TX_FIFO_WR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_FIFO_WR to value 0"]
impl crate::Resettable for TX_FIFO_WR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
