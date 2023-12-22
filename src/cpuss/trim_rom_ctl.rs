#[doc = "Register `TRIM_ROM_CTL` reader"]
pub type R = crate::R<TRIM_ROM_CTL_SPEC>;
#[doc = "Register `TRIM_ROM_CTL` writer"]
pub type W = crate::W<TRIM_ROM_CTL_SPEC>;
#[doc = "Field `TRIM` reader - N/A"]
pub type TRIM_R = crate::FieldReader<u32>;
#[doc = "Field `TRIM` writer - N/A"]
pub type TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - N/A"]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn trim(&mut self) -> TRIM_W<TRIM_ROM_CTL_SPEC> {
        TRIM_W::new(self, 0)
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
#[doc = "ROM trim control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trim_rom_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trim_rom_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRIM_ROM_CTL_SPEC;
impl crate::RegisterSpec for TRIM_ROM_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trim_rom_ctl::R`](R) reader structure"]
impl crate::Readable for TRIM_ROM_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trim_rom_ctl::W`](W) writer structure"]
impl crate::Writable for TRIM_ROM_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRIM_ROM_CTL to value 0"]
impl crate::Resettable for TRIM_ROM_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
