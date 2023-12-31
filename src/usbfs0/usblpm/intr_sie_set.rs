#[doc = "Register `INTR_SIE_SET` reader"]
pub type R = crate::R<INTR_SIE_SET_SPEC>;
#[doc = "Register `INTR_SIE_SET` writer"]
pub type W = crate::W<INTR_SIE_SET_SPEC>;
#[doc = "Field `SOF_INTR_SET` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type SOF_INTR_SET_R = crate::BitReader;
#[doc = "Field `SOF_INTR_SET` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type SOF_INTR_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUS_RESET_INTR_SET` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type BUS_RESET_INTR_SET_R = crate::BitReader;
#[doc = "Field `BUS_RESET_INTR_SET` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type BUS_RESET_INTR_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP0_INTR_SET` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type EP0_INTR_SET_R = crate::BitReader;
#[doc = "Field `EP0_INTR_SET` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type EP0_INTR_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPM_INTR_SET` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type LPM_INTR_SET_R = crate::BitReader;
#[doc = "Field `LPM_INTR_SET` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type LPM_INTR_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESUME_INTR_SET` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type RESUME_INTR_SET_R = crate::BitReader;
#[doc = "Field `RESUME_INTR_SET` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type RESUME_INTR_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn sof_intr_set(&self) -> SOF_INTR_SET_R {
        SOF_INTR_SET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn bus_reset_intr_set(&self) -> BUS_RESET_INTR_SET_R {
        BUS_RESET_INTR_SET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn ep0_intr_set(&self) -> EP0_INTR_SET_R {
        EP0_INTR_SET_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn lpm_intr_set(&self) -> LPM_INTR_SET_R {
        LPM_INTR_SET_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn resume_intr_set(&self) -> RESUME_INTR_SET_R {
        RESUME_INTR_SET_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn sof_intr_set(&mut self) -> SOF_INTR_SET_W<INTR_SIE_SET_SPEC> {
        SOF_INTR_SET_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn bus_reset_intr_set(&mut self) -> BUS_RESET_INTR_SET_W<INTR_SIE_SET_SPEC> {
        BUS_RESET_INTR_SET_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn ep0_intr_set(&mut self) -> EP0_INTR_SET_W<INTR_SIE_SET_SPEC> {
        EP0_INTR_SET_W::new(self, 2)
    }
    #[doc = "Bit 3 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn lpm_intr_set(&mut self) -> LPM_INTR_SET_W<INTR_SIE_SET_SPEC> {
        LPM_INTR_SET_W::new(self, 3)
    }
    #[doc = "Bit 4 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn resume_intr_set(&mut self) -> RESUME_INTR_SET_W<INTR_SIE_SET_SPEC> {
        RESUME_INTR_SET_W::new(self, 4)
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
#[doc = "USB SOF, BUS RESET and EP0 Interrupt Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_sie_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_sie_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_SIE_SET_SPEC;
impl crate::RegisterSpec for INTR_SIE_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_sie_set::R`](R) reader structure"]
impl crate::Readable for INTR_SIE_SET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr_sie_set::W`](W) writer structure"]
impl crate::Writable for INTR_SIE_SET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_SIE_SET to value 0"]
impl crate::Resettable for INTR_SIE_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
