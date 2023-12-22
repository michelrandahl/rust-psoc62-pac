#[doc = "Register `HOST_RTIMER` reader"]
pub type R = crate::R<HOST_RTIMER_SPEC>;
#[doc = "Register `HOST_RTIMER` writer"]
pub type W = crate::W<HOST_RTIMER_SPEC>;
#[doc = "Field `RTIMER` reader - These bits are used to specify the retry time in this register. The retry timer is activated when token sending starts while the RETRY bit of Host Control 2 Register (HOST_CTL2) is '1'. The retry time is then decremented by one when a 1-bit transfer clock (12 MHz in the full-speed mode) is output. When the retry timer reaches 0, the target token is sent, and processing is ended. If a token retry occurs in the EOF area, the retry timer is stopped until SOF sending is ended. After SOF sending has been completed, the retry timer restarts with the value that is set when the timer stopped."]
pub type RTIMER_R = crate::FieldReader<u32>;
#[doc = "Field `RTIMER` writer - These bits are used to specify the retry time in this register. The retry timer is activated when token sending starts while the RETRY bit of Host Control 2 Register (HOST_CTL2) is '1'. The retry time is then decremented by one when a 1-bit transfer clock (12 MHz in the full-speed mode) is output. When the retry timer reaches 0, the target token is sent, and processing is ended. If a token retry occurs in the EOF area, the retry timer is stopped until SOF sending is ended. After SOF sending has been completed, the retry timer restarts with the value that is set when the timer stopped."]
pub type RTIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - These bits are used to specify the retry time in this register. The retry timer is activated when token sending starts while the RETRY bit of Host Control 2 Register (HOST_CTL2) is '1'. The retry time is then decremented by one when a 1-bit transfer clock (12 MHz in the full-speed mode) is output. When the retry timer reaches 0, the target token is sent, and processing is ended. If a token retry occurs in the EOF area, the retry timer is stopped until SOF sending is ended. After SOF sending has been completed, the retry timer restarts with the value that is set when the timer stopped."]
    #[inline(always)]
    pub fn rtimer(&self) -> RTIMER_R {
        RTIMER_R::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - These bits are used to specify the retry time in this register. The retry timer is activated when token sending starts while the RETRY bit of Host Control 2 Register (HOST_CTL2) is '1'. The retry time is then decremented by one when a 1-bit transfer clock (12 MHz in the full-speed mode) is output. When the retry timer reaches 0, the target token is sent, and processing is ended. If a token retry occurs in the EOF area, the retry timer is stopped until SOF sending is ended. After SOF sending has been completed, the retry timer restarts with the value that is set when the timer stopped."]
    #[inline(always)]
    #[must_use]
    pub fn rtimer(&mut self) -> RTIMER_W<HOST_RTIMER_SPEC> {
        RTIMER_W::new(self, 0)
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
#[doc = "Host Retry Timer Setup Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_rtimer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_rtimer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_RTIMER_SPEC;
impl crate::RegisterSpec for HOST_RTIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_rtimer::R`](R) reader structure"]
impl crate::Readable for HOST_RTIMER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_rtimer::W`](W) writer structure"]
impl crate::Writable for HOST_RTIMER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOST_RTIMER to value 0"]
impl crate::Resettable for HOST_RTIMER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
