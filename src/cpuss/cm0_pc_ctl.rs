#[doc = "Register `CM0_PC_CTL` reader"]
pub type R = crate::R<CM0_PC_CTL_SPEC>;
#[doc = "Register `CM0_PC_CTL` writer"]
pub type W = crate::W<CM0_PC_CTL_SPEC>;
#[doc = "Field `VALID` reader - Valid fields for the protection context handler CM0_PCi_HANDLER registers: Bit 0: Valid field for CM0_PC0_HANDLER. Bit 1: Valid field for CM0_PC1_HANDLER. Bit 2: Valid field for CM0_PC2_HANDLER. Bit 3: Valid field for CM0_PC3_HANDLER."]
pub type VALID_R = crate::FieldReader;
#[doc = "Field `VALID` writer - Valid fields for the protection context handler CM0_PCi_HANDLER registers: Bit 0: Valid field for CM0_PC0_HANDLER. Bit 1: Valid field for CM0_PC1_HANDLER. Bit 2: Valid field for CM0_PC2_HANDLER. Bit 3: Valid field for CM0_PC3_HANDLER."]
pub type VALID_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Valid fields for the protection context handler CM0_PCi_HANDLER registers: Bit 0: Valid field for CM0_PC0_HANDLER. Bit 1: Valid field for CM0_PC1_HANDLER. Bit 2: Valid field for CM0_PC2_HANDLER. Bit 3: Valid field for CM0_PC3_HANDLER."]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Valid fields for the protection context handler CM0_PCi_HANDLER registers: Bit 0: Valid field for CM0_PC0_HANDLER. Bit 1: Valid field for CM0_PC1_HANDLER. Bit 2: Valid field for CM0_PC2_HANDLER. Bit 3: Valid field for CM0_PC3_HANDLER."]
    #[inline(always)]
    #[must_use]
    pub fn valid(&mut self) -> VALID_W<CM0_PC_CTL_SPEC> {
        VALID_W::new(self, 0)
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
#[doc = "CM0+ protection context control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm0_pc_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm0_pc_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CM0_PC_CTL_SPEC;
impl crate::RegisterSpec for CM0_PC_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm0_pc_ctl::R`](R) reader structure"]
impl crate::Readable for CM0_PC_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cm0_pc_ctl::W`](W) writer structure"]
impl crate::Writable for CM0_PC_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CM0_PC_CTL to value 0"]
impl crate::Resettable for CM0_PC_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
