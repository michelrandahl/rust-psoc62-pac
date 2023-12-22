#[doc = "Register `INTR_SIE` reader"]
pub type R = crate::R<INTR_SIE_SPEC>;
#[doc = "Register `INTR_SIE` writer"]
pub type W = crate::W<INTR_SIE_SPEC>;
#[doc = "Field `SOF_INTR` reader - Interrupt status for USB SOF"]
pub type SOF_INTR_R = crate::BitReader;
#[doc = "Field `SOF_INTR` writer - Interrupt status for USB SOF"]
pub type SOF_INTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUS_RESET_INTR` reader - Interrupt status for BUS RESET"]
pub type BUS_RESET_INTR_R = crate::BitReader;
#[doc = "Field `BUS_RESET_INTR` writer - Interrupt status for BUS RESET"]
pub type BUS_RESET_INTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP0_INTR` reader - Interrupt status for EP0"]
pub type EP0_INTR_R = crate::BitReader;
#[doc = "Field `EP0_INTR` writer - Interrupt status for EP0"]
pub type EP0_INTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPM_INTR` reader - Interrupt status for LPM (Link Power Management, L1 entry)"]
pub type LPM_INTR_R = crate::BitReader;
#[doc = "Field `LPM_INTR` writer - Interrupt status for LPM (Link Power Management, L1 entry)"]
pub type LPM_INTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESUME_INTR` reader - Interrupt status for Resume"]
pub type RESUME_INTR_R = crate::BitReader;
#[doc = "Field `RESUME_INTR` writer - Interrupt status for Resume"]
pub type RESUME_INTR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt status for USB SOF"]
    #[inline(always)]
    pub fn sof_intr(&self) -> SOF_INTR_R {
        SOF_INTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt status for BUS RESET"]
    #[inline(always)]
    pub fn bus_reset_intr(&self) -> BUS_RESET_INTR_R {
        BUS_RESET_INTR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt status for EP0"]
    #[inline(always)]
    pub fn ep0_intr(&self) -> EP0_INTR_R {
        EP0_INTR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt status for LPM (Link Power Management, L1 entry)"]
    #[inline(always)]
    pub fn lpm_intr(&self) -> LPM_INTR_R {
        LPM_INTR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt status for Resume"]
    #[inline(always)]
    pub fn resume_intr(&self) -> RESUME_INTR_R {
        RESUME_INTR_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt status for USB SOF"]
    #[inline(always)]
    #[must_use]
    pub fn sof_intr(&mut self) -> SOF_INTR_W<INTR_SIE_SPEC> {
        SOF_INTR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt status for BUS RESET"]
    #[inline(always)]
    #[must_use]
    pub fn bus_reset_intr(&mut self) -> BUS_RESET_INTR_W<INTR_SIE_SPEC> {
        BUS_RESET_INTR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt status for EP0"]
    #[inline(always)]
    #[must_use]
    pub fn ep0_intr(&mut self) -> EP0_INTR_W<INTR_SIE_SPEC> {
        EP0_INTR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt status for LPM (Link Power Management, L1 entry)"]
    #[inline(always)]
    #[must_use]
    pub fn lpm_intr(&mut self) -> LPM_INTR_W<INTR_SIE_SPEC> {
        LPM_INTR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt status for Resume"]
    #[inline(always)]
    #[must_use]
    pub fn resume_intr(&mut self) -> RESUME_INTR_W<INTR_SIE_SPEC> {
        RESUME_INTR_W::new(self, 4)
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
#[doc = "USB SOF, BUS RESET and EP0 Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_sie::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_sie::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_SIE_SPEC;
impl crate::RegisterSpec for INTR_SIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_sie::R`](R) reader structure"]
impl crate::Readable for INTR_SIE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr_sie::W`](W) writer structure"]
impl crate::Writable for INTR_SIE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_SIE to value 0"]
impl crate::Resettable for INTR_SIE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
