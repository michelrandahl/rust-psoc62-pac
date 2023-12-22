#[doc = "Register `ACT_SRC` reader"]
pub type R = crate::R<ACT_SRC_SPEC>;
#[doc = "Field `SRC_ADDR` reader - Current address of source location."]
pub type SRC_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Current address of source location."]
    #[inline(always)]
    pub fn src_addr(&self) -> SRC_ADDR_R {
        SRC_ADDR_R::new(self.bits)
    }
}
#[doc = "Active source\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`act_src::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACT_SRC_SPEC;
impl crate::RegisterSpec for ACT_SRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`act_src::R`](R) reader structure"]
impl crate::Readable for ACT_SRC_SPEC {}
#[doc = "`reset()` method sets ACT_SRC to value 0"]
impl crate::Resettable for ACT_SRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
