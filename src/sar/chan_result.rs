#[doc = "Register `CHAN_RESULT[%s]` reader"]
pub type R = crate::R<CHAN_RESULT_SPEC>;
#[doc = "Field `RESULT` reader - SAR conversion result of the channel. The data is copied here from the WORK field after all enabled channels in this scan have been sampled."]
pub type RESULT_R = crate::FieldReader<u16>;
#[doc = "Field `CHAN_RESULT_NEWVALUE_MIR` reader - mirror bit of corresponding bit in SAR_CHAN_RESULT_NEWVALUE register"]
pub type CHAN_RESULT_NEWVALUE_MIR_R = crate::BitReader;
#[doc = "Field `SATURATE_INTR_MIR` reader - mirror bit of corresponding bit in SAR_SATURATE_INTR register"]
pub type SATURATE_INTR_MIR_R = crate::BitReader;
#[doc = "Field `RANGE_INTR_MIR` reader - mirror bit of corresponding bit in SAR_RANGE_INTR register"]
pub type RANGE_INTR_MIR_R = crate::BitReader;
#[doc = "Field `CHAN_RESULT_UPDATED_MIR` reader - mirror bit of corresponding bit in SAR_CHAN_RESULT_UPDATED register"]
pub type CHAN_RESULT_UPDATED_MIR_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - SAR conversion result of the channel. The data is copied here from the WORK field after all enabled channels in this scan have been sampled."]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 27 - mirror bit of corresponding bit in SAR_CHAN_RESULT_NEWVALUE register"]
    #[inline(always)]
    pub fn chan_result_newvalue_mir(&self) -> CHAN_RESULT_NEWVALUE_MIR_R {
        CHAN_RESULT_NEWVALUE_MIR_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - mirror bit of corresponding bit in SAR_SATURATE_INTR register"]
    #[inline(always)]
    pub fn saturate_intr_mir(&self) -> SATURATE_INTR_MIR_R {
        SATURATE_INTR_MIR_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - mirror bit of corresponding bit in SAR_RANGE_INTR register"]
    #[inline(always)]
    pub fn range_intr_mir(&self) -> RANGE_INTR_MIR_R {
        RANGE_INTR_MIR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - mirror bit of corresponding bit in SAR_CHAN_RESULT_UPDATED register"]
    #[inline(always)]
    pub fn chan_result_updated_mir(&self) -> CHAN_RESULT_UPDATED_MIR_R {
        CHAN_RESULT_UPDATED_MIR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Channel result data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chan_result::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHAN_RESULT_SPEC;
impl crate::RegisterSpec for CHAN_RESULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chan_result::R`](R) reader structure"]
impl crate::Readable for CHAN_RESULT_SPEC {}
#[doc = "`reset()` method sets CHAN_RESULT[%s]
to value 0"]
impl crate::Resettable for CHAN_RESULT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
