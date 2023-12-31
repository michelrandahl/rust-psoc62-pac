#[doc = "Register `ARB_RW2_DR16` reader"]
pub type R = crate::R<ARB_RW2_DR16_SPEC>;
#[doc = "Register `ARB_RW2_DR16` writer"]
pub type W = crate::W<ARB_RW2_DR16_SPEC>;
#[doc = "Field `DR16` reader - Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
pub type DR16_R = crate::FieldReader<u16>;
#[doc = "Field `DR16` writer - Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
pub type DR16_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub fn dr16(&self) -> DR16_R {
        DR16_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    #[must_use]
    pub fn dr16(&mut self) -> DR16_W<ARB_RW2_DR16_SPEC> {
        DR16_W::new(self, 0)
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
#[doc = "Endpoint Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw2_dr16::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw2_dr16::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARB_RW2_DR16_SPEC;
impl crate::RegisterSpec for ARB_RW2_DR16_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arb_rw2_dr16::R`](R) reader structure"]
impl crate::Readable for ARB_RW2_DR16_SPEC {}
#[doc = "`write(|w| ..)` method takes [`arb_rw2_dr16::W`](W) writer structure"]
impl crate::Writable for ARB_RW2_DR16_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ARB_RW2_DR16 to value 0"]
impl crate::Resettable for ARB_RW2_DR16_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
