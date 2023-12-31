#[doc = "Register `ACT_DST` reader"]
pub type R = crate::R<ACT_DST_SPEC>;
#[doc = "Field `DST_ADDR` reader - Current address of destination location."]
pub type DST_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Current address of destination location."]
    #[inline(always)]
    pub fn dst_addr(&self) -> DST_ADDR_R {
        DST_ADDR_R::new(self.bits)
    }
}
#[doc = "Active destination\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`act_dst::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACT_DST_SPEC;
impl crate::RegisterSpec for ACT_DST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`act_dst::R`](R) reader structure"]
impl crate::Readable for ACT_DST_SPEC {}
#[doc = "`reset()` method sets ACT_DST to value 0"]
impl crate::Resettable for ACT_DST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
