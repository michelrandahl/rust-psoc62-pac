#[doc = "Register `IZTAT_TRIM0` reader"]
pub type R = crate::R<IZTAT_TRIM0_SPEC>;
#[doc = "Register `IZTAT_TRIM0` writer"]
pub type W = crate::W<IZTAT_TRIM0_SPEC>;
#[doc = "Field `IZTAT_ABS_TRIM` reader - N/A"]
pub type IZTAT_ABS_TRIM_R = crate::FieldReader;
#[doc = "Field `IZTAT_ABS_TRIM` writer - N/A"]
pub type IZTAT_ABS_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    pub fn iztat_abs_trim(&self) -> IZTAT_ABS_TRIM_R {
        IZTAT_ABS_TRIM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn iztat_abs_trim(&mut self) -> IZTAT_ABS_TRIM_W<IZTAT_TRIM0_SPEC> {
        IZTAT_ABS_TRIM_W::new(self, 0)
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
#[doc = "IZTAT Trim bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iztat_trim0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iztat_trim0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IZTAT_TRIM0_SPEC;
impl crate::RegisterSpec for IZTAT_TRIM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iztat_trim0::R`](R) reader structure"]
impl crate::Readable for IZTAT_TRIM0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iztat_trim0::W`](W) writer structure"]
impl crate::Writable for IZTAT_TRIM0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IZTAT_TRIM0 to value 0"]
impl crate::Resettable for IZTAT_TRIM0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
