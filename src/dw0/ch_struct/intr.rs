#[doc = "Register `INTR` reader"]
pub type R = crate::R<INTR_SPEC>;
#[doc = "Register `INTR` writer"]
pub type W = crate::W<INTR_SPEC>;
#[doc = "Field `CH` reader - Set to '1', when event (as specified by CH_STATUS.INTR_CAUSE) is detected. Write INTR.CH field with '1', to clear bit. Write INTR_SET.CH field with '1', to set bit."]
pub type CH_R = crate::BitReader;
#[doc = "Field `CH` writer - Set to '1', when event (as specified by CH_STATUS.INTR_CAUSE) is detected. Write INTR.CH field with '1', to clear bit. Write INTR_SET.CH field with '1', to set bit."]
pub type CH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set to '1', when event (as specified by CH_STATUS.INTR_CAUSE) is detected. Write INTR.CH field with '1', to clear bit. Write INTR_SET.CH field with '1', to set bit."]
    #[inline(always)]
    pub fn ch(&self) -> CH_R {
        CH_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set to '1', when event (as specified by CH_STATUS.INTR_CAUSE) is detected. Write INTR.CH field with '1', to clear bit. Write INTR_SET.CH field with '1', to set bit."]
    #[inline(always)]
    #[must_use]
    pub fn ch(&mut self) -> CH_W<INTR_SPEC> {
        CH_W::new(self, 0)
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
#[doc = "Interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
