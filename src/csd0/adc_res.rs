#[doc = "Register `ADC_RES` reader"]
pub type R = crate::R<ADC_RES_SPEC>;
#[doc = "Field `VIN_CNT` reader - Count to source/sink Cref1 + Cref2 from Vin to Vrefhi."]
pub type VIN_CNT_R = crate::FieldReader<u16>;
#[doc = "Field `HSCMP_POL` reader - Polarity used for IDACB for this last ADC result, 0= source, 1= sink"]
pub type HSCMP_POL_R = crate::BitReader;
#[doc = "Field `ADC_OVERFLOW` reader - This flag is set when the ADC counter overflows. This is an indication to the firmware that the IDACB current level is too low."]
pub type ADC_OVERFLOW_R = crate::BitReader;
#[doc = "Field `ADC_ABORT` reader - This flag is set when the ADC sequencer was aborted before tripping HSCMP."]
pub type ADC_ABORT_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - Count to source/sink Cref1 + Cref2 from Vin to Vrefhi."]
    #[inline(always)]
    pub fn vin_cnt(&self) -> VIN_CNT_R {
        VIN_CNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Polarity used for IDACB for this last ADC result, 0= source, 1= sink"]
    #[inline(always)]
    pub fn hscmp_pol(&self) -> HSCMP_POL_R {
        HSCMP_POL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 30 - This flag is set when the ADC counter overflows. This is an indication to the firmware that the IDACB current level is too low."]
    #[inline(always)]
    pub fn adc_overflow(&self) -> ADC_OVERFLOW_R {
        ADC_OVERFLOW_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This flag is set when the ADC sequencer was aborted before tripping HSCMP."]
    #[inline(always)]
    pub fn adc_abort(&self) -> ADC_ABORT_R {
        ADC_ABORT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "ADC measurement\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_res::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC_RES_SPEC;
impl crate::RegisterSpec for ADC_RES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_res::R`](R) reader structure"]
impl crate::Readable for ADC_RES_SPEC {}
#[doc = "`reset()` method sets ADC_RES to value 0"]
impl crate::Resettable for ADC_RES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
