#[doc = "Register `CH_STATUS` reader"]
pub type R = crate::R<CH_STATUS_SPEC>;
#[doc = "Field `INTR_CAUSE` reader - Specifies the source of the interrupt cause: '0': NO_INTR '1': COMPLETION '2': SRC_BUS_ERROR '3': DST_BUS_ERROR '4': SRC_MISAL '5': DST_MISAL '6': CURR_PTR_NULL '7': ACTIVE_CH_DISABLED '8': DESCR_BUS_ERROR '9'-'15': Not used. For error related interrupt causes (INTR_CAUSE is '2', '3', ..., '8'), the channel is disabled (HW sets CH_CTL.ENABLED to '0')."]
pub type INTR_CAUSE_R = crate::FieldReader;
#[doc = "Field `PENDING` reader - Specifies pending DW channels; i.e. enabled channels whose trigger got activated. This field includes all channels that are in the pending state (not scheduled) or active state (scheduled and performing data transfer(s))."]
pub type PENDING_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Specifies the source of the interrupt cause: '0': NO_INTR '1': COMPLETION '2': SRC_BUS_ERROR '3': DST_BUS_ERROR '4': SRC_MISAL '5': DST_MISAL '6': CURR_PTR_NULL '7': ACTIVE_CH_DISABLED '8': DESCR_BUS_ERROR '9'-'15': Not used. For error related interrupt causes (INTR_CAUSE is '2', '3', ..., '8'), the channel is disabled (HW sets CH_CTL.ENABLED to '0')."]
    #[inline(always)]
    pub fn intr_cause(&self) -> INTR_CAUSE_R {
        INTR_CAUSE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Specifies pending DW channels; i.e. enabled channels whose trigger got activated. This field includes all channels that are in the pending state (not scheduled) or active state (scheduled and performing data transfer(s))."]
    #[inline(always)]
    pub fn pending(&self) -> PENDING_R {
        PENDING_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Channel status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_STATUS_SPEC;
impl crate::RegisterSpec for CH_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_status::R`](R) reader structure"]
impl crate::Readable for CH_STATUS_SPEC {}
#[doc = "`reset()` method sets CH_STATUS to value 0"]
impl crate::Resettable for CH_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
