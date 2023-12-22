#[doc = "Register `HOST_EP2_BLK` reader"]
pub type R = crate::R<HOST_EP2_BLK_SPEC>;
#[doc = "Register `HOST_EP2_BLK` writer"]
pub type W = crate::W<HOST_EP2_BLK_SPEC>;
#[doc = "Field `BLK_NUM` reader - Set the total byte number for DMA transfer. If HOST_EP2_RW1_DR or HOST_EP2_RW2_DR is written, the block number counter is decrement when DMAE='1'. - Set this bits before DMA transfer is enabled (HOST_DMA_ENBL.DM_DP2DRQE='1')"]
pub type BLK_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `BLK_NUM` writer - Set the total byte number for DMA transfer. If HOST_EP2_RW1_DR or HOST_EP2_RW2_DR is written, the block number counter is decrement when DMAE='1'. - Set this bits before DMA transfer is enabled (HOST_DMA_ENBL.DM_DP2DRQE='1')"]
pub type BLK_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 16:31 - Set the total byte number for DMA transfer. If HOST_EP2_RW1_DR or HOST_EP2_RW2_DR is written, the block number counter is decrement when DMAE='1'. - Set this bits before DMA transfer is enabled (HOST_DMA_ENBL.DM_DP2DRQE='1')"]
    #[inline(always)]
    pub fn blk_num(&self) -> BLK_NUM_R {
        BLK_NUM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Set the total byte number for DMA transfer. If HOST_EP2_RW1_DR or HOST_EP2_RW2_DR is written, the block number counter is decrement when DMAE='1'. - Set this bits before DMA transfer is enabled (HOST_DMA_ENBL.DM_DP2DRQE='1')"]
    #[inline(always)]
    #[must_use]
    pub fn blk_num(&mut self) -> BLK_NUM_W<HOST_EP2_BLK_SPEC> {
        BLK_NUM_W::new(self, 16)
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
#[doc = "Host Endpoint 2 Block Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_ep2_blk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_ep2_blk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_EP2_BLK_SPEC;
impl crate::RegisterSpec for HOST_EP2_BLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_ep2_blk::R`](R) reader structure"]
impl crate::Readable for HOST_EP2_BLK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_ep2_blk::W`](W) writer structure"]
impl crate::Writable for HOST_EP2_BLK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOST_EP2_BLK to value 0"]
impl crate::Resettable for HOST_EP2_BLK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
