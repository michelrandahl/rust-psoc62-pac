#[doc = "Register `ADC_CTL` reader"]
pub type R = crate::R<ADC_CTL_SPEC>;
#[doc = "Register `ADC_CTL` writer"]
pub type W = crate::W<ADC_CTL_SPEC>;
#[doc = "Field `ADC_TIME` reader - ADC timing -1 in csd_sense clock cycles (actual time is ADC_TIME+1 cycles), either used to discharge Cref1&amp;2, or as the aperture to capture the input voltage on Cref1&amp;2"]
pub type ADC_TIME_R = crate::FieldReader;
#[doc = "Field `ADC_TIME` writer - ADC timing -1 in csd_sense clock cycles (actual time is ADC_TIME+1 cycles), either used to discharge Cref1&amp;2, or as the aperture to capture the input voltage on Cref1&amp;2"]
pub type ADC_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ADC_MODE` reader - Enable ADC measurement. When enabled the ADC sequencer will be started when the main sequencer goes to the SAMPLE_NORM state"]
pub type ADC_MODE_R = crate::FieldReader<ADC_MODE_A>;
#[doc = "Enable ADC measurement. When enabled the ADC sequencer will be started when the main sequencer goes to the SAMPLE_NORM state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC_MODE_A {
    #[doc = "0: No ADC measurement"]
    OFF = 0,
    #[doc = "1: Count time A to bring Cref1 + Cref2 up from Vssa to Vrefhi with IDACB"]
    VREF_CNT = 1,
    #[doc = "2: Count time B to bring Cref1 + Cref2 back up to Vrefhi with IDACB (after bringing them down for time A/2 cycles with IDACB sinking)"]
    VREF_BY2_CNT = 2,
    #[doc = "3: Determine HSCMP polarity and count time C to source/sink Cref1 + Cref2 from Vin to Vrefhi."]
    VIN_CNT = 3,
}
impl From<ADC_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADC_MODE_A {
    type Ux = u8;
}
impl ADC_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC_MODE_A {
        match self.bits {
            0 => ADC_MODE_A::OFF,
            1 => ADC_MODE_A::VREF_CNT,
            2 => ADC_MODE_A::VREF_BY2_CNT,
            3 => ADC_MODE_A::VIN_CNT,
            _ => unreachable!(),
        }
    }
    #[doc = "No ADC measurement"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == ADC_MODE_A::OFF
    }
    #[doc = "Count time A to bring Cref1 + Cref2 up from Vssa to Vrefhi with IDACB"]
    #[inline(always)]
    pub fn is_vref_cnt(&self) -> bool {
        *self == ADC_MODE_A::VREF_CNT
    }
    #[doc = "Count time B to bring Cref1 + Cref2 back up to Vrefhi with IDACB (after bringing them down for time A/2 cycles with IDACB sinking)"]
    #[inline(always)]
    pub fn is_vref_by2_cnt(&self) -> bool {
        *self == ADC_MODE_A::VREF_BY2_CNT
    }
    #[doc = "Determine HSCMP polarity and count time C to source/sink Cref1 + Cref2 from Vin to Vrefhi."]
    #[inline(always)]
    pub fn is_vin_cnt(&self) -> bool {
        *self == ADC_MODE_A::VIN_CNT
    }
}
#[doc = "Field `ADC_MODE` writer - Enable ADC measurement. When enabled the ADC sequencer will be started when the main sequencer goes to the SAMPLE_NORM state"]
pub type ADC_MODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ADC_MODE_A>;
impl<'a, REG> ADC_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No ADC measurement"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_MODE_A::OFF)
    }
    #[doc = "Count time A to bring Cref1 + Cref2 up from Vssa to Vrefhi with IDACB"]
    #[inline(always)]
    pub fn vref_cnt(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_MODE_A::VREF_CNT)
    }
    #[doc = "Count time B to bring Cref1 + Cref2 back up to Vrefhi with IDACB (after bringing them down for time A/2 cycles with IDACB sinking)"]
    #[inline(always)]
    pub fn vref_by2_cnt(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_MODE_A::VREF_BY2_CNT)
    }
    #[doc = "Determine HSCMP polarity and count time C to source/sink Cref1 + Cref2 from Vin to Vrefhi."]
    #[inline(always)]
    pub fn vin_cnt(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_MODE_A::VIN_CNT)
    }
}
impl R {
    #[doc = "Bits 0:7 - ADC timing -1 in csd_sense clock cycles (actual time is ADC_TIME+1 cycles), either used to discharge Cref1&amp;2, or as the aperture to capture the input voltage on Cref1&amp;2"]
    #[inline(always)]
    pub fn adc_time(&self) -> ADC_TIME_R {
        ADC_TIME_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Enable ADC measurement. When enabled the ADC sequencer will be started when the main sequencer goes to the SAMPLE_NORM state"]
    #[inline(always)]
    pub fn adc_mode(&self) -> ADC_MODE_R {
        ADC_MODE_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADC timing -1 in csd_sense clock cycles (actual time is ADC_TIME+1 cycles), either used to discharge Cref1&amp;2, or as the aperture to capture the input voltage on Cref1&amp;2"]
    #[inline(always)]
    #[must_use]
    pub fn adc_time(&mut self) -> ADC_TIME_W<ADC_CTL_SPEC> {
        ADC_TIME_W::new(self, 0)
    }
    #[doc = "Bits 16:17 - Enable ADC measurement. When enabled the ADC sequencer will be started when the main sequencer goes to the SAMPLE_NORM state"]
    #[inline(always)]
    #[must_use]
    pub fn adc_mode(&mut self) -> ADC_MODE_W<ADC_CTL_SPEC> {
        ADC_MODE_W::new(self, 16)
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
#[doc = "ADC Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC_CTL_SPEC;
impl crate::RegisterSpec for ADC_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_ctl::R`](R) reader structure"]
impl crate::Readable for ADC_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adc_ctl::W`](W) writer structure"]
impl crate::Writable for ADC_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_CTL to value 0"]
impl crate::Resettable for ADC_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
