#[doc = "Register `INTR_HOST_EP_MASK` reader"]
pub type R = crate::R<INTR_HOST_EP_MASK_SPEC>;
#[doc = "Register `INTR_HOST_EP_MASK` writer"]
pub type W = crate::W<INTR_HOST_EP_MASK_SPEC>;
#[doc = "Field `EP1DRQM` reader - This bit masks the interrupt by EP1DRQ flag. '0' : Disables '1' : Enables"]
pub type EP1DRQM_R = crate::BitReader;
#[doc = "Field `EP1DRQM` writer - This bit masks the interrupt by EP1DRQ flag. '0' : Disables '1' : Enables"]
pub type EP1DRQM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP1SPKM` reader - This bit masks the interrupt by EP1SPK flag. '0' : Disables '1' : Enables"]
pub type EP1SPKM_R = crate::BitReader;
#[doc = "Field `EP1SPKM` writer - This bit masks the interrupt by EP1SPK flag. '0' : Disables '1' : Enables"]
pub type EP1SPKM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP2DRQM` reader - This bit masks the interrupt by EP2DRQ flag. '0' : Disables '1' : Enables"]
pub type EP2DRQM_R = crate::BitReader;
#[doc = "Field `EP2DRQM` writer - This bit masks the interrupt by EP2DRQ flag. '0' : Disables '1' : Enables"]
pub type EP2DRQM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP2SPKM` reader - This bit masks the interrupt by EP2SPK flag. '0' : Disables '1' : Enables"]
pub type EP2SPKM_R = crate::BitReader;
#[doc = "Field `EP2SPKM` writer - This bit masks the interrupt by EP2SPK flag. '0' : Disables '1' : Enables"]
pub type EP2SPKM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - This bit masks the interrupt by EP1DRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn ep1drqm(&self) -> EP1DRQM_R {
        EP1DRQM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit masks the interrupt by EP1SPK flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn ep1spkm(&self) -> EP1SPKM_R {
        EP1SPKM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit masks the interrupt by EP2DRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn ep2drqm(&self) -> EP2DRQM_R {
        EP2DRQM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit masks the interrupt by EP2SPK flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn ep2spkm(&self) -> EP2SPKM_R {
        EP2SPKM_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - This bit masks the interrupt by EP1DRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    #[must_use]
    pub fn ep1drqm(&mut self) -> EP1DRQM_W<INTR_HOST_EP_MASK_SPEC> {
        EP1DRQM_W::new(self, 2)
    }
    #[doc = "Bit 3 - This bit masks the interrupt by EP1SPK flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    #[must_use]
    pub fn ep1spkm(&mut self) -> EP1SPKM_W<INTR_HOST_EP_MASK_SPEC> {
        EP1SPKM_W::new(self, 3)
    }
    #[doc = "Bit 4 - This bit masks the interrupt by EP2DRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    #[must_use]
    pub fn ep2drqm(&mut self) -> EP2DRQM_W<INTR_HOST_EP_MASK_SPEC> {
        EP2DRQM_W::new(self, 4)
    }
    #[doc = "Bit 5 - This bit masks the interrupt by EP2SPK flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    #[must_use]
    pub fn ep2spkm(&mut self) -> EP2SPKM_W<INTR_HOST_EP_MASK_SPEC> {
        EP2SPKM_W::new(self, 5)
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
#[doc = "Interrupt USB Host Endpoint Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_host_ep_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_host_ep_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_HOST_EP_MASK_SPEC;
impl crate::RegisterSpec for INTR_HOST_EP_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_host_ep_mask::R`](R) reader structure"]
impl crate::Readable for INTR_HOST_EP_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr_host_ep_mask::W`](W) writer structure"]
impl crate::Writable for INTR_HOST_EP_MASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_HOST_EP_MASK to value 0"]
impl crate::Resettable for INTR_HOST_EP_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
