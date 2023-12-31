#[doc = "Register `SEQ_TIME` reader"]
pub type R = crate::R<SEQ_TIME_SPEC>;
#[doc = "Register `SEQ_TIME` writer"]
pub type W = crate::W<SEQ_TIME_SPEC>;
#[doc = "Field `AZ_TIME` reader - Define Auto-Zero time in csd_sense cycles -1."]
pub type AZ_TIME_R = crate::FieldReader;
#[doc = "Field `AZ_TIME` writer - Define Auto-Zero time in csd_sense cycles -1."]
pub type AZ_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Define Auto-Zero time in csd_sense cycles -1."]
    #[inline(always)]
    pub fn az_time(&self) -> AZ_TIME_R {
        AZ_TIME_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Define Auto-Zero time in csd_sense cycles -1."]
    #[inline(always)]
    #[must_use]
    pub fn az_time(&mut self) -> AZ_TIME_W<SEQ_TIME_SPEC> {
        AZ_TIME_W::new(self, 0)
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
#[doc = "Sequencer Timing\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seq_time::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seq_time::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEQ_TIME_SPEC;
impl crate::RegisterSpec for SEQ_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seq_time::R`](R) reader structure"]
impl crate::Readable for SEQ_TIME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`seq_time::W`](W) writer structure"]
impl crate::Writable for SEQ_TIME_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEQ_TIME to value 0"]
impl crate::Resettable for SEQ_TIME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
