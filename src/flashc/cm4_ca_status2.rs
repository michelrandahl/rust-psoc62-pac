#[doc = "Register `CM4_CA_STATUS2` reader"]
pub type R = crate::R<CM4_CA_STATUS2_SPEC>;
#[doc = "Field `LRU` reader - See CM0_CA_STATUS2."]
pub type LRU_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - See CM0_CA_STATUS2."]
    #[inline(always)]
    pub fn lru(&self) -> LRU_R {
        LRU_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "CM4 cache status 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm4_ca_status2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CM4_CA_STATUS2_SPEC;
impl crate::RegisterSpec for CM4_CA_STATUS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm4_ca_status2::R`](R) reader structure"]
impl crate::Readable for CM4_CA_STATUS2_SPEC {}
#[doc = "`reset()` method sets CM4_CA_STATUS2 to value 0"]
impl crate::Resettable for CM4_CA_STATUS2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
