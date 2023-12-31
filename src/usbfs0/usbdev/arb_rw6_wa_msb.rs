#[doc = "Register `ARB_RW6_WA_MSB` reader"]
pub type R = crate::R<ARB_RW6_WA_MSB_SPEC>;
#[doc = "Register `ARB_RW6_WA_MSB` writer"]
pub type W = crate::W<ARB_RW6_WA_MSB_SPEC>;
#[doc = "Field `WA_MSB` reader - Write Address for EP"]
pub type WA_MSB_R = crate::BitReader;
#[doc = "Field `WA_MSB` writer - Write Address for EP"]
pub type WA_MSB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write Address for EP"]
    #[inline(always)]
    pub fn wa_msb(&self) -> WA_MSB_R {
        WA_MSB_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write Address for EP"]
    #[inline(always)]
    #[must_use]
    pub fn wa_msb(&mut self) -> WA_MSB_W<ARB_RW6_WA_MSB_SPEC> {
        WA_MSB_W::new(self, 0)
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
#[doc = "Endpoint Write Address value *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw6_wa_msb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw6_wa_msb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARB_RW6_WA_MSB_SPEC;
impl crate::RegisterSpec for ARB_RW6_WA_MSB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arb_rw6_wa_msb::R`](R) reader structure"]
impl crate::Readable for ARB_RW6_WA_MSB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`arb_rw6_wa_msb::W`](W) writer structure"]
impl crate::Writable for ARB_RW6_WA_MSB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ARB_RW6_WA_MSB to value 0"]
impl crate::Resettable for ARB_RW6_WA_MSB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
