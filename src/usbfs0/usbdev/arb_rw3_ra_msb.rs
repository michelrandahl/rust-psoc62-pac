#[doc = "Register `ARB_RW3_RA_MSB` reader"]
pub type R = crate::R<ARB_RW3_RA_MSB_SPEC>;
#[doc = "Register `ARB_RW3_RA_MSB` writer"]
pub type W = crate::W<ARB_RW3_RA_MSB_SPEC>;
#[doc = "Field `RA_MSB` reader - Read Address for EP"]
pub type RA_MSB_R = crate::BitReader;
#[doc = "Field `RA_MSB` writer - Read Address for EP"]
pub type RA_MSB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Read Address for EP"]
    #[inline(always)]
    pub fn ra_msb(&self) -> RA_MSB_R {
        RA_MSB_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Read Address for EP"]
    #[inline(always)]
    #[must_use]
    pub fn ra_msb(&mut self) -> RA_MSB_W<ARB_RW3_RA_MSB_SPEC> {
        RA_MSB_W::new(self, 0)
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
#[doc = "Endpoint Read Address value *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw3_ra_msb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw3_ra_msb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARB_RW3_RA_MSB_SPEC;
impl crate::RegisterSpec for ARB_RW3_RA_MSB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arb_rw3_ra_msb::R`](R) reader structure"]
impl crate::Readable for ARB_RW3_RA_MSB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`arb_rw3_ra_msb::W`](W) writer structure"]
impl crate::Writable for ARB_RW3_RA_MSB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ARB_RW3_RA_MSB to value 0"]
impl crate::Resettable for ARB_RW3_RA_MSB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
