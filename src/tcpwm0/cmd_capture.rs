#[doc = "Register `CMD_CAPTURE` reader"]
pub type R = crate::R<CMD_CAPTURE_SPEC>;
#[doc = "Register `CMD_CAPTURE` writer"]
pub type W = crate::W<CMD_CAPTURE_SPEC>;
#[doc = "Field `COUNTER_CAPTURE` reader - Counters SW capture trigger. When written with '1', a capture trigger is generated and the HW sets the field to '0' when the SW trigger has taken effect. It should be noted that the HW operates on the counter frequency. If the counter is disabled through CTRL.COUNTER_ENABLED, the field is immediately set to '0'."]
pub type COUNTER_CAPTURE_R = crate::FieldReader<u32>;
#[doc = "Field `COUNTER_CAPTURE` writer - Counters SW capture trigger. When written with '1', a capture trigger is generated and the HW sets the field to '0' when the SW trigger has taken effect. It should be noted that the HW operates on the counter frequency. If the counter is disabled through CTRL.COUNTER_ENABLED, the field is immediately set to '0'."]
pub type COUNTER_CAPTURE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Counters SW capture trigger. When written with '1', a capture trigger is generated and the HW sets the field to '0' when the SW trigger has taken effect. It should be noted that the HW operates on the counter frequency. If the counter is disabled through CTRL.COUNTER_ENABLED, the field is immediately set to '0'."]
    #[inline(always)]
    pub fn counter_capture(&self) -> COUNTER_CAPTURE_R {
        COUNTER_CAPTURE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counters SW capture trigger. When written with '1', a capture trigger is generated and the HW sets the field to '0' when the SW trigger has taken effect. It should be noted that the HW operates on the counter frequency. If the counter is disabled through CTRL.COUNTER_ENABLED, the field is immediately set to '0'."]
    #[inline(always)]
    #[must_use]
    pub fn counter_capture(&mut self) -> COUNTER_CAPTURE_W<CMD_CAPTURE_SPEC> {
        COUNTER_CAPTURE_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "TCPWM capture command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_capture::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_capture::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_CAPTURE_SPEC;
impl crate::RegisterSpec for CMD_CAPTURE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_capture::R`](R) reader structure"]
impl crate::Readable for CMD_CAPTURE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd_capture::W`](W) writer structure"]
impl crate::Writable for CMD_CAPTURE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD_CAPTURE to value 0"]
impl crate::Resettable for CMD_CAPTURE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
