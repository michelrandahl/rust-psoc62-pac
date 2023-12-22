#[doc = "Register `INTR_SIE_MASK` reader"]
pub type R = crate::R<INTR_SIE_MASK_SPEC>;
#[doc = "Register `INTR_SIE_MASK` writer"]
pub type W = crate::W<INTR_SIE_MASK_SPEC>;
#[doc = "Field `SOF_INTR_MASK` reader - Set to 1 to enable interrupt corresponding to interrupt request register"]
pub type SOF_INTR_MASK_R = crate::BitReader;
#[doc = "Field `SOF_INTR_MASK` writer - Set to 1 to enable interrupt corresponding to interrupt request register"]
pub type SOF_INTR_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUS_RESET_INTR_MASK` reader - Set to 1 to enable interrupt corresponding to interrupt request register"]
pub type BUS_RESET_INTR_MASK_R = crate::BitReader;
#[doc = "Field `BUS_RESET_INTR_MASK` writer - Set to 1 to enable interrupt corresponding to interrupt request register"]
pub type BUS_RESET_INTR_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP0_INTR_MASK` reader - Set to 1 to enable interrupt corresponding to interrupt request register"]
pub type EP0_INTR_MASK_R = crate::BitReader;
#[doc = "Field `EP0_INTR_MASK` writer - Set to 1 to enable interrupt corresponding to interrupt request register"]
pub type EP0_INTR_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPM_INTR_MASK` reader - Set to 1 to enable interrupt corresponding to interrupt request register"]
pub type LPM_INTR_MASK_R = crate::BitReader;
#[doc = "Field `LPM_INTR_MASK` writer - Set to 1 to enable interrupt corresponding to interrupt request register"]
pub type LPM_INTR_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESUME_INTR_MASK` reader - Set to 1 to enable interrupt corresponding to interrupt request register"]
pub type RESUME_INTR_MASK_R = crate::BitReader;
#[doc = "Field `RESUME_INTR_MASK` writer - Set to 1 to enable interrupt corresponding to interrupt request register"]
pub type RESUME_INTR_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    pub fn sof_intr_mask(&self) -> SOF_INTR_MASK_R {
        SOF_INTR_MASK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    pub fn bus_reset_intr_mask(&self) -> BUS_RESET_INTR_MASK_R {
        BUS_RESET_INTR_MASK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    pub fn ep0_intr_mask(&self) -> EP0_INTR_MASK_R {
        EP0_INTR_MASK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    pub fn lpm_intr_mask(&self) -> LPM_INTR_MASK_R {
        LPM_INTR_MASK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    pub fn resume_intr_mask(&self) -> RESUME_INTR_MASK_R {
        RESUME_INTR_MASK_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    #[must_use]
    pub fn sof_intr_mask(&mut self) -> SOF_INTR_MASK_W<INTR_SIE_MASK_SPEC> {
        SOF_INTR_MASK_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    #[must_use]
    pub fn bus_reset_intr_mask(&mut self) -> BUS_RESET_INTR_MASK_W<INTR_SIE_MASK_SPEC> {
        BUS_RESET_INTR_MASK_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    #[must_use]
    pub fn ep0_intr_mask(&mut self) -> EP0_INTR_MASK_W<INTR_SIE_MASK_SPEC> {
        EP0_INTR_MASK_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    #[must_use]
    pub fn lpm_intr_mask(&mut self) -> LPM_INTR_MASK_W<INTR_SIE_MASK_SPEC> {
        LPM_INTR_MASK_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    #[must_use]
    pub fn resume_intr_mask(&mut self) -> RESUME_INTR_MASK_W<INTR_SIE_MASK_SPEC> {
        RESUME_INTR_MASK_W::new(self, 4)
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
#[doc = "USB SOF, BUS RESET and EP0 Interrupt Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_sie_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_sie_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_SIE_MASK_SPEC;
impl crate::RegisterSpec for INTR_SIE_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_sie_mask::R`](R) reader structure"]
impl crate::Readable for INTR_SIE_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr_sie_mask::W`](W) writer structure"]
impl crate::Writable for INTR_SIE_MASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_SIE_MASK to value 0"]
impl crate::Resettable for INTR_SIE_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
