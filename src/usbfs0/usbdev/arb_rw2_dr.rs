#[doc = "Register `ARB_RW2_DR` reader"]
pub type R = crate::R<ARB_RW2_DR_SPEC>;
#[doc = "Register `ARB_RW2_DR` writer"]
pub type W = crate::W<ARB_RW2_DR_SPEC>;
#[doc = "Field `DR` reader - Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
pub type DR_R = crate::FieldReader;
#[doc = "Field `DR` writer - Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
pub type DR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    #[must_use]
    pub fn dr(&mut self) -> DR_W<ARB_RW2_DR_SPEC> {
        DR_W::new(self, 0)
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
#[doc = "Endpoint Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw2_dr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw2_dr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARB_RW2_DR_SPEC;
impl crate::RegisterSpec for ARB_RW2_DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arb_rw2_dr::R`](R) reader structure"]
impl crate::Readable for ARB_RW2_DR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`arb_rw2_dr::W`](W) writer structure"]
impl crate::Writable for ARB_RW2_DR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ARB_RW2_DR to value 0"]
impl crate::Resettable for ARB_RW2_DR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
