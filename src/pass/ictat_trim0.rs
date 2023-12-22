#[doc = "Register `ICTAT_TRIM0` reader"]
pub type R = crate::R<ICTAT_TRIM0_SPEC>;
#[doc = "Register `ICTAT_TRIM0` writer"]
pub type W = crate::W<ICTAT_TRIM0_SPEC>;
#[doc = "Field `ICTAT_TRIM` reader - ICTAT trim 0x00 : Minimum ICTAT current (~150nA at room) 0x0F : Maximum ICTAT current (~350nA at room)"]
pub type ICTAT_TRIM_R = crate::FieldReader;
#[doc = "Field `ICTAT_TRIM` writer - ICTAT trim 0x00 : Minimum ICTAT current (~150nA at room) 0x0F : Maximum ICTAT current (~350nA at room)"]
pub type ICTAT_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - ICTAT trim 0x00 : Minimum ICTAT current (~150nA at room) 0x0F : Maximum ICTAT current (~350nA at room)"]
    #[inline(always)]
    pub fn ictat_trim(&self) -> ICTAT_TRIM_R {
        ICTAT_TRIM_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ICTAT trim 0x00 : Minimum ICTAT current (~150nA at room) 0x0F : Maximum ICTAT current (~350nA at room)"]
    #[inline(always)]
    #[must_use]
    pub fn ictat_trim(&mut self) -> ICTAT_TRIM_W<ICTAT_TRIM0_SPEC> {
        ICTAT_TRIM_W::new(self, 0)
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
#[doc = "ICTAT Trim bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ictat_trim0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ictat_trim0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICTAT_TRIM0_SPEC;
impl crate::RegisterSpec for ICTAT_TRIM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ictat_trim0::R`](R) reader structure"]
impl crate::Readable for ICTAT_TRIM0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ictat_trim0::W`](W) writer structure"]
impl crate::Writable for ICTAT_TRIM0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICTAT_TRIM0 to value 0"]
impl crate::Resettable for ICTAT_TRIM0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
