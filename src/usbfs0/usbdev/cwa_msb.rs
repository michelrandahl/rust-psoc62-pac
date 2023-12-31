#[doc = "Register `CWA_MSB` reader"]
pub type R = crate::R<CWA_MSB_SPEC>;
#[doc = "Register `CWA_MSB` writer"]
pub type W = crate::W<CWA_MSB_SPEC>;
#[doc = "Field `CWA_MSB` reader - Write Address for Common Area"]
pub type CWA_MSB_R = crate::BitReader;
#[doc = "Field `CWA_MSB` writer - Write Address for Common Area"]
pub type CWA_MSB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write Address for Common Area"]
    #[inline(always)]
    pub fn cwa_msb(&self) -> CWA_MSB_R {
        CWA_MSB_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write Address for Common Area"]
    #[inline(always)]
    #[must_use]
    pub fn cwa_msb(&mut self) -> CWA_MSB_W<CWA_MSB_SPEC> {
        CWA_MSB_W::new(self, 0)
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
#[doc = "Endpoint Read Address value *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cwa_msb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cwa_msb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CWA_MSB_SPEC;
impl crate::RegisterSpec for CWA_MSB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cwa_msb::R`](R) reader structure"]
impl crate::Readable for CWA_MSB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cwa_msb::W`](W) writer structure"]
impl crate::Writable for CWA_MSB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CWA_MSB to value 0"]
impl crate::Resettable for CWA_MSB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
