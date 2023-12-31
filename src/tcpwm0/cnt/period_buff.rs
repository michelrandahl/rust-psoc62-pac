#[doc = "Register `PERIOD_BUFF` reader"]
pub type R = crate::R<PERIOD_BUFF_SPEC>;
#[doc = "Register `PERIOD_BUFF` writer"]
pub type W = crate::W<PERIOD_BUFF_SPEC>;
#[doc = "Field `PERIOD` reader - Additional buffer for counter PERIOD register."]
pub type PERIOD_R = crate::FieldReader<u32>;
#[doc = "Field `PERIOD` writer - Additional buffer for counter PERIOD register."]
pub type PERIOD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Additional buffer for counter PERIOD register."]
    #[inline(always)]
    pub fn period(&self) -> PERIOD_R {
        PERIOD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Additional buffer for counter PERIOD register."]
    #[inline(always)]
    #[must_use]
    pub fn period(&mut self) -> PERIOD_W<PERIOD_BUFF_SPEC> {
        PERIOD_W::new(self, 0)
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
#[doc = "Counter buffered period register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`period_buff::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`period_buff::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERIOD_BUFF_SPEC;
impl crate::RegisterSpec for PERIOD_BUFF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`period_buff::R`](R) reader structure"]
impl crate::Readable for PERIOD_BUFF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`period_buff::W`](W) writer structure"]
impl crate::Writable for PERIOD_BUFF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERIOD_BUFF to value 0xffff_ffff"]
impl crate::Resettable for PERIOD_BUFF_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
