#[doc = "Register `ARB_RW3_WA` reader"]
pub type R = crate::R<ARB_RW3_WA_SPEC>;
#[doc = "Register `ARB_RW3_WA` writer"]
pub type W = crate::W<ARB_RW3_WA_SPEC>;
#[doc = "Field `WA` reader - Write Address for EP"]
pub type WA_R = crate::FieldReader;
#[doc = "Field `WA` writer - Write Address for EP"]
pub type WA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Write Address for EP"]
    #[inline(always)]
    pub fn wa(&self) -> WA_R {
        WA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Write Address for EP"]
    #[inline(always)]
    #[must_use]
    pub fn wa(&mut self) -> WA_W<ARB_RW3_WA_SPEC> {
        WA_W::new(self, 0)
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
#[doc = "Endpoint Write Address value *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw3_wa::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw3_wa::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARB_RW3_WA_SPEC;
impl crate::RegisterSpec for ARB_RW3_WA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arb_rw3_wa::R`](R) reader structure"]
impl crate::Readable for ARB_RW3_WA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`arb_rw3_wa::W`](W) writer structure"]
impl crate::Writable for ARB_RW3_WA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ARB_RW3_WA to value 0"]
impl crate::Resettable for ARB_RW3_WA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
