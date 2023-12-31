#[doc = "Register `CQCRA` reader"]
pub type R = crate::R<CQCRA_SPEC>;
#[doc = "Field `CMD_RESP_ARG` reader - Last Command Response argument This field stores the argument of the last received command response. Controller updates the value every time a command response is received."]
pub type CMD_RESP_ARG_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Last Command Response argument This field stores the argument of the last received command response. Controller updates the value every time a command response is received."]
    #[inline(always)]
    pub fn cmd_resp_arg(&self) -> CMD_RESP_ARG_R {
        CMD_RESP_ARG_R::new(self.bits)
    }
}
#[doc = "CQ Command response argument register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqcra::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CQCRA_SPEC;
impl crate::RegisterSpec for CQCRA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqcra::R`](R) reader structure"]
impl crate::Readable for CQCRA_SPEC {}
#[doc = "`reset()` method sets CQCRA to value 0"]
impl crate::Resettable for CQCRA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
