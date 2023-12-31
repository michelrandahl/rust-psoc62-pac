#[doc = "Register `ACT_DESCR_SRC` reader"]
pub type R = crate::R<ACT_DESCR_SRC_SPEC>;
#[doc = "Field `DATA` reader - Copy of DESCR_SRC of the currently active descriptor."]
pub type DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Copy of DESCR_SRC of the currently active descriptor."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
#[doc = "Active descriptor source\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`act_descr_src::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACT_DESCR_SRC_SPEC;
impl crate::RegisterSpec for ACT_DESCR_SRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`act_descr_src::R`](R) reader structure"]
impl crate::Readable for ACT_DESCR_SRC_SPEC {}
#[doc = "`reset()` method sets ACT_DESCR_SRC to value 0"]
impl crate::Resettable for ACT_DESCR_SRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
