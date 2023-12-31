#[doc = "Register `SAMPLE_TIME01` reader"]
pub type R = crate::R<SAMPLE_TIME01_SPEC>;
#[doc = "Register `SAMPLE_TIME01` writer"]
pub type W = crate::W<SAMPLE_TIME01_SPEC>;
#[doc = "Field `SAMPLE_TIME0` reader - Sample time0 (aperture) in ADC clock cycles. Note that actual sample time is one clock less than specified here. The minimum sample time is 167ns, which is 3.0 cycles (4 in this field) with an 18MHz clock. Minimum legal value in this register is 2."]
pub type SAMPLE_TIME0_R = crate::FieldReader<u16>;
#[doc = "Field `SAMPLE_TIME0` writer - Sample time0 (aperture) in ADC clock cycles. Note that actual sample time is one clock less than specified here. The minimum sample time is 167ns, which is 3.0 cycles (4 in this field) with an 18MHz clock. Minimum legal value in this register is 2."]
pub type SAMPLE_TIME0_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SAMPLE_TIME1` reader - Sample time1"]
pub type SAMPLE_TIME1_R = crate::FieldReader<u16>;
#[doc = "Field `SAMPLE_TIME1` writer - Sample time1"]
pub type SAMPLE_TIME1_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Sample time0 (aperture) in ADC clock cycles. Note that actual sample time is one clock less than specified here. The minimum sample time is 167ns, which is 3.0 cycles (4 in this field) with an 18MHz clock. Minimum legal value in this register is 2."]
    #[inline(always)]
    pub fn sample_time0(&self) -> SAMPLE_TIME0_R {
        SAMPLE_TIME0_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Sample time1"]
    #[inline(always)]
    pub fn sample_time1(&self) -> SAMPLE_TIME1_R {
        SAMPLE_TIME1_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Sample time0 (aperture) in ADC clock cycles. Note that actual sample time is one clock less than specified here. The minimum sample time is 167ns, which is 3.0 cycles (4 in this field) with an 18MHz clock. Minimum legal value in this register is 2."]
    #[inline(always)]
    #[must_use]
    pub fn sample_time0(&mut self) -> SAMPLE_TIME0_W<SAMPLE_TIME01_SPEC> {
        SAMPLE_TIME0_W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Sample time1"]
    #[inline(always)]
    #[must_use]
    pub fn sample_time1(&mut self) -> SAMPLE_TIME1_W<SAMPLE_TIME01_SPEC> {
        SAMPLE_TIME1_W::new(self, 16)
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
#[doc = "Sample time specification ST0 and ST1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sample_time01::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sample_time01::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAMPLE_TIME01_SPEC;
impl crate::RegisterSpec for SAMPLE_TIME01_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sample_time01::R`](R) reader structure"]
impl crate::Readable for SAMPLE_TIME01_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sample_time01::W`](W) writer structure"]
impl crate::Writable for SAMPLE_TIME01_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAMPLE_TIME01 to value 0x0003_0003"]
impl crate::Resettable for SAMPLE_TIME01_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_0003;
}
