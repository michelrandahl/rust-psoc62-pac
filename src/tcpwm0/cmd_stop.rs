#[doc = "Register `CMD_STOP` reader"]
pub type R = crate::R<CMD_STOP_SPEC>;
#[doc = "Register `CMD_STOP` writer"]
pub type W = crate::W<CMD_STOP_SPEC>;
#[doc = "Field `COUNTER_STOP` reader - Counters SW stop trigger. For HW behavior, see COUNTER_CAPTURE field."]
pub type COUNTER_STOP_R = crate::FieldReader<u32>;
#[doc = "Field `COUNTER_STOP` writer - Counters SW stop trigger. For HW behavior, see COUNTER_CAPTURE field."]
pub type COUNTER_STOP_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Counters SW stop trigger. For HW behavior, see COUNTER_CAPTURE field."]
    #[inline(always)]
    pub fn counter_stop(&self) -> COUNTER_STOP_R {
        COUNTER_STOP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counters SW stop trigger. For HW behavior, see COUNTER_CAPTURE field."]
    #[inline(always)]
    #[must_use]
    pub fn counter_stop(&mut self) -> COUNTER_STOP_W<CMD_STOP_SPEC> {
        COUNTER_STOP_W::new(self, 0)
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
#[doc = "TCPWM stop command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_stop::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_stop::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_STOP_SPEC;
impl crate::RegisterSpec for CMD_STOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_stop::R`](R) reader structure"]
impl crate::Readable for CMD_STOP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd_stop::W`](W) writer structure"]
impl crate::Writable for CMD_STOP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD_STOP to value 0"]
impl crate::Resettable for CMD_STOP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
