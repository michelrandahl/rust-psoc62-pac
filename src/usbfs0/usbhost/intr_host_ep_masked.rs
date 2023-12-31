#[doc = "Register `INTR_HOST_EP_MASKED` reader"]
pub type R = crate::R<INTR_HOST_EP_MASKED_SPEC>;
#[doc = "Field `EP1DRQED` reader - This bit indicates the interrupt by EP1DRQ flag. '0' : Doesn't request the interrupt by EP1DRQ '1' : Request the interrupt by EP1DRQ"]
pub type EP1DRQED_R = crate::BitReader;
#[doc = "Field `EP1SPKED` reader - This bit indicates the interrupt by EP1SPK flag. '0' : Doesn't request the interrupt by EP1SPK '1' : Request the interrupt by EP1SPK"]
pub type EP1SPKED_R = crate::BitReader;
#[doc = "Field `EP2DRQED` reader - This bit indicates the interrupt by EP2DRQ flag. '0' : Doesn't request the interrupt by EP2DRQ '1' : Request the interrupt by EP2DRQ"]
pub type EP2DRQED_R = crate::BitReader;
#[doc = "Field `EP2SPKED` reader - This bit indicates the interrupt by EP2SPK flag. '0' : Doesn't request the interrupt by EP2SPK '1' : Request the interrupt by EP2SPK"]
pub type EP2SPKED_R = crate::BitReader;
impl R {
    #[doc = "Bit 2 - This bit indicates the interrupt by EP1DRQ flag. '0' : Doesn't request the interrupt by EP1DRQ '1' : Request the interrupt by EP1DRQ"]
    #[inline(always)]
    pub fn ep1drqed(&self) -> EP1DRQED_R {
        EP1DRQED_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit indicates the interrupt by EP1SPK flag. '0' : Doesn't request the interrupt by EP1SPK '1' : Request the interrupt by EP1SPK"]
    #[inline(always)]
    pub fn ep1spked(&self) -> EP1SPKED_R {
        EP1SPKED_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit indicates the interrupt by EP2DRQ flag. '0' : Doesn't request the interrupt by EP2DRQ '1' : Request the interrupt by EP2DRQ"]
    #[inline(always)]
    pub fn ep2drqed(&self) -> EP2DRQED_R {
        EP2DRQED_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit indicates the interrupt by EP2SPK flag. '0' : Doesn't request the interrupt by EP2SPK '1' : Request the interrupt by EP2SPK"]
    #[inline(always)]
    pub fn ep2spked(&self) -> EP2SPKED_R {
        EP2SPKED_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Interrupt USB Host Endpoint Masked Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_host_ep_masked::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_HOST_EP_MASKED_SPEC;
impl crate::RegisterSpec for INTR_HOST_EP_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_host_ep_masked::R`](R) reader structure"]
impl crate::Readable for INTR_HOST_EP_MASKED_SPEC {}
#[doc = "`reset()` method sets INTR_HOST_EP_MASKED to value 0"]
impl crate::Resettable for INTR_HOST_EP_MASKED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
