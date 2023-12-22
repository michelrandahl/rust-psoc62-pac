#[doc = "Register `RESP23_R` reader"]
pub type R = crate::R<RESP23_R_SPEC>;
#[doc = "Field `RESP23` reader - Command Response These bits reflect 71-40 bits of the SD/eMMC Response"]
pub type RESP23_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Command Response These bits reflect 71-40 bits of the SD/eMMC Response"]
    #[inline(always)]
    pub fn resp23(&self) -> RESP23_R {
        RESP23_R::new(self.bits)
    }
}
#[doc = "Response Register 2/3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp23_r::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESP23_R_SPEC;
impl crate::RegisterSpec for RESP23_R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp23_r::R`](R) reader structure"]
impl crate::Readable for RESP23_R_SPEC {}
#[doc = "`reset()` method sets RESP23_R to value 0"]
impl crate::Resettable for RESP23_R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
