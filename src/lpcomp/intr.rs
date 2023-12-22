#[doc = "Register `INTR` reader"]
pub type R = crate::R<INTR_SPEC>;
#[doc = "Register `INTR` writer"]
pub type W = crate::W<INTR_SPEC>;
#[doc = "Field `COMP0` reader - Comparator 0 Interrupt: hardware sets this interrupt when comparator 0 triggers. Write with '1' to clear bit."]
pub type COMP0_R = crate::BitReader;
#[doc = "Field `COMP0` writer - Comparator 0 Interrupt: hardware sets this interrupt when comparator 0 triggers. Write with '1' to clear bit."]
pub type COMP0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1` reader - Comparator 1 Interrupt: hardware sets this interrupt when comparator 1 triggers. Write with '1' to clear bit."]
pub type COMP1_R = crate::BitReader;
#[doc = "Field `COMP1` writer - Comparator 1 Interrupt: hardware sets this interrupt when comparator 1 triggers. Write with '1' to clear bit."]
pub type COMP1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comparator 0 Interrupt: hardware sets this interrupt when comparator 0 triggers. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn comp0(&self) -> COMP0_R {
        COMP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 Interrupt: hardware sets this interrupt when comparator 1 triggers. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn comp1(&self) -> COMP1_R {
        COMP1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 0 Interrupt: hardware sets this interrupt when comparator 0 triggers. Write with '1' to clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn comp0(&mut self) -> COMP0_W<INTR_SPEC> {
        COMP0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Comparator 1 Interrupt: hardware sets this interrupt when comparator 1 triggers. Write with '1' to clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn comp1(&mut self) -> COMP1_W<INTR_SPEC> {
        COMP1_W::new(self, 1)
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
#[doc = "LPCOMP Interrupt request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_SPEC;
impl crate::RegisterSpec for INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr::R`](R) reader structure"]
impl crate::Readable for INTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr::W`](W) writer structure"]
impl crate::Writable for INTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for INTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
