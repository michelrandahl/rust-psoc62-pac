#[doc = "Register `DMA_THRES` reader"]
pub type R = crate::R<DMA_THRES_SPEC>;
#[doc = "Register `DMA_THRES` writer"]
pub type W = crate::W<DMA_THRES_SPEC>;
#[doc = "Field `DMA_THS` reader - DMA Threshold count"]
pub type DMA_THS_R = crate::FieldReader;
#[doc = "Field `DMA_THS` writer - DMA Threshold count"]
pub type DMA_THS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DMA Threshold count"]
    #[inline(always)]
    pub fn dma_ths(&self) -> DMA_THS_R {
        DMA_THS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DMA Threshold count"]
    #[inline(always)]
    #[must_use]
    pub fn dma_ths(&mut self) -> DMA_THS_W<DMA_THRES_SPEC> {
        DMA_THS_W::new(self, 0)
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
#[doc = "DMA Burst / Threshold Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_thres::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_thres::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_THRES_SPEC;
impl crate::RegisterSpec for DMA_THRES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_thres::R`](R) reader structure"]
impl crate::Readable for DMA_THRES_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_thres::W`](W) writer structure"]
impl crate::Writable for DMA_THRES_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_THRES to value 0"]
impl crate::Resettable for DMA_THRES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
