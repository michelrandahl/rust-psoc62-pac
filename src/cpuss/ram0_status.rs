#[doc = "Register `RAM0_STATUS` reader"]
pub type R = crate::R<RAM0_STATUS_SPEC>;
#[doc = "Field `WB_EMPTY` reader - Write buffer empty. This information is used when entering DeepSleep power mode: WB_EMPTY must be '1' before a transition to system DeepSleep power mode. '0': Write buffer NOT empty. '1': Write buffer empty. Note: the SRAM controller write buffer is only used when ECC checking is enabled. (RAMi_CTL.ECC_EN is '1')."]
pub type WB_EMPTY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Write buffer empty. This information is used when entering DeepSleep power mode: WB_EMPTY must be '1' before a transition to system DeepSleep power mode. '0': Write buffer NOT empty. '1': Write buffer empty. Note: the SRAM controller write buffer is only used when ECC checking is enabled. (RAMi_CTL.ECC_EN is '1')."]
    #[inline(always)]
    pub fn wb_empty(&self) -> WB_EMPTY_R {
        WB_EMPTY_R::new((self.bits & 1) != 0)
    }
}
#[doc = "RAM 0 status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram0_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAM0_STATUS_SPEC;
impl crate::RegisterSpec for RAM0_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ram0_status::R`](R) reader structure"]
impl crate::Readable for RAM0_STATUS_SPEC {}
#[doc = "`reset()` method sets RAM0_STATUS to value 0x01"]
impl crate::Resettable for RAM0_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
