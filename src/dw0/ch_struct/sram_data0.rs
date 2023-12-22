#[doc = "Register `SRAM_DATA0` reader"]
pub type R = crate::R<SRAM_DATA0_SPEC>;
#[doc = "Register `SRAM_DATA0` writer"]
pub type W = crate::W<SRAM_DATA0_SPEC>;
#[doc = "Field `DATA` reader - N/A"]
pub type DATA_R = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - N/A"]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - N/A"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<SRAM_DATA0_SPEC> {
        DATA_W::new(self, 0)
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
#[doc = "SRAM data 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_data0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_data0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRAM_DATA0_SPEC;
impl crate::RegisterSpec for SRAM_DATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_data0::R`](R) reader structure"]
impl crate::Readable for SRAM_DATA0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sram_data0::W`](W) writer structure"]
impl crate::Writable for SRAM_DATA0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRAM_DATA0 to value 0"]
impl crate::Resettable for SRAM_DATA0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
