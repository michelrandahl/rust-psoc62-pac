#[doc = "Register `IZTAT_TRIM1` reader"]
pub type R = crate::R<IZTAT_TRIM1_SPEC>;
#[doc = "Register `IZTAT_TRIM1` writer"]
pub type W = crate::W<IZTAT_TRIM1_SPEC>;
#[doc = "Field `IZTAT_TC_TRIM` reader - IZTAT temperature correction trim (RMB) 0x00 : No IZTAT temperature correction 0xFF : Maximum IZTAT temperature correction As this is a Risk Mitigation Register, it should be loaded with 0x08."]
pub type IZTAT_TC_TRIM_R = crate::FieldReader;
#[doc = "Field `IZTAT_TC_TRIM` writer - IZTAT temperature correction trim (RMB) 0x00 : No IZTAT temperature correction 0xFF : Maximum IZTAT temperature correction As this is a Risk Mitigation Register, it should be loaded with 0x08."]
pub type IZTAT_TC_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - IZTAT temperature correction trim (RMB) 0x00 : No IZTAT temperature correction 0xFF : Maximum IZTAT temperature correction As this is a Risk Mitigation Register, it should be loaded with 0x08."]
    #[inline(always)]
    pub fn iztat_tc_trim(&self) -> IZTAT_TC_TRIM_R {
        IZTAT_TC_TRIM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - IZTAT temperature correction trim (RMB) 0x00 : No IZTAT temperature correction 0xFF : Maximum IZTAT temperature correction As this is a Risk Mitigation Register, it should be loaded with 0x08."]
    #[inline(always)]
    #[must_use]
    pub fn iztat_tc_trim(&mut self) -> IZTAT_TC_TRIM_W<IZTAT_TRIM1_SPEC> {
        IZTAT_TC_TRIM_W::new(self, 0)
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
#[doc = "IZTAT Trim bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iztat_trim1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iztat_trim1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IZTAT_TRIM1_SPEC;
impl crate::RegisterSpec for IZTAT_TRIM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iztat_trim1::R`](R) reader structure"]
impl crate::Readable for IZTAT_TRIM1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iztat_trim1::W`](W) writer structure"]
impl crate::Writable for IZTAT_TRIM1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IZTAT_TRIM1 to value 0"]
impl crate::Resettable for IZTAT_TRIM1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
