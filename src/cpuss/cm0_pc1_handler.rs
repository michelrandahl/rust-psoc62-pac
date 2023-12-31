#[doc = "Register `CM0_PC1_HANDLER` reader"]
pub type R = crate::R<CM0_PC1_HANDLER_SPEC>;
#[doc = "Register `CM0_PC1_HANDLER` writer"]
pub type W = crate::W<CM0_PC1_HANDLER_SPEC>;
#[doc = "Field `ADDR` reader - Address of the protection context 1 handler."]
pub type ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Address of the protection context 1 handler."]
pub type ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Address of the protection context 1 handler."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Address of the protection context 1 handler."]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<CM0_PC1_HANDLER_SPEC> {
        ADDR_W::new(self, 0)
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
#[doc = "CM0+ protection context 1 handler\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm0_pc1_handler::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm0_pc1_handler::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CM0_PC1_HANDLER_SPEC;
impl crate::RegisterSpec for CM0_PC1_HANDLER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm0_pc1_handler::R`](R) reader structure"]
impl crate::Readable for CM0_PC1_HANDLER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cm0_pc1_handler::W`](W) writer structure"]
impl crate::Writable for CM0_PC1_HANDLER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CM0_PC1_HANDLER to value 0"]
impl crate::Resettable for CM0_PC1_HANDLER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
