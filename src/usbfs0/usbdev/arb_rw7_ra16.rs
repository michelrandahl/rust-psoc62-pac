#[doc = "Register `ARB_RW7_RA16` reader"]
pub type R = crate::R<ARB_RW7_RA16_SPEC>;
#[doc = "Register `ARB_RW7_RA16` writer"]
pub type W = crate::W<ARB_RW7_RA16_SPEC>;
#[doc = "Field `RA16` reader - Read Address for EP"]
pub type RA16_R = crate::FieldReader<u16>;
#[doc = "Field `RA16` writer - Read Address for EP"]
pub type RA16_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Read Address for EP"]
    #[inline(always)]
    pub fn ra16(&self) -> RA16_R {
        RA16_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Read Address for EP"]
    #[inline(always)]
    #[must_use]
    pub fn ra16(&mut self) -> RA16_W<ARB_RW7_RA16_SPEC> {
        RA16_W::new(self, 0)
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
#[doc = "Endpoint Read Address value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw7_ra16::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw7_ra16::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARB_RW7_RA16_SPEC;
impl crate::RegisterSpec for ARB_RW7_RA16_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arb_rw7_ra16::R`](R) reader structure"]
impl crate::Readable for ARB_RW7_RA16_SPEC {}
#[doc = "`write(|w| ..)` method takes [`arb_rw7_ra16::W`](W) writer structure"]
impl crate::Writable for ARB_RW7_RA16_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ARB_RW7_RA16 to value 0"]
impl crate::Resettable for ARB_RW7_RA16_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
