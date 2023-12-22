#[doc = "Register `SIE_EP_INT_SR` reader"]
pub type R = crate::R<SIE_EP_INT_SR_SPEC>;
#[doc = "Register `SIE_EP_INT_SR` writer"]
pub type W = crate::W<SIE_EP_INT_SR_SPEC>;
#[doc = "Field `EP1_INTR` reader - Interrupt status for EP1"]
pub type EP1_INTR_R = crate::BitReader;
#[doc = "Field `EP1_INTR` writer - Interrupt status for EP1"]
pub type EP1_INTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP2_INTR` reader - Interrupt status for EP2"]
pub type EP2_INTR_R = crate::BitReader;
#[doc = "Field `EP2_INTR` writer - Interrupt status for EP2"]
pub type EP2_INTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP3_INTR` reader - Interrupt status for EP3"]
pub type EP3_INTR_R = crate::BitReader;
#[doc = "Field `EP3_INTR` writer - Interrupt status for EP3"]
pub type EP3_INTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP4_INTR` reader - Interrupt status for EP4"]
pub type EP4_INTR_R = crate::BitReader;
#[doc = "Field `EP4_INTR` writer - Interrupt status for EP4"]
pub type EP4_INTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP5_INTR` reader - Interrupt status for EP5"]
pub type EP5_INTR_R = crate::BitReader;
#[doc = "Field `EP5_INTR` writer - Interrupt status for EP5"]
pub type EP5_INTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP6_INTR` reader - Interrupt status for EP6"]
pub type EP6_INTR_R = crate::BitReader;
#[doc = "Field `EP6_INTR` writer - Interrupt status for EP6"]
pub type EP6_INTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP7_INTR` reader - Interrupt status for EP7"]
pub type EP7_INTR_R = crate::BitReader;
#[doc = "Field `EP7_INTR` writer - Interrupt status for EP7"]
pub type EP7_INTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP8_INTR` reader - Interrupt status for EP8"]
pub type EP8_INTR_R = crate::BitReader;
#[doc = "Field `EP8_INTR` writer - Interrupt status for EP8"]
pub type EP8_INTR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt status for EP1"]
    #[inline(always)]
    pub fn ep1_intr(&self) -> EP1_INTR_R {
        EP1_INTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt status for EP2"]
    #[inline(always)]
    pub fn ep2_intr(&self) -> EP2_INTR_R {
        EP2_INTR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt status for EP3"]
    #[inline(always)]
    pub fn ep3_intr(&self) -> EP3_INTR_R {
        EP3_INTR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt status for EP4"]
    #[inline(always)]
    pub fn ep4_intr(&self) -> EP4_INTR_R {
        EP4_INTR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt status for EP5"]
    #[inline(always)]
    pub fn ep5_intr(&self) -> EP5_INTR_R {
        EP5_INTR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt status for EP6"]
    #[inline(always)]
    pub fn ep6_intr(&self) -> EP6_INTR_R {
        EP6_INTR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt status for EP7"]
    #[inline(always)]
    pub fn ep7_intr(&self) -> EP7_INTR_R {
        EP7_INTR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt status for EP8"]
    #[inline(always)]
    pub fn ep8_intr(&self) -> EP8_INTR_R {
        EP8_INTR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt status for EP1"]
    #[inline(always)]
    #[must_use]
    pub fn ep1_intr(&mut self) -> EP1_INTR_W<SIE_EP_INT_SR_SPEC> {
        EP1_INTR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt status for EP2"]
    #[inline(always)]
    #[must_use]
    pub fn ep2_intr(&mut self) -> EP2_INTR_W<SIE_EP_INT_SR_SPEC> {
        EP2_INTR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt status for EP3"]
    #[inline(always)]
    #[must_use]
    pub fn ep3_intr(&mut self) -> EP3_INTR_W<SIE_EP_INT_SR_SPEC> {
        EP3_INTR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt status for EP4"]
    #[inline(always)]
    #[must_use]
    pub fn ep4_intr(&mut self) -> EP4_INTR_W<SIE_EP_INT_SR_SPEC> {
        EP4_INTR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt status for EP5"]
    #[inline(always)]
    #[must_use]
    pub fn ep5_intr(&mut self) -> EP5_INTR_W<SIE_EP_INT_SR_SPEC> {
        EP5_INTR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt status for EP6"]
    #[inline(always)]
    #[must_use]
    pub fn ep6_intr(&mut self) -> EP6_INTR_W<SIE_EP_INT_SR_SPEC> {
        EP6_INTR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt status for EP7"]
    #[inline(always)]
    #[must_use]
    pub fn ep7_intr(&mut self) -> EP7_INTR_W<SIE_EP_INT_SR_SPEC> {
        EP7_INTR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Interrupt status for EP8"]
    #[inline(always)]
    #[must_use]
    pub fn ep8_intr(&mut self) -> EP8_INTR_W<SIE_EP_INT_SR_SPEC> {
        EP8_INTR_W::new(self, 7)
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
#[doc = "USB SIE Data Endpoint Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sie_ep_int_sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sie_ep_int_sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SIE_EP_INT_SR_SPEC;
impl crate::RegisterSpec for SIE_EP_INT_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sie_ep_int_sr::R`](R) reader structure"]
impl crate::Readable for SIE_EP_INT_SR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sie_ep_int_sr::W`](W) writer structure"]
impl crate::Writable for SIE_EP_INT_SR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIE_EP_INT_SR to value 0"]
impl crate::Resettable for SIE_EP_INT_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
