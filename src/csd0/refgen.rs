#[doc = "Register `REFGEN` reader"]
pub type R = crate::R<REFGEN_SPEC>;
#[doc = "Register `REFGEN` writer"]
pub type W = crate::W<REFGEN_SPEC>;
#[doc = "Field `REFGEN_EN` reader - Reference Generator Enable"]
pub type REFGEN_EN_R = crate::BitReader<REFGEN_EN_A>;
#[doc = "Reference Generator Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFGEN_EN_A {
    #[doc = "0: Disable Reference Generator"]
    OFF = 0,
    #[doc = "1: On, regular operation. Note that CONFIG.LP_MODE determines the power mode level"]
    ON = 1,
}
impl From<REFGEN_EN_A> for bool {
    #[inline(always)]
    fn from(variant: REFGEN_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl REFGEN_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> REFGEN_EN_A {
        match self.bits {
            false => REFGEN_EN_A::OFF,
            true => REFGEN_EN_A::ON,
        }
    }
    #[doc = "Disable Reference Generator"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == REFGEN_EN_A::OFF
    }
    #[doc = "On, regular operation. Note that CONFIG.LP_MODE determines the power mode level"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == REFGEN_EN_A::ON
    }
}
#[doc = "Field `REFGEN_EN` writer - Reference Generator Enable"]
pub type REFGEN_EN_W<'a, REG> = crate::BitWriter<'a, REG, REFGEN_EN_A>;
impl<'a, REG> REFGEN_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Reference Generator"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(REFGEN_EN_A::OFF)
    }
    #[doc = "On, regular operation. Note that CONFIG.LP_MODE determines the power mode level"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(REFGEN_EN_A::ON)
    }
}
#[doc = "Field `BYPASS` reader - Bypass selected input reference unbuffered to Vrefhi"]
pub type BYPASS_R = crate::BitReader;
#[doc = "Field `BYPASS` writer - Bypass selected input reference unbuffered to Vrefhi"]
pub type BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDA_EN` reader - Close Vdda switch to top of resistor string (or Vrefhi?)"]
pub type VDDA_EN_R = crate::BitReader;
#[doc = "Field `VDDA_EN` writer - Close Vdda switch to top of resistor string (or Vrefhi?)"]
pub type VDDA_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES_EN` reader - Resistor string enable; 0= open switch on top of the resistor string (Vreflo=Vssa)"]
pub type RES_EN_R = crate::BitReader;
#[doc = "Field `RES_EN` writer - Resistor string enable; 0= open switch on top of the resistor string (Vreflo=Vssa)"]
pub type RES_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAIN` reader - Select resistor string tap for feedback, 0= minimum vout, 31= maximum vout = vrefhi -> gain=1 (only works if the resistor string is enabled; RES_EN=1)"]
pub type GAIN_R = crate::FieldReader;
#[doc = "Field `GAIN` writer - Select resistor string tap for feedback, 0= minimum vout, 31= maximum vout = vrefhi -> gain=1 (only works if the resistor string is enabled; RES_EN=1)"]
pub type GAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `VREFLO_SEL` reader - Select resistor string tap for Vreflo/Vreflo_int, 0= minimum vout, 31= maximum vout = vrefhi (only works if the resistor string is enabled; RES_EN=1)"]
pub type VREFLO_SEL_R = crate::FieldReader;
#[doc = "Field `VREFLO_SEL` writer - Select resistor string tap for Vreflo/Vreflo_int, 0= minimum vout, 31= maximum vout = vrefhi (only works if the resistor string is enabled; RES_EN=1)"]
pub type VREFLO_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `VREFLO_INT` reader - Ouput the resistor string tap either to Vreflo (0) or Vreflo_int (1)."]
pub type VREFLO_INT_R = crate::BitReader;
#[doc = "Field `VREFLO_INT` writer - Ouput the resistor string tap either to Vreflo (0) or Vreflo_int (1)."]
pub type VREFLO_INT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reference Generator Enable"]
    #[inline(always)]
    pub fn refgen_en(&self) -> REFGEN_EN_R {
        REFGEN_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Bypass selected input reference unbuffered to Vrefhi"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Close Vdda switch to top of resistor string (or Vrefhi?)"]
    #[inline(always)]
    pub fn vdda_en(&self) -> VDDA_EN_R {
        VDDA_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Resistor string enable; 0= open switch on top of the resistor string (Vreflo=Vssa)"]
    #[inline(always)]
    pub fn res_en(&self) -> RES_EN_R {
        RES_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Select resistor string tap for feedback, 0= minimum vout, 31= maximum vout = vrefhi -> gain=1 (only works if the resistor string is enabled; RES_EN=1)"]
    #[inline(always)]
    pub fn gain(&self) -> GAIN_R {
        GAIN_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Select resistor string tap for Vreflo/Vreflo_int, 0= minimum vout, 31= maximum vout = vrefhi (only works if the resistor string is enabled; RES_EN=1)"]
    #[inline(always)]
    pub fn vreflo_sel(&self) -> VREFLO_SEL_R {
        VREFLO_SEL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - Ouput the resistor string tap either to Vreflo (0) or Vreflo_int (1)."]
    #[inline(always)]
    pub fn vreflo_int(&self) -> VREFLO_INT_R {
        VREFLO_INT_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reference Generator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn refgen_en(&mut self) -> REFGEN_EN_W<REFGEN_SPEC> {
        REFGEN_EN_W::new(self, 0)
    }
    #[doc = "Bit 4 - Bypass selected input reference unbuffered to Vrefhi"]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BYPASS_W<REFGEN_SPEC> {
        BYPASS_W::new(self, 4)
    }
    #[doc = "Bit 5 - Close Vdda switch to top of resistor string (or Vrefhi?)"]
    #[inline(always)]
    #[must_use]
    pub fn vdda_en(&mut self) -> VDDA_EN_W<REFGEN_SPEC> {
        VDDA_EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Resistor string enable; 0= open switch on top of the resistor string (Vreflo=Vssa)"]
    #[inline(always)]
    #[must_use]
    pub fn res_en(&mut self) -> RES_EN_W<REFGEN_SPEC> {
        RES_EN_W::new(self, 6)
    }
    #[doc = "Bits 8:12 - Select resistor string tap for feedback, 0= minimum vout, 31= maximum vout = vrefhi -> gain=1 (only works if the resistor string is enabled; RES_EN=1)"]
    #[inline(always)]
    #[must_use]
    pub fn gain(&mut self) -> GAIN_W<REFGEN_SPEC> {
        GAIN_W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Select resistor string tap for Vreflo/Vreflo_int, 0= minimum vout, 31= maximum vout = vrefhi (only works if the resistor string is enabled; RES_EN=1)"]
    #[inline(always)]
    #[must_use]
    pub fn vreflo_sel(&mut self) -> VREFLO_SEL_W<REFGEN_SPEC> {
        VREFLO_SEL_W::new(self, 16)
    }
    #[doc = "Bit 23 - Ouput the resistor string tap either to Vreflo (0) or Vreflo_int (1)."]
    #[inline(always)]
    #[must_use]
    pub fn vreflo_int(&mut self) -> VREFLO_INT_W<REFGEN_SPEC> {
        VREFLO_INT_W::new(self, 23)
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
#[doc = "Reference Generator configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`refgen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`refgen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REFGEN_SPEC;
impl crate::RegisterSpec for REFGEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`refgen::R`](R) reader structure"]
impl crate::Readable for REFGEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`refgen::W`](W) writer structure"]
impl crate::Writable for REFGEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REFGEN to value 0"]
impl crate::Resettable for REFGEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
