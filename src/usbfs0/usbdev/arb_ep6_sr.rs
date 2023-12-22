#[doc = "Register `ARB_EP6_SR` reader"]
pub type R = crate::R<ARB_EP6_SR_SPEC>;
#[doc = "Register `ARB_EP6_SR` writer"]
pub type W = crate::W<ARB_EP6_SR_SPEC>;
#[doc = "Field `IN_BUF_FULL` reader - IN Endpoint Local Buffer Full Interrupt"]
pub type IN_BUF_FULL_R = crate::BitReader;
#[doc = "Field `IN_BUF_FULL` writer - IN Endpoint Local Buffer Full Interrupt"]
pub type IN_BUF_FULL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_GNT` reader - Endpoint DMA Grant Interrupt"]
pub type DMA_GNT_R = crate::BitReader;
#[doc = "Field `DMA_GNT` writer - Endpoint DMA Grant Interrupt"]
pub type DMA_GNT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF_OVER` reader - Endpoint Buffer Overflow Interrupt"]
pub type BUF_OVER_R = crate::BitReader;
#[doc = "Field `BUF_OVER` writer - Endpoint Buffer Overflow Interrupt"]
pub type BUF_OVER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF_UNDER` reader - Endpoint Buffer Underflow Interrupt"]
pub type BUF_UNDER_R = crate::BitReader;
#[doc = "Field `BUF_UNDER` writer - Endpoint Buffer Underflow Interrupt"]
pub type BUF_UNDER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_TERMIN` reader - Endpoint DMA Terminated Interrupt"]
pub type DMA_TERMIN_R = crate::BitReader;
#[doc = "Field `DMA_TERMIN` writer - Endpoint DMA Terminated Interrupt"]
pub type DMA_TERMIN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IN Endpoint Local Buffer Full Interrupt"]
    #[inline(always)]
    pub fn in_buf_full(&self) -> IN_BUF_FULL_R {
        IN_BUF_FULL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint DMA Grant Interrupt"]
    #[inline(always)]
    pub fn dma_gnt(&self) -> DMA_GNT_R {
        DMA_GNT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Endpoint Buffer Overflow Interrupt"]
    #[inline(always)]
    pub fn buf_over(&self) -> BUF_OVER_R {
        BUF_OVER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Endpoint Buffer Underflow Interrupt"]
    #[inline(always)]
    pub fn buf_under(&self) -> BUF_UNDER_R {
        BUF_UNDER_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Endpoint DMA Terminated Interrupt"]
    #[inline(always)]
    pub fn dma_termin(&self) -> DMA_TERMIN_R {
        DMA_TERMIN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IN Endpoint Local Buffer Full Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn in_buf_full(&mut self) -> IN_BUF_FULL_W<ARB_EP6_SR_SPEC> {
        IN_BUF_FULL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint DMA Grant Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn dma_gnt(&mut self) -> DMA_GNT_W<ARB_EP6_SR_SPEC> {
        DMA_GNT_W::new(self, 1)
    }
    #[doc = "Bit 2 - Endpoint Buffer Overflow Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn buf_over(&mut self) -> BUF_OVER_W<ARB_EP6_SR_SPEC> {
        BUF_OVER_W::new(self, 2)
    }
    #[doc = "Bit 3 - Endpoint Buffer Underflow Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn buf_under(&mut self) -> BUF_UNDER_W<ARB_EP6_SR_SPEC> {
        BUF_UNDER_W::new(self, 3)
    }
    #[doc = "Bit 5 - Endpoint DMA Terminated Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn dma_termin(&mut self) -> DMA_TERMIN_W<ARB_EP6_SR_SPEC> {
        DMA_TERMIN_W::new(self, 5)
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
#[doc = "Endpoint Interrupt Enable Register *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_ep6_sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_ep6_sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARB_EP6_SR_SPEC;
impl crate::RegisterSpec for ARB_EP6_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arb_ep6_sr::R`](R) reader structure"]
impl crate::Readable for ARB_EP6_SR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`arb_ep6_sr::W`](W) writer structure"]
impl crate::Writable for ARB_EP6_SR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ARB_EP6_SR to value 0"]
impl crate::Resettable for ARB_EP6_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
