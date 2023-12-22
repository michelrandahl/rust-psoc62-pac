#[doc = "Register `ANA_TRIM0` reader"]
pub type R = crate::R<ANA_TRIM0_SPEC>;
#[doc = "Register `ANA_TRIM0` writer"]
pub type W = crate::W<ANA_TRIM0_SPEC>;
#[doc = "Field `CAP_TRIM` reader - Attenuation cap trimming"]
pub type CAP_TRIM_R = crate::FieldReader;
#[doc = "Field `CAP_TRIM` writer - Attenuation cap trimming"]
pub type CAP_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TRIMUNIT` reader - Attenuation cap trimming"]
pub type TRIMUNIT_R = crate::BitReader;
#[doc = "Field `TRIMUNIT` writer - Attenuation cap trimming"]
pub type TRIMUNIT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Attenuation cap trimming"]
    #[inline(always)]
    pub fn cap_trim(&self) -> CAP_TRIM_R {
        CAP_TRIM_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Attenuation cap trimming"]
    #[inline(always)]
    pub fn trimunit(&self) -> TRIMUNIT_R {
        TRIMUNIT_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Attenuation cap trimming"]
    #[inline(always)]
    #[must_use]
    pub fn cap_trim(&mut self) -> CAP_TRIM_W<ANA_TRIM0_SPEC> {
        CAP_TRIM_W::new(self, 0)
    }
    #[doc = "Bit 5 - Attenuation cap trimming"]
    #[inline(always)]
    #[must_use]
    pub fn trimunit(&mut self) -> TRIMUNIT_W<ANA_TRIM0_SPEC> {
        TRIMUNIT_W::new(self, 5)
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
#[doc = "Analog trim register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ana_trim0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ana_trim0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ANA_TRIM0_SPEC;
impl crate::RegisterSpec for ANA_TRIM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ana_trim0::R`](R) reader structure"]
impl crate::Readable for ANA_TRIM0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ana_trim0::W`](W) writer structure"]
impl crate::Writable for ANA_TRIM0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ANA_TRIM0 to value 0"]
impl crate::Resettable for ANA_TRIM0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
