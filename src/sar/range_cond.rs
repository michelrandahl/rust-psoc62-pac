#[doc = "Register `RANGE_COND` reader"]
pub type R = crate::R<RANGE_COND_SPEC>;
#[doc = "Register `RANGE_COND` writer"]
pub type W = crate::W<RANGE_COND_SPEC>;
#[doc = "Field `RANGE_COND` reader - Range condition select."]
pub type RANGE_COND_R = crate::FieldReader<RANGE_COND_A>;
#[doc = "Range condition select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RANGE_COND_A {
    #[doc = "0: result &lt; RANGE_LOW"]
    BELOW = 0,
    #[doc = "1: RANGE_LOW &lt;= result &lt; RANGE_HIGH"]
    INSIDE = 1,
    #[doc = "2: RANGE_HIGH &lt;= result"]
    ABOVE = 2,
    #[doc = "3: result &lt; RANGE_LOW || RANGE_HIGH &lt;= result"]
    OUTSIDE = 3,
}
impl From<RANGE_COND_A> for u8 {
    #[inline(always)]
    fn from(variant: RANGE_COND_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RANGE_COND_A {
    type Ux = u8;
}
impl RANGE_COND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RANGE_COND_A {
        match self.bits {
            0 => RANGE_COND_A::BELOW,
            1 => RANGE_COND_A::INSIDE,
            2 => RANGE_COND_A::ABOVE,
            3 => RANGE_COND_A::OUTSIDE,
            _ => unreachable!(),
        }
    }
    #[doc = "result &lt; RANGE_LOW"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == RANGE_COND_A::BELOW
    }
    #[doc = "RANGE_LOW &lt;= result &lt; RANGE_HIGH"]
    #[inline(always)]
    pub fn is_inside(&self) -> bool {
        *self == RANGE_COND_A::INSIDE
    }
    #[doc = "RANGE_HIGH &lt;= result"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == RANGE_COND_A::ABOVE
    }
    #[doc = "result &lt; RANGE_LOW || RANGE_HIGH &lt;= result"]
    #[inline(always)]
    pub fn is_outside(&self) -> bool {
        *self == RANGE_COND_A::OUTSIDE
    }
}
#[doc = "Field `RANGE_COND` writer - Range condition select."]
pub type RANGE_COND_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, RANGE_COND_A>;
impl<'a, REG> RANGE_COND_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "result &lt; RANGE_LOW"]
    #[inline(always)]
    pub fn below(self) -> &'a mut crate::W<REG> {
        self.variant(RANGE_COND_A::BELOW)
    }
    #[doc = "RANGE_LOW &lt;= result &lt; RANGE_HIGH"]
    #[inline(always)]
    pub fn inside(self) -> &'a mut crate::W<REG> {
        self.variant(RANGE_COND_A::INSIDE)
    }
    #[doc = "RANGE_HIGH &lt;= result"]
    #[inline(always)]
    pub fn above(self) -> &'a mut crate::W<REG> {
        self.variant(RANGE_COND_A::ABOVE)
    }
    #[doc = "result &lt; RANGE_LOW || RANGE_HIGH &lt;= result"]
    #[inline(always)]
    pub fn outside(self) -> &'a mut crate::W<REG> {
        self.variant(RANGE_COND_A::OUTSIDE)
    }
}
impl R {
    #[doc = "Bits 30:31 - Range condition select."]
    #[inline(always)]
    pub fn range_cond(&self) -> RANGE_COND_R {
        RANGE_COND_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Range condition select."]
    #[inline(always)]
    #[must_use]
    pub fn range_cond(&mut self) -> RANGE_COND_W<RANGE_COND_SPEC> {
        RANGE_COND_W::new(self, 30)
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
#[doc = "Global range detect mode register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`range_cond::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`range_cond::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RANGE_COND_SPEC;
impl crate::RegisterSpec for RANGE_COND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`range_cond::R`](R) reader structure"]
impl crate::Readable for RANGE_COND_SPEC {}
#[doc = "`write(|w| ..)` method takes [`range_cond::W`](W) writer structure"]
impl crate::Writable for RANGE_COND_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RANGE_COND to value 0"]
impl crate::Resettable for RANGE_COND_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
