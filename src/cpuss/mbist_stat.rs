#[doc = "Register `MBIST_STAT` reader"]
pub type R = crate::R<MBIST_STAT_SPEC>;
#[doc = "Field `SFP_READY` reader - Flag indicating the BIST run is done. Note that after starting a BIST run this flag must be set before a new run can be started. For the first BIST run this will be 0."]
pub type SFP_READY_R = crate::BitReader;
#[doc = "Field `SFP_FAIL` reader - Report status of the BIST run, only valid if SFP_READY=1"]
pub type SFP_FAIL_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Flag indicating the BIST run is done. Note that after starting a BIST run this flag must be set before a new run can be started. For the first BIST run this will be 0."]
    #[inline(always)]
    pub fn sfp_ready(&self) -> SFP_READY_R {
        SFP_READY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Report status of the BIST run, only valid if SFP_READY=1"]
    #[inline(always)]
    pub fn sfp_fail(&self) -> SFP_FAIL_R {
        SFP_FAIL_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Memory BIST status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mbist_stat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MBIST_STAT_SPEC;
impl crate::RegisterSpec for MBIST_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mbist_stat::R`](R) reader structure"]
impl crate::Readable for MBIST_STAT_SPEC {}
#[doc = "`reset()` method sets MBIST_STAT to value 0"]
impl crate::Resettable for MBIST_STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
