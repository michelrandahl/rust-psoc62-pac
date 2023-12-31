#[doc = "Register `RAM2_STATUS` reader"]
pub type R = crate::R<RAM2_STATUS_SPEC>;
#[doc = "Field `WB_EMPTY` reader - See RAM0_STATUS."]
pub type WB_EMPTY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - See RAM0_STATUS."]
    #[inline(always)]
    pub fn wb_empty(&self) -> WB_EMPTY_R {
        WB_EMPTY_R::new((self.bits & 1) != 0)
    }
}
#[doc = "RAM 2 status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram2_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAM2_STATUS_SPEC;
impl crate::RegisterSpec for RAM2_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ram2_status::R`](R) reader structure"]
impl crate::Readable for RAM2_STATUS_SPEC {}
#[doc = "`reset()` method sets RAM2_STATUS to value 0x01"]
impl crate::Resettable for RAM2_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
