#[doc = "Register `CWA16` reader"]
pub type R = crate::R<CWA16_SPEC>;
#[doc = "Register `CWA16` writer"]
pub type W = crate::W<CWA16_SPEC>;
#[doc = "Field `CWA16` reader - Write Address for Common Area"]
pub type CWA16_R = crate::FieldReader<u16>;
#[doc = "Field `CWA16` writer - Write Address for Common Area"]
pub type CWA16_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Write Address for Common Area"]
    #[inline(always)]
    pub fn cwa16(&self) -> CWA16_R {
        CWA16_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Write Address for Common Area"]
    #[inline(always)]
    #[must_use]
    pub fn cwa16(&mut self) -> CWA16_W<CWA16_SPEC> {
        CWA16_W::new(self, 0)
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
#[doc = "Common Area Write Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cwa16::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cwa16::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CWA16_SPEC;
impl crate::RegisterSpec for CWA16_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cwa16::R`](R) reader structure"]
impl crate::Readable for CWA16_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cwa16::W`](W) writer structure"]
impl crate::Writable for CWA16_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CWA16 to value 0"]
impl crate::Resettable for CWA16_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
