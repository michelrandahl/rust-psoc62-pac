#[doc = "Register `CM0_CA_STATUS1` reader"]
pub type R = crate::R<CM0_CA_STATUS1_SPEC>;
#[doc = "Field `TAG` reader - Cache line address of the cache line specified by CM0_CA_CTL.WAY and CM0_CA_CTL.SET_ADDR."]
pub type TAG_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Cache line address of the cache line specified by CM0_CA_CTL.WAY and CM0_CA_CTL.SET_ADDR."]
    #[inline(always)]
    pub fn tag(&self) -> TAG_R {
        TAG_R::new(self.bits)
    }
}
#[doc = "CM0+ cache status 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm0_ca_status1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CM0_CA_STATUS1_SPEC;
impl crate::RegisterSpec for CM0_CA_STATUS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm0_ca_status1::R`](R) reader structure"]
impl crate::Readable for CM0_CA_STATUS1_SPEC {}
#[doc = "`reset()` method sets CM0_CA_STATUS1 to value 0"]
impl crate::Resettable for CM0_CA_STATUS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
