#[doc = "Register `HOST_DMA_ENBL` reader"]
pub type R = crate::R<HOST_DMA_ENBL_SPEC>;
#[doc = "Register `HOST_DMA_ENBL` writer"]
pub type W = crate::W<HOST_DMA_ENBL_SPEC>;
#[doc = "Field `DM_EP1DRQE` reader - This bit enables DMA Request by EP1DRQ. '0' : Disable '1' : Enable"]
pub type DM_EP1DRQE_R = crate::BitReader;
#[doc = "Field `DM_EP1DRQE` writer - This bit enables DMA Request by EP1DRQ. '0' : Disable '1' : Enable"]
pub type DM_EP1DRQE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DM_EP2DRQE` reader - This bit enables DMA Request by EP2DRQ. '0' : Disable '1' : Enable"]
pub type DM_EP2DRQE_R = crate::BitReader;
#[doc = "Field `DM_EP2DRQE` writer - This bit enables DMA Request by EP2DRQ. '0' : Disable '1' : Enable"]
pub type DM_EP2DRQE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - This bit enables DMA Request by EP1DRQ. '0' : Disable '1' : Enable"]
    #[inline(always)]
    pub fn dm_ep1drqe(&self) -> DM_EP1DRQE_R {
        DM_EP1DRQE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit enables DMA Request by EP2DRQ. '0' : Disable '1' : Enable"]
    #[inline(always)]
    pub fn dm_ep2drqe(&self) -> DM_EP2DRQE_R {
        DM_EP2DRQE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - This bit enables DMA Request by EP1DRQ. '0' : Disable '1' : Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dm_ep1drqe(&mut self) -> DM_EP1DRQE_W<HOST_DMA_ENBL_SPEC> {
        DM_EP1DRQE_W::new(self, 2)
    }
    #[doc = "Bit 3 - This bit enables DMA Request by EP2DRQ. '0' : Disable '1' : Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dm_ep2drqe(&mut self) -> DM_EP2DRQE_W<HOST_DMA_ENBL_SPEC> {
        DM_EP2DRQE_W::new(self, 3)
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
#[doc = "Host DMA Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_dma_enbl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_dma_enbl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_DMA_ENBL_SPEC;
impl crate::RegisterSpec for HOST_DMA_ENBL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_dma_enbl::R`](R) reader structure"]
impl crate::Readable for HOST_DMA_ENBL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_dma_enbl::W`](W) writer structure"]
impl crate::Writable for HOST_DMA_ENBL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOST_DMA_ENBL to value 0"]
impl crate::Resettable for HOST_DMA_ENBL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
