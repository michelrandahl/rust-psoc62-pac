#[doc = "Register `TR_CTRL2` reader"]
pub type R = crate::R<TR_CTRL2_SPEC>;
#[doc = "Register `TR_CTRL2` writer"]
pub type W = crate::W<TR_CTRL2_SPEC>;
#[doc = "Field `CC_MATCH_MODE` reader - Determines the effect of a compare match event (COUNTER equals CC register) on the 'line_out' output signals. Note that INVERT is especially useful for center aligned pulse width modulation. To generate a duty cycle of 0 percent, the counter CC register should be set to '0'. For a 100 percent duty cycle, the counter CC register should be set to larger than the counter PERIOD register."]
pub type CC_MATCH_MODE_R = crate::FieldReader<CC_MATCH_MODE_A>;
#[doc = "Determines the effect of a compare match event (COUNTER equals CC register) on the 'line_out' output signals. Note that INVERT is especially useful for center aligned pulse width modulation. To generate a duty cycle of 0 percent, the counter CC register should be set to '0'. For a 100 percent duty cycle, the counter CC register should be set to larger than the counter PERIOD register.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CC_MATCH_MODE_A {
    #[doc = "0: Set to '1'"]
    SET = 0,
    #[doc = "1: Set to '0'"]
    CLEAR = 1,
    #[doc = "2: Invert"]
    INVERT = 2,
    #[doc = "3: No Change"]
    NO_CHANGE = 3,
}
impl From<CC_MATCH_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CC_MATCH_MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CC_MATCH_MODE_A {
    type Ux = u8;
}
impl CC_MATCH_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CC_MATCH_MODE_A {
        match self.bits {
            0 => CC_MATCH_MODE_A::SET,
            1 => CC_MATCH_MODE_A::CLEAR,
            2 => CC_MATCH_MODE_A::INVERT,
            3 => CC_MATCH_MODE_A::NO_CHANGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Set to '1'"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CC_MATCH_MODE_A::SET
    }
    #[doc = "Set to '0'"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CC_MATCH_MODE_A::CLEAR
    }
    #[doc = "Invert"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == CC_MATCH_MODE_A::INVERT
    }
    #[doc = "No Change"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == CC_MATCH_MODE_A::NO_CHANGE
    }
}
#[doc = "Field `CC_MATCH_MODE` writer - Determines the effect of a compare match event (COUNTER equals CC register) on the 'line_out' output signals. Note that INVERT is especially useful for center aligned pulse width modulation. To generate a duty cycle of 0 percent, the counter CC register should be set to '0'. For a 100 percent duty cycle, the counter CC register should be set to larger than the counter PERIOD register."]
pub type CC_MATCH_MODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CC_MATCH_MODE_A>;
impl<'a, REG> CC_MATCH_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Set to '1'"]
    #[inline(always)]
    pub fn set(self) -> &'a mut crate::W<REG> {
        self.variant(CC_MATCH_MODE_A::SET)
    }
    #[doc = "Set to '0'"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CC_MATCH_MODE_A::CLEAR)
    }
    #[doc = "Invert"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut crate::W<REG> {
        self.variant(CC_MATCH_MODE_A::INVERT)
    }
    #[doc = "No Change"]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut crate::W<REG> {
        self.variant(CC_MATCH_MODE_A::NO_CHANGE)
    }
}
#[doc = "Field `OVERFLOW_MODE` reader - Determines the effect of a counter overflow event (COUNTER reaches PERIOD) on the 'line_out' output signals."]
pub type OVERFLOW_MODE_R = crate::FieldReader<OVERFLOW_MODE_A>;
#[doc = "Determines the effect of a counter overflow event (COUNTER reaches PERIOD) on the 'line_out' output signals.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OVERFLOW_MODE_A {
    #[doc = "0: Set to '1'"]
    SET = 0,
    #[doc = "1: Set to '0'"]
    CLEAR = 1,
    #[doc = "2: Invert"]
    INVERT = 2,
    #[doc = "3: No Change"]
    NO_CHANGE = 3,
}
impl From<OVERFLOW_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: OVERFLOW_MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OVERFLOW_MODE_A {
    type Ux = u8;
}
impl OVERFLOW_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVERFLOW_MODE_A {
        match self.bits {
            0 => OVERFLOW_MODE_A::SET,
            1 => OVERFLOW_MODE_A::CLEAR,
            2 => OVERFLOW_MODE_A::INVERT,
            3 => OVERFLOW_MODE_A::NO_CHANGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Set to '1'"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == OVERFLOW_MODE_A::SET
    }
    #[doc = "Set to '0'"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == OVERFLOW_MODE_A::CLEAR
    }
    #[doc = "Invert"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == OVERFLOW_MODE_A::INVERT
    }
    #[doc = "No Change"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == OVERFLOW_MODE_A::NO_CHANGE
    }
}
#[doc = "Field `OVERFLOW_MODE` writer - Determines the effect of a counter overflow event (COUNTER reaches PERIOD) on the 'line_out' output signals."]
pub type OVERFLOW_MODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, OVERFLOW_MODE_A>;
impl<'a, REG> OVERFLOW_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Set to '1'"]
    #[inline(always)]
    pub fn set(self) -> &'a mut crate::W<REG> {
        self.variant(OVERFLOW_MODE_A::SET)
    }
    #[doc = "Set to '0'"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(OVERFLOW_MODE_A::CLEAR)
    }
    #[doc = "Invert"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut crate::W<REG> {
        self.variant(OVERFLOW_MODE_A::INVERT)
    }
    #[doc = "No Change"]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut crate::W<REG> {
        self.variant(OVERFLOW_MODE_A::NO_CHANGE)
    }
}
#[doc = "Field `UNDERFLOW_MODE` reader - Determines the effect of a counter underflow event (COUNTER reaches '0') on the 'line_out' output signals."]
pub type UNDERFLOW_MODE_R = crate::FieldReader<UNDERFLOW_MODE_A>;
#[doc = "Determines the effect of a counter underflow event (COUNTER reaches '0') on the 'line_out' output signals.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UNDERFLOW_MODE_A {
    #[doc = "0: Set to '1'"]
    SET = 0,
    #[doc = "1: Set to '0'"]
    CLEAR = 1,
    #[doc = "2: Invert"]
    INVERT = 2,
    #[doc = "3: No Change"]
    NO_CHANGE = 3,
}
impl From<UNDERFLOW_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: UNDERFLOW_MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UNDERFLOW_MODE_A {
    type Ux = u8;
}
impl UNDERFLOW_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UNDERFLOW_MODE_A {
        match self.bits {
            0 => UNDERFLOW_MODE_A::SET,
            1 => UNDERFLOW_MODE_A::CLEAR,
            2 => UNDERFLOW_MODE_A::INVERT,
            3 => UNDERFLOW_MODE_A::NO_CHANGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Set to '1'"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == UNDERFLOW_MODE_A::SET
    }
    #[doc = "Set to '0'"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == UNDERFLOW_MODE_A::CLEAR
    }
    #[doc = "Invert"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == UNDERFLOW_MODE_A::INVERT
    }
    #[doc = "No Change"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == UNDERFLOW_MODE_A::NO_CHANGE
    }
}
#[doc = "Field `UNDERFLOW_MODE` writer - Determines the effect of a counter underflow event (COUNTER reaches '0') on the 'line_out' output signals."]
pub type UNDERFLOW_MODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, UNDERFLOW_MODE_A>;
impl<'a, REG> UNDERFLOW_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Set to '1'"]
    #[inline(always)]
    pub fn set(self) -> &'a mut crate::W<REG> {
        self.variant(UNDERFLOW_MODE_A::SET)
    }
    #[doc = "Set to '0'"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(UNDERFLOW_MODE_A::CLEAR)
    }
    #[doc = "Invert"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut crate::W<REG> {
        self.variant(UNDERFLOW_MODE_A::INVERT)
    }
    #[doc = "No Change"]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut crate::W<REG> {
        self.variant(UNDERFLOW_MODE_A::NO_CHANGE)
    }
}
impl R {
    #[doc = "Bits 0:1 - Determines the effect of a compare match event (COUNTER equals CC register) on the 'line_out' output signals. Note that INVERT is especially useful for center aligned pulse width modulation. To generate a duty cycle of 0 percent, the counter CC register should be set to '0'. For a 100 percent duty cycle, the counter CC register should be set to larger than the counter PERIOD register."]
    #[inline(always)]
    pub fn cc_match_mode(&self) -> CC_MATCH_MODE_R {
        CC_MATCH_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Determines the effect of a counter overflow event (COUNTER reaches PERIOD) on the 'line_out' output signals."]
    #[inline(always)]
    pub fn overflow_mode(&self) -> OVERFLOW_MODE_R {
        OVERFLOW_MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Determines the effect of a counter underflow event (COUNTER reaches '0') on the 'line_out' output signals."]
    #[inline(always)]
    pub fn underflow_mode(&self) -> UNDERFLOW_MODE_R {
        UNDERFLOW_MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Determines the effect of a compare match event (COUNTER equals CC register) on the 'line_out' output signals. Note that INVERT is especially useful for center aligned pulse width modulation. To generate a duty cycle of 0 percent, the counter CC register should be set to '0'. For a 100 percent duty cycle, the counter CC register should be set to larger than the counter PERIOD register."]
    #[inline(always)]
    #[must_use]
    pub fn cc_match_mode(&mut self) -> CC_MATCH_MODE_W<TR_CTRL2_SPEC> {
        CC_MATCH_MODE_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Determines the effect of a counter overflow event (COUNTER reaches PERIOD) on the 'line_out' output signals."]
    #[inline(always)]
    #[must_use]
    pub fn overflow_mode(&mut self) -> OVERFLOW_MODE_W<TR_CTRL2_SPEC> {
        OVERFLOW_MODE_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Determines the effect of a counter underflow event (COUNTER reaches '0') on the 'line_out' output signals."]
    #[inline(always)]
    #[must_use]
    pub fn underflow_mode(&mut self) -> UNDERFLOW_MODE_W<TR_CTRL2_SPEC> {
        UNDERFLOW_MODE_W::new(self, 4)
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
#[doc = "Counter trigger control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tr_ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tr_ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TR_CTRL2_SPEC;
impl crate::RegisterSpec for TR_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr_ctrl2::R`](R) reader structure"]
impl crate::Readable for TR_CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tr_ctrl2::W`](W) writer structure"]
impl crate::Writable for TR_CTRL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TR_CTRL2 to value 0x3f"]
impl crate::Resettable for TR_CTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0x3f;
}
