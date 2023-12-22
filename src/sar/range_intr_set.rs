#[doc = "Register `RANGE_INTR_SET` reader"]
pub type R = crate::R<RANGE_INTR_SET_SPEC>;
#[doc = "Register `RANGE_INTR_SET` writer"]
pub type W = crate::W<RANGE_INTR_SET_SPEC>;
#[doc = "Field `RANGE_SET` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type RANGE_SET_R = crate::FieldReader<u16>;
#[doc = "Field `RANGE_SET` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type RANGE_SET_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn range_set(&self) -> RANGE_SET_R {
        RANGE_SET_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn range_set(&mut self) -> RANGE_SET_W<RANGE_INTR_SET_SPEC> {
        RANGE_SET_W::new(self, 0)
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
#[doc = "Range detect interrupt set request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`range_intr_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`range_intr_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RANGE_INTR_SET_SPEC;
impl crate::RegisterSpec for RANGE_INTR_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`range_intr_set::R`](R) reader structure"]
impl crate::Readable for RANGE_INTR_SET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`range_intr_set::W`](W) writer structure"]
impl crate::Writable for RANGE_INTR_SET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RANGE_INTR_SET to value 0"]
impl crate::Resettable for RANGE_INTR_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
