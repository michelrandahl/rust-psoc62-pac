#[doc = "Register `INTR_USBHOST_MASK` reader"]
pub type R = crate::R<INTR_USBHOST_MASK_SPEC>;
#[doc = "Register `INTR_USBHOST_MASK` writer"]
pub type W = crate::W<INTR_USBHOST_MASK_SPEC>;
#[doc = "Field `SOFIRQM` reader - This bit masks the interrupt by SOF flag. '0' : Disables '1' : Enables"]
pub type SOFIRQM_R = crate::BitReader;
#[doc = "Field `SOFIRQM` writer - This bit masks the interrupt by SOF flag. '0' : Disables '1' : Enables"]
pub type SOFIRQM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRQM` reader - This bit masks the interrupt by DIRQ flag. '0' : Disables '1' : Enables"]
pub type DIRQM_R = crate::BitReader;
#[doc = "Field `DIRQM` writer - This bit masks the interrupt by DIRQ flag. '0' : Disables '1' : Enables"]
pub type DIRQM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNNIRQM` reader - This bit masks the interrupt by CNNIRQ flag. '0' : Disables '1' : Enables"]
pub type CNNIRQM_R = crate::BitReader;
#[doc = "Field `CNNIRQM` writer - This bit masks the interrupt by CNNIRQ flag. '0' : Disables '1' : Enables"]
pub type CNNIRQM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPIRQM` reader - This bit masks the interrupt by CMPIRQ flag. '0' : Disables '1' : Enables"]
pub type CMPIRQM_R = crate::BitReader;
#[doc = "Field `CMPIRQM` writer - This bit masks the interrupt by CMPIRQ flag. '0' : Disables '1' : Enables"]
pub type CMPIRQM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `URIRQM` reader - This bit masks the interrupt by URIRQ flag. '0' : Disables '1' : Enables"]
pub type URIRQM_R = crate::BitReader;
#[doc = "Field `URIRQM` writer - This bit masks the interrupt by URIRQ flag. '0' : Disables '1' : Enables"]
pub type URIRQM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWKIRQM` reader - This bit masks the interrupt by RWKIRQ flag. '0' : Disables '1' : Enables"]
pub type RWKIRQM_R = crate::BitReader;
#[doc = "Field `RWKIRQM` writer - This bit masks the interrupt by RWKIRQ flag. '0' : Disables '1' : Enables"]
pub type RWKIRQM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD_6` reader - N/A"]
pub type RSVD_6_R = crate::BitReader;
#[doc = "Field `RSVD_6` writer - N/A"]
pub type RSVD_6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCANM` reader - This bit masks the interrupt by TCAN flag. '0' : Disables '1' : Enables"]
pub type TCANM_R = crate::BitReader;
#[doc = "Field `TCANM` writer - This bit masks the interrupt by TCAN flag. '0' : Disables '1' : Enables"]
pub type TCANM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit masks the interrupt by SOF flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn sofirqm(&self) -> SOFIRQM_R {
        SOFIRQM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit masks the interrupt by DIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn dirqm(&self) -> DIRQM_R {
        DIRQM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit masks the interrupt by CNNIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn cnnirqm(&self) -> CNNIRQM_R {
        CNNIRQM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit masks the interrupt by CMPIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn cmpirqm(&self) -> CMPIRQM_R {
        CMPIRQM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit masks the interrupt by URIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn urirqm(&self) -> URIRQM_R {
        URIRQM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit masks the interrupt by RWKIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn rwkirqm(&self) -> RWKIRQM_R {
        RWKIRQM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn rsvd_6(&self) -> RSVD_6_R {
        RSVD_6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit masks the interrupt by TCAN flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn tcanm(&self) -> TCANM_R {
        TCANM_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit masks the interrupt by SOF flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    #[must_use]
    pub fn sofirqm(&mut self) -> SOFIRQM_W<INTR_USBHOST_MASK_SPEC> {
        SOFIRQM_W::new(self, 0)
    }
    #[doc = "Bit 1 - This bit masks the interrupt by DIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    #[must_use]
    pub fn dirqm(&mut self) -> DIRQM_W<INTR_USBHOST_MASK_SPEC> {
        DIRQM_W::new(self, 1)
    }
    #[doc = "Bit 2 - This bit masks the interrupt by CNNIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    #[must_use]
    pub fn cnnirqm(&mut self) -> CNNIRQM_W<INTR_USBHOST_MASK_SPEC> {
        CNNIRQM_W::new(self, 2)
    }
    #[doc = "Bit 3 - This bit masks the interrupt by CMPIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    #[must_use]
    pub fn cmpirqm(&mut self) -> CMPIRQM_W<INTR_USBHOST_MASK_SPEC> {
        CMPIRQM_W::new(self, 3)
    }
    #[doc = "Bit 4 - This bit masks the interrupt by URIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    #[must_use]
    pub fn urirqm(&mut self) -> URIRQM_W<INTR_USBHOST_MASK_SPEC> {
        URIRQM_W::new(self, 4)
    }
    #[doc = "Bit 5 - This bit masks the interrupt by RWKIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    #[must_use]
    pub fn rwkirqm(&mut self) -> RWKIRQM_W<INTR_USBHOST_MASK_SPEC> {
        RWKIRQM_W::new(self, 5)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn rsvd_6(&mut self) -> RSVD_6_W<INTR_USBHOST_MASK_SPEC> {
        RSVD_6_W::new(self, 6)
    }
    #[doc = "Bit 7 - This bit masks the interrupt by TCAN flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    #[must_use]
    pub fn tcanm(&mut self) -> TCANM_W<INTR_USBHOST_MASK_SPEC> {
        TCANM_W::new(self, 7)
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
#[doc = "Interrupt USB Host Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_usbhost_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_usbhost_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_USBHOST_MASK_SPEC;
impl crate::RegisterSpec for INTR_USBHOST_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_usbhost_mask::R`](R) reader structure"]
impl crate::Readable for INTR_USBHOST_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr_usbhost_mask::W`](W) writer structure"]
impl crate::Writable for INTR_USBHOST_MASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_USBHOST_MASK to value 0"]
impl crate::Resettable for INTR_USBHOST_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
