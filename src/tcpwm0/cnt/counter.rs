#[doc = "Register `COUNTER` reader"]
pub type R = crate::R<COUNTER_SPEC>;
#[doc = "Register `COUNTER` writer"]
pub type W = crate::W<COUNTER_SPEC>;
#[doc = "Field `COUNTER` reader - 16-bit / 32-bit counter value. It is advised to not write to this field when the counter is running."]
pub type COUNTER_R = crate::FieldReader<u32>;
#[doc = "Field `COUNTER` writer - 16-bit / 32-bit counter value. It is advised to not write to this field when the counter is running."]
pub type COUNTER_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 16-bit / 32-bit counter value. It is advised to not write to this field when the counter is running."]
    #[inline(always)]
    pub fn counter(&self) -> COUNTER_R {
        COUNTER_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 16-bit / 32-bit counter value. It is advised to not write to this field when the counter is running."]
    #[inline(always)]
    #[must_use]
    pub fn counter(&mut self) -> COUNTER_W<COUNTER_SPEC> {
        COUNTER_W::new(self, 0)
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
#[doc = "Counter count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`counter::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`counter::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COUNTER_SPEC;
impl crate::RegisterSpec for COUNTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`counter::R`](R) reader structure"]
impl crate::Readable for COUNTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`counter::W`](W) writer structure"]
impl crate::Writable for COUNTER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COUNTER to value 0"]
impl crate::Resettable for COUNTER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
