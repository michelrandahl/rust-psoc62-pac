#[doc = "Register `AMBUF` reader"]
pub type R = crate::R<AMBUF_SPEC>;
#[doc = "Register `AMBUF` writer"]
pub type W = crate::W<AMBUF_SPEC>;
#[doc = "Field `PWR_MODE` reader - Amux buffer power level"]
pub type PWR_MODE_R = crate::FieldReader<PWR_MODE_A>;
#[doc = "Amux buffer power level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWR_MODE_A {
    #[doc = "0: Disable buffer"]
    OFF = 0,
    #[doc = "1: On, normal or low power level depending on CONFIG.LP_MODE."]
    NORM = 1,
    #[doc = "2: On, high or low power level depending on CONFIG.LP_MODE."]
    HI = 2,
}
impl From<PWR_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PWR_MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PWR_MODE_A {
    type Ux = u8;
}
impl PWR_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PWR_MODE_A> {
        match self.bits {
            0 => Some(PWR_MODE_A::OFF),
            1 => Some(PWR_MODE_A::NORM),
            2 => Some(PWR_MODE_A::HI),
            _ => None,
        }
    }
    #[doc = "Disable buffer"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == PWR_MODE_A::OFF
    }
    #[doc = "On, normal or low power level depending on CONFIG.LP_MODE."]
    #[inline(always)]
    pub fn is_norm(&self) -> bool {
        *self == PWR_MODE_A::NORM
    }
    #[doc = "On, high or low power level depending on CONFIG.LP_MODE."]
    #[inline(always)]
    pub fn is_hi(&self) -> bool {
        *self == PWR_MODE_A::HI
    }
}
#[doc = "Field `PWR_MODE` writer - Amux buffer power level"]
pub type PWR_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PWR_MODE_A>;
impl<'a, REG> PWR_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable buffer"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(PWR_MODE_A::OFF)
    }
    #[doc = "On, normal or low power level depending on CONFIG.LP_MODE."]
    #[inline(always)]
    pub fn norm(self) -> &'a mut crate::W<REG> {
        self.variant(PWR_MODE_A::NORM)
    }
    #[doc = "On, high or low power level depending on CONFIG.LP_MODE."]
    #[inline(always)]
    pub fn hi(self) -> &'a mut crate::W<REG> {
        self.variant(PWR_MODE_A::HI)
    }
}
impl R {
    #[doc = "Bits 0:1 - Amux buffer power level"]
    #[inline(always)]
    pub fn pwr_mode(&self) -> PWR_MODE_R {
        PWR_MODE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Amux buffer power level"]
    #[inline(always)]
    #[must_use]
    pub fn pwr_mode(&mut self) -> PWR_MODE_W<AMBUF_SPEC> {
        PWR_MODE_W::new(self, 0)
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
#[doc = "Reference Generator configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ambuf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ambuf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AMBUF_SPEC;
impl crate::RegisterSpec for AMBUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ambuf::R`](R) reader structure"]
impl crate::Readable for AMBUF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ambuf::W`](W) writer structure"]
impl crate::Writable for AMBUF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AMBUF to value 0"]
impl crate::Resettable for AMBUF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
