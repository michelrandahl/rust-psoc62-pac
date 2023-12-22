#[doc = "Register `CQCRI` reader"]
pub type R = crate::R<CQCRI_SPEC>;
#[doc = "Field `CMD_RESP_INDX` reader - Last Command Response index This field stores the index of the last received command response. Controller updates the value every time a command response is received."]
pub type CMD_RESP_INDX_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Last Command Response index This field stores the index of the last received command response. Controller updates the value every time a command response is received."]
    #[inline(always)]
    pub fn cmd_resp_indx(&self) -> CMD_RESP_INDX_R {
        CMD_RESP_INDX_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "CQ Command response index\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqcri::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CQCRI_SPEC;
impl crate::RegisterSpec for CQCRI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqcri::R`](R) reader structure"]
impl crate::Readable for CQCRI_SPEC {}
#[doc = "`reset()` method sets CQCRI to value 0"]
impl crate::Resettable for CQCRI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
