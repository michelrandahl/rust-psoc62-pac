#[doc = "Register `RESP67_R` reader"]
pub type R = crate::R<RESP67_R_SPEC>;
#[doc = "Field `RESP67` reader - Command Response These bits reflect bits 135-104 of SD/EMMC Response Field. Note: For Auto CMD, this register also reflects the 32-bit response (bits 39-8 of the Response Field)."]
pub type RESP67_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Command Response These bits reflect bits 135-104 of SD/EMMC Response Field. Note: For Auto CMD, this register also reflects the 32-bit response (bits 39-8 of the Response Field)."]
    #[inline(always)]
    pub fn resp67(&self) -> RESP67_R {
        RESP67_R::new(self.bits)
    }
}
#[doc = "Response Register 6/7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp67_r::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESP67_R_SPEC;
impl crate::RegisterSpec for RESP67_R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp67_r::R`](R) reader structure"]
impl crate::Readable for RESP67_R_SPEC {}
#[doc = "`reset()` method sets RESP67_R to value 0"]
impl crate::Resettable for RESP67_R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
