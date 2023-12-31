#[doc = "Register `CHAN_RESULT_UPDATED` reader"]
pub type R = crate::R<CHAN_RESULT_UPDATED_SPEC>;
#[doc = "Field `CHAN_RESULT_UPDATED` reader - If set the corresponding RESULT register was updated, i.e. was sampled during the previous scan and, in case of Interleaved averaging, reached the averaging count. If this bit is low then either the channel is not enabled or the averaging count is not yet reached for Interleaved averaging."]
pub type CHAN_RESULT_UPDATED_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - If set the corresponding RESULT register was updated, i.e. was sampled during the previous scan and, in case of Interleaved averaging, reached the averaging count. If this bit is low then either the channel is not enabled or the averaging count is not yet reached for Interleaved averaging."]
    #[inline(always)]
    pub fn chan_result_updated(&self) -> CHAN_RESULT_UPDATED_R {
        CHAN_RESULT_UPDATED_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Channel result data register 'updated' bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chan_result_updated::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHAN_RESULT_UPDATED_SPEC;
impl crate::RegisterSpec for CHAN_RESULT_UPDATED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chan_result_updated::R`](R) reader structure"]
impl crate::Readable for CHAN_RESULT_UPDATED_SPEC {}
#[doc = "`reset()` method sets CHAN_RESULT_UPDATED to value 0"]
impl crate::Resettable for CHAN_RESULT_UPDATED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
