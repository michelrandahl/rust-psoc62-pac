#[doc = "Register `CWA` reader"]
pub type R = crate::R<CWA_SPEC>;
#[doc = "Register `CWA` writer"]
pub type W = crate::W<CWA_SPEC>;
#[doc = "Field `CWA` reader - Write Address for Common Area"]
pub type CWA_R = crate::FieldReader;
#[doc = "Field `CWA` writer - Write Address for Common Area"]
pub type CWA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Write Address for Common Area"]
    #[inline(always)]
    pub fn cwa(&self) -> CWA_R {
        CWA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Write Address for Common Area"]
    #[inline(always)]
    #[must_use]
    pub fn cwa(&mut self) -> CWA_W<CWA_SPEC> {
        CWA_W::new(self, 0)
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
#[doc = "Common Area Write Address *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cwa::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cwa::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CWA_SPEC;
impl crate::RegisterSpec for CWA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cwa::R`](R) reader structure"]
impl crate::Readable for CWA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cwa::W`](W) writer structure"]
impl crate::Writable for CWA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CWA to value 0"]
impl crate::Resettable for CWA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
