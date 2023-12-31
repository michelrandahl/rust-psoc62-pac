#[doc = "Register `SAMPLE_TIME23` reader"]
pub type R = crate::R<SAMPLE_TIME23_SPEC>;
#[doc = "Register `SAMPLE_TIME23` writer"]
pub type W = crate::W<SAMPLE_TIME23_SPEC>;
#[doc = "Field `SAMPLE_TIME2` reader - Sample time2"]
pub type SAMPLE_TIME2_R = crate::FieldReader<u16>;
#[doc = "Field `SAMPLE_TIME2` writer - Sample time2"]
pub type SAMPLE_TIME2_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SAMPLE_TIME3` reader - Sample time3"]
pub type SAMPLE_TIME3_R = crate::FieldReader<u16>;
#[doc = "Field `SAMPLE_TIME3` writer - Sample time3"]
pub type SAMPLE_TIME3_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Sample time2"]
    #[inline(always)]
    pub fn sample_time2(&self) -> SAMPLE_TIME2_R {
        SAMPLE_TIME2_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Sample time3"]
    #[inline(always)]
    pub fn sample_time3(&self) -> SAMPLE_TIME3_R {
        SAMPLE_TIME3_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Sample time2"]
    #[inline(always)]
    #[must_use]
    pub fn sample_time2(&mut self) -> SAMPLE_TIME2_W<SAMPLE_TIME23_SPEC> {
        SAMPLE_TIME2_W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Sample time3"]
    #[inline(always)]
    #[must_use]
    pub fn sample_time3(&mut self) -> SAMPLE_TIME3_W<SAMPLE_TIME23_SPEC> {
        SAMPLE_TIME3_W::new(self, 16)
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
#[doc = "Sample time specification ST2 and ST3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sample_time23::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sample_time23::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAMPLE_TIME23_SPEC;
impl crate::RegisterSpec for SAMPLE_TIME23_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sample_time23::R`](R) reader structure"]
impl crate::Readable for SAMPLE_TIME23_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sample_time23::W`](W) writer structure"]
impl crate::Writable for SAMPLE_TIME23_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAMPLE_TIME23 to value 0x0003_0003"]
impl crate::Resettable for SAMPLE_TIME23_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_0003;
}
