#[doc = "Register `INTR_USBHOST_MASKED` reader"]
pub type R = crate::R<INTR_USBHOST_MASKED_SPEC>;
#[doc = "Field `SOFIRQED` reader - This bit indicates the interrupt by SOF flag. '0' : Doesn't request the interrupt by SOF '1' : Request the interrupt by SOF"]
pub type SOFIRQED_R = crate::BitReader;
#[doc = "Field `DIRQED` reader - This bit indicates the interrupt by DIRQ flag. '0' : Doesn't request the interrupt by DIRQ '1' : Request the interrupt by DIRQ"]
pub type DIRQED_R = crate::BitReader;
#[doc = "Field `CNNIRQED` reader - This bit indicates the interrupt by CNNIRQ flag. '0' : Doesn't request the interrupt by CNNIRQ '1' : Request the interrupt by CNNIRQ"]
pub type CNNIRQED_R = crate::BitReader;
#[doc = "Field `CMPIRQED` reader - This bit indicates the interrupt by CMPIRQ flag. '0' : Doesn't request the interrupt by CMPIRQ '1' : Request the interrupt by CMPIRQ"]
pub type CMPIRQED_R = crate::BitReader;
#[doc = "Field `URIRQED` reader - This bit indicates the interrupt by URIRQ flag. '0' : Doesn't request the interrupt by URIRQ '1' : Request the interrupt by URIRQ"]
pub type URIRQED_R = crate::BitReader;
#[doc = "Field `RWKIRQED` reader - This bit indicates the interrupt by RWKIRQ flag. '0' : Doesn't request the interrupt by RWKIRQ '1' : Request the interrupt by RWKIRQ"]
pub type RWKIRQED_R = crate::BitReader;
#[doc = "Field `RSVD_6` reader - N/A"]
pub type RSVD_6_R = crate::BitReader;
#[doc = "Field `TCANED` reader - This bit indicates the interrupt by TCAN flag. '0' : Doesn't request the interrupt by TCAN '1' : Request the interrupt by TCAN"]
pub type TCANED_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This bit indicates the interrupt by SOF flag. '0' : Doesn't request the interrupt by SOF '1' : Request the interrupt by SOF"]
    #[inline(always)]
    pub fn sofirqed(&self) -> SOFIRQED_R {
        SOFIRQED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit indicates the interrupt by DIRQ flag. '0' : Doesn't request the interrupt by DIRQ '1' : Request the interrupt by DIRQ"]
    #[inline(always)]
    pub fn dirqed(&self) -> DIRQED_R {
        DIRQED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit indicates the interrupt by CNNIRQ flag. '0' : Doesn't request the interrupt by CNNIRQ '1' : Request the interrupt by CNNIRQ"]
    #[inline(always)]
    pub fn cnnirqed(&self) -> CNNIRQED_R {
        CNNIRQED_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit indicates the interrupt by CMPIRQ flag. '0' : Doesn't request the interrupt by CMPIRQ '1' : Request the interrupt by CMPIRQ"]
    #[inline(always)]
    pub fn cmpirqed(&self) -> CMPIRQED_R {
        CMPIRQED_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit indicates the interrupt by URIRQ flag. '0' : Doesn't request the interrupt by URIRQ '1' : Request the interrupt by URIRQ"]
    #[inline(always)]
    pub fn urirqed(&self) -> URIRQED_R {
        URIRQED_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit indicates the interrupt by RWKIRQ flag. '0' : Doesn't request the interrupt by RWKIRQ '1' : Request the interrupt by RWKIRQ"]
    #[inline(always)]
    pub fn rwkirqed(&self) -> RWKIRQED_R {
        RWKIRQED_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn rsvd_6(&self) -> RSVD_6_R {
        RSVD_6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit indicates the interrupt by TCAN flag. '0' : Doesn't request the interrupt by TCAN '1' : Request the interrupt by TCAN"]
    #[inline(always)]
    pub fn tcaned(&self) -> TCANED_R {
        TCANED_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Interrupt USB Host Masked Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_usbhost_masked::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_USBHOST_MASKED_SPEC;
impl crate::RegisterSpec for INTR_USBHOST_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_usbhost_masked::R`](R) reader structure"]
impl crate::Readable for INTR_USBHOST_MASKED_SPEC {}
#[doc = "`reset()` method sets INTR_USBHOST_MASKED to value 0"]
impl crate::Resettable for INTR_USBHOST_MASKED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
