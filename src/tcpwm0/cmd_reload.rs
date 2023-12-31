#[doc = "Register `CMD_RELOAD` reader"]
pub type R = crate::R<CMD_RELOAD_SPEC>;
#[doc = "Register `CMD_RELOAD` writer"]
pub type W = crate::W<CMD_RELOAD_SPEC>;
#[doc = "Field `COUNTER_RELOAD` reader - Counters SW reload trigger. For HW behavior, see COUNTER_CAPTURE field."]
pub type COUNTER_RELOAD_R = crate::FieldReader<u32>;
#[doc = "Field `COUNTER_RELOAD` writer - Counters SW reload trigger. For HW behavior, see COUNTER_CAPTURE field."]
pub type COUNTER_RELOAD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Counters SW reload trigger. For HW behavior, see COUNTER_CAPTURE field."]
    #[inline(always)]
    pub fn counter_reload(&self) -> COUNTER_RELOAD_R {
        COUNTER_RELOAD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counters SW reload trigger. For HW behavior, see COUNTER_CAPTURE field."]
    #[inline(always)]
    #[must_use]
    pub fn counter_reload(&mut self) -> COUNTER_RELOAD_W<CMD_RELOAD_SPEC> {
        COUNTER_RELOAD_W::new(self, 0)
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
#[doc = "TCPWM reload command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_reload::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_reload::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_RELOAD_SPEC;
impl crate::RegisterSpec for CMD_RELOAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_reload::R`](R) reader structure"]
impl crate::Readable for CMD_RELOAD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd_reload::W`](W) writer structure"]
impl crate::Writable for CMD_RELOAD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD_RELOAD to value 0"]
impl crate::Resettable for CMD_RELOAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
