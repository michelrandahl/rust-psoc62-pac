#[doc = "Register `STAT_SEQ` reader"]
pub type R = crate::R<STAT_SEQ_SPEC>;
#[doc = "Field `SEQ_STATE` reader - CSD sequencer state"]
pub type SEQ_STATE_R = crate::FieldReader;
#[doc = "Field `ADC_STATE` reader - ADC sequencer state (only relevant after SEQ_STATE has reached SAMPLE_NORM and ADC sequencer has started)"]
pub type ADC_STATE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - CSD sequencer state"]
    #[inline(always)]
    pub fn seq_state(&self) -> SEQ_STATE_R {
        SEQ_STATE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 16:18 - ADC sequencer state (only relevant after SEQ_STATE has reached SAMPLE_NORM and ADC sequencer has started)"]
    #[inline(always)]
    pub fn adc_state(&self) -> ADC_STATE_R {
        ADC_STATE_R::new(((self.bits >> 16) & 7) as u8)
    }
}
#[doc = "Current Sequencer status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat_seq::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STAT_SEQ_SPEC;
impl crate::RegisterSpec for STAT_SEQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat_seq::R`](R) reader structure"]
impl crate::Readable for STAT_SEQ_SPEC {}
#[doc = "`reset()` method sets STAT_SEQ to value 0"]
impl crate::Resettable for STAT_SEQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
