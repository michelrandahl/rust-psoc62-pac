#[doc = "Register `RESULT_VAL2` reader"]
pub type R = crate::R<RESULT_VAL2_SPEC>;
#[doc = "Field `VALUE` reader - Only used in case of Mutual cap with two counters (CSX = config.mutual_cap &amp; config.csx_dual_cnt), this counter counts when csd_sense is low."]
pub type VALUE_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Only used in case of Mutual cap with two counters (CSX = config.mutual_cap &amp; config.csx_dual_cnt), this counter counts when csd_sense is low."]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Result CSX accumulation counter value 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`result_val2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESULT_VAL2_SPEC;
impl crate::RegisterSpec for RESULT_VAL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`result_val2::R`](R) reader structure"]
impl crate::Readable for RESULT_VAL2_SPEC {}
#[doc = "`reset()` method sets RESULT_VAL2 to value 0"]
impl crate::Resettable for RESULT_VAL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
