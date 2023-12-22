#[doc = "Register `CHAN_EN` reader"]
pub type R = crate::R<CHAN_EN_SPEC>;
#[doc = "Register `CHAN_EN` writer"]
pub type W = crate::W<CHAN_EN_SPEC>;
#[doc = "Field `CHAN_EN` reader - Channel enable. - 0: the corresponding channel is disabled. - 1: the corresponding channel is enabled, it will be included in the next scan."]
pub type CHAN_EN_R = crate::FieldReader<u16>;
#[doc = "Field `CHAN_EN` writer - Channel enable. - 0: the corresponding channel is disabled. - 1: the corresponding channel is enabled, it will be included in the next scan."]
pub type CHAN_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Channel enable. - 0: the corresponding channel is disabled. - 1: the corresponding channel is enabled, it will be included in the next scan."]
    #[inline(always)]
    pub fn chan_en(&self) -> CHAN_EN_R {
        CHAN_EN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Channel enable. - 0: the corresponding channel is disabled. - 1: the corresponding channel is enabled, it will be included in the next scan."]
    #[inline(always)]
    #[must_use]
    pub fn chan_en(&mut self) -> CHAN_EN_W<CHAN_EN_SPEC> {
        CHAN_EN_W::new(self, 0)
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
#[doc = "Enable bits for the channels\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chan_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chan_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHAN_EN_SPEC;
impl crate::RegisterSpec for CHAN_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chan_en::R`](R) reader structure"]
impl crate::Readable for CHAN_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chan_en::W`](W) writer structure"]
impl crate::Writable for CHAN_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHAN_EN to value 0"]
impl crate::Resettable for CHAN_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
