#[doc = "Register `IPTAT_TRIM0` reader"]
pub type R = crate::R<IPTAT_TRIM0_SPEC>;
#[doc = "Register `IPTAT_TRIM0` writer"]
pub type W = crate::W<IPTAT_TRIM0_SPEC>;
#[doc = "Field `IPTAT_CORE_TRIM` reader - IPTAT trim 0x0 : Minimum IPTAT current (~150nA at room) 0xF : Maximum IPTAT current (~350nA at room)"]
pub type IPTAT_CORE_TRIM_R = crate::FieldReader;
#[doc = "Field `IPTAT_CORE_TRIM` writer - IPTAT trim 0x0 : Minimum IPTAT current (~150nA at room) 0xF : Maximum IPTAT current (~350nA at room)"]
pub type IPTAT_CORE_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IPTAT_CTBM_TRIM` reader - CTMB PTAT Current Trim 0x0 : Minimum CTMB IPTAT Current (~875nA) 0xF : Maximum CTMB IPTAT Current (~1.1uA)"]
pub type IPTAT_CTBM_TRIM_R = crate::FieldReader;
#[doc = "Field `IPTAT_CTBM_TRIM` writer - CTMB PTAT Current Trim 0x0 : Minimum CTMB IPTAT Current (~875nA) 0xF : Maximum CTMB IPTAT Current (~1.1uA)"]
pub type IPTAT_CTBM_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - IPTAT trim 0x0 : Minimum IPTAT current (~150nA at room) 0xF : Maximum IPTAT current (~350nA at room)"]
    #[inline(always)]
    pub fn iptat_core_trim(&self) -> IPTAT_CORE_TRIM_R {
        IPTAT_CORE_TRIM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - CTMB PTAT Current Trim 0x0 : Minimum CTMB IPTAT Current (~875nA) 0xF : Maximum CTMB IPTAT Current (~1.1uA)"]
    #[inline(always)]
    pub fn iptat_ctbm_trim(&self) -> IPTAT_CTBM_TRIM_R {
        IPTAT_CTBM_TRIM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - IPTAT trim 0x0 : Minimum IPTAT current (~150nA at room) 0xF : Maximum IPTAT current (~350nA at room)"]
    #[inline(always)]
    #[must_use]
    pub fn iptat_core_trim(&mut self) -> IPTAT_CORE_TRIM_W<IPTAT_TRIM0_SPEC> {
        IPTAT_CORE_TRIM_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - CTMB PTAT Current Trim 0x0 : Minimum CTMB IPTAT Current (~875nA) 0xF : Maximum CTMB IPTAT Current (~1.1uA)"]
    #[inline(always)]
    #[must_use]
    pub fn iptat_ctbm_trim(&mut self) -> IPTAT_CTBM_TRIM_W<IPTAT_TRIM0_SPEC> {
        IPTAT_CTBM_TRIM_W::new(self, 4)
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
#[doc = "IPTAT Trim bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iptat_trim0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iptat_trim0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IPTAT_TRIM0_SPEC;
impl crate::RegisterSpec for IPTAT_TRIM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iptat_trim0::R`](R) reader structure"]
impl crate::Readable for IPTAT_TRIM0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iptat_trim0::W`](W) writer structure"]
impl crate::Writable for IPTAT_TRIM0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IPTAT_TRIM0 to value 0"]
impl crate::Resettable for IPTAT_TRIM0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
