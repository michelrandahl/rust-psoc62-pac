#[doc = "Register `ARB_EP6_INT_EN` reader"]
pub type R = crate::R<ARB_EP6_INT_EN_SPEC>;
#[doc = "Register `ARB_EP6_INT_EN` writer"]
pub type W = crate::W<ARB_EP6_INT_EN_SPEC>;
#[doc = "Field `IN_BUF_FULL_EN` reader - IN Endpoint Local Buffer Full Enable"]
pub type IN_BUF_FULL_EN_R = crate::BitReader;
#[doc = "Field `IN_BUF_FULL_EN` writer - IN Endpoint Local Buffer Full Enable"]
pub type IN_BUF_FULL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_GNT_EN` reader - Endpoint DMA Grant Enable"]
pub type DMA_GNT_EN_R = crate::BitReader;
#[doc = "Field `DMA_GNT_EN` writer - Endpoint DMA Grant Enable"]
pub type DMA_GNT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF_OVER_EN` reader - Endpoint Buffer Overflow Enable"]
pub type BUF_OVER_EN_R = crate::BitReader;
#[doc = "Field `BUF_OVER_EN` writer - Endpoint Buffer Overflow Enable"]
pub type BUF_OVER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF_UNDER_EN` reader - Endpoint Buffer Underflow Enable"]
pub type BUF_UNDER_EN_R = crate::BitReader;
#[doc = "Field `BUF_UNDER_EN` writer - Endpoint Buffer Underflow Enable"]
pub type BUF_UNDER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_INT_EN` reader - Endpoint Error in Transaction Interrupt Enable"]
pub type ERR_INT_EN_R = crate::BitReader;
#[doc = "Field `ERR_INT_EN` writer - Endpoint Error in Transaction Interrupt Enable"]
pub type ERR_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_TERMIN_EN` reader - Endpoint DMA Terminated Enable"]
pub type DMA_TERMIN_EN_R = crate::BitReader;
#[doc = "Field `DMA_TERMIN_EN` writer - Endpoint DMA Terminated Enable"]
pub type DMA_TERMIN_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IN Endpoint Local Buffer Full Enable"]
    #[inline(always)]
    pub fn in_buf_full_en(&self) -> IN_BUF_FULL_EN_R {
        IN_BUF_FULL_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint DMA Grant Enable"]
    #[inline(always)]
    pub fn dma_gnt_en(&self) -> DMA_GNT_EN_R {
        DMA_GNT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Endpoint Buffer Overflow Enable"]
    #[inline(always)]
    pub fn buf_over_en(&self) -> BUF_OVER_EN_R {
        BUF_OVER_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Endpoint Buffer Underflow Enable"]
    #[inline(always)]
    pub fn buf_under_en(&self) -> BUF_UNDER_EN_R {
        BUF_UNDER_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Endpoint Error in Transaction Interrupt Enable"]
    #[inline(always)]
    pub fn err_int_en(&self) -> ERR_INT_EN_R {
        ERR_INT_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Endpoint DMA Terminated Enable"]
    #[inline(always)]
    pub fn dma_termin_en(&self) -> DMA_TERMIN_EN_R {
        DMA_TERMIN_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IN Endpoint Local Buffer Full Enable"]
    #[inline(always)]
    #[must_use]
    pub fn in_buf_full_en(&mut self) -> IN_BUF_FULL_EN_W<ARB_EP6_INT_EN_SPEC> {
        IN_BUF_FULL_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint DMA Grant Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_gnt_en(&mut self) -> DMA_GNT_EN_W<ARB_EP6_INT_EN_SPEC> {
        DMA_GNT_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Endpoint Buffer Overflow Enable"]
    #[inline(always)]
    #[must_use]
    pub fn buf_over_en(&mut self) -> BUF_OVER_EN_W<ARB_EP6_INT_EN_SPEC> {
        BUF_OVER_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Endpoint Buffer Underflow Enable"]
    #[inline(always)]
    #[must_use]
    pub fn buf_under_en(&mut self) -> BUF_UNDER_EN_W<ARB_EP6_INT_EN_SPEC> {
        BUF_UNDER_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Endpoint Error in Transaction Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn err_int_en(&mut self) -> ERR_INT_EN_W<ARB_EP6_INT_EN_SPEC> {
        ERR_INT_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Endpoint DMA Terminated Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_termin_en(&mut self) -> DMA_TERMIN_EN_W<ARB_EP6_INT_EN_SPEC> {
        DMA_TERMIN_EN_W::new(self, 5)
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
#[doc = "Endpoint Interrupt Enable Register *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_ep6_int_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_ep6_int_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARB_EP6_INT_EN_SPEC;
impl crate::RegisterSpec for ARB_EP6_INT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arb_ep6_int_en::R`](R) reader structure"]
impl crate::Readable for ARB_EP6_INT_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`arb_ep6_int_en::W`](W) writer structure"]
impl crate::Writable for ARB_EP6_INT_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ARB_EP6_INT_EN to value 0"]
impl crate::Resettable for ARB_EP6_INT_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
