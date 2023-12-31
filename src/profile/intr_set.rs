#[doc = "Register `INTR_SET` reader"]
pub type R = crate::R<INTR_SET_SPEC>;
#[doc = "Register `INTR_SET` writer"]
pub type W = crate::W<INTR_SET_SPEC>;
#[doc = "Field `CNT_OVFLW` reader - SW writes a '1' to a bit of this field to set the corresponding bit in the INTR register."]
pub type CNT_OVFLW_R = crate::FieldReader<u32>;
#[doc = "Field `CNT_OVFLW` writer - SW writes a '1' to a bit of this field to set the corresponding bit in the INTR register."]
pub type CNT_OVFLW_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SW writes a '1' to a bit of this field to set the corresponding bit in the INTR register."]
    #[inline(always)]
    pub fn cnt_ovflw(&self) -> CNT_OVFLW_R {
        CNT_OVFLW_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SW writes a '1' to a bit of this field to set the corresponding bit in the INTR register."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_ovflw(&mut self) -> CNT_OVFLW_W<INTR_SET_SPEC> {
        CNT_OVFLW_W::new(self, 0)
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
#[doc = "Profile interrupt set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_SET_SPEC;
impl crate::RegisterSpec for INTR_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_set::R`](R) reader structure"]
impl crate::Readable for INTR_SET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr_set::W`](W) writer structure"]
impl crate::Writable for INTR_SET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_SET to value 0"]
impl crate::Resettable for INTR_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
