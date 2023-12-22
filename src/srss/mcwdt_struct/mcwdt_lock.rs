#[doc = "Register `MCWDT_LOCK` reader"]
pub type R = crate::R<MCWDT_LOCK_SPEC>;
#[doc = "Register `MCWDT_LOCK` writer"]
pub type W = crate::W<MCWDT_LOCK_SPEC>;
#[doc = "Field `MCWDT_LOCK` reader - Prohibits writing control and configuration registers related to this MCWDT when not equal 0 (as specified in the other register descriptions). Requires at least two different writes to unlock. Note that this field is 2 bits to force multiple writes only. Each MCWDT has a separate local lock. LFCLK settings are locked by the global WDT_LOCK register, and this register has no effect on that."]
pub type MCWDT_LOCK_R = crate::FieldReader<MCWDT_LOCK_A>;
#[doc = "Prohibits writing control and configuration registers related to this MCWDT when not equal 0 (as specified in the other register descriptions). Requires at least two different writes to unlock. Note that this field is 2 bits to force multiple writes only. Each MCWDT has a separate local lock. LFCLK settings are locked by the global WDT_LOCK register, and this register has no effect on that.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCWDT_LOCK_A {
    #[doc = "0: No effect"]
    NO_CHG = 0,
    #[doc = "1: Clears bit 0"]
    CLR0 = 1,
    #[doc = "2: Clears bit 1"]
    CLR1 = 2,
    #[doc = "3: Sets both bits 0 and 1"]
    SET01 = 3,
}
impl From<MCWDT_LOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: MCWDT_LOCK_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MCWDT_LOCK_A {
    type Ux = u8;
}
impl MCWDT_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MCWDT_LOCK_A {
        match self.bits {
            0 => MCWDT_LOCK_A::NO_CHG,
            1 => MCWDT_LOCK_A::CLR0,
            2 => MCWDT_LOCK_A::CLR1,
            3 => MCWDT_LOCK_A::SET01,
            _ => unreachable!(),
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_chg(&self) -> bool {
        *self == MCWDT_LOCK_A::NO_CHG
    }
    #[doc = "Clears bit 0"]
    #[inline(always)]
    pub fn is_clr0(&self) -> bool {
        *self == MCWDT_LOCK_A::CLR0
    }
    #[doc = "Clears bit 1"]
    #[inline(always)]
    pub fn is_clr1(&self) -> bool {
        *self == MCWDT_LOCK_A::CLR1
    }
    #[doc = "Sets both bits 0 and 1"]
    #[inline(always)]
    pub fn is_set01(&self) -> bool {
        *self == MCWDT_LOCK_A::SET01
    }
}
#[doc = "Field `MCWDT_LOCK` writer - Prohibits writing control and configuration registers related to this MCWDT when not equal 0 (as specified in the other register descriptions). Requires at least two different writes to unlock. Note that this field is 2 bits to force multiple writes only. Each MCWDT has a separate local lock. LFCLK settings are locked by the global WDT_LOCK register, and this register has no effect on that."]
pub type MCWDT_LOCK_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, MCWDT_LOCK_A>;
impl<'a, REG> MCWDT_LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_chg(self) -> &'a mut crate::W<REG> {
        self.variant(MCWDT_LOCK_A::NO_CHG)
    }
    #[doc = "Clears bit 0"]
    #[inline(always)]
    pub fn clr0(self) -> &'a mut crate::W<REG> {
        self.variant(MCWDT_LOCK_A::CLR0)
    }
    #[doc = "Clears bit 1"]
    #[inline(always)]
    pub fn clr1(self) -> &'a mut crate::W<REG> {
        self.variant(MCWDT_LOCK_A::CLR1)
    }
    #[doc = "Sets both bits 0 and 1"]
    #[inline(always)]
    pub fn set01(self) -> &'a mut crate::W<REG> {
        self.variant(MCWDT_LOCK_A::SET01)
    }
}
impl R {
    #[doc = "Bits 30:31 - Prohibits writing control and configuration registers related to this MCWDT when not equal 0 (as specified in the other register descriptions). Requires at least two different writes to unlock. Note that this field is 2 bits to force multiple writes only. Each MCWDT has a separate local lock. LFCLK settings are locked by the global WDT_LOCK register, and this register has no effect on that."]
    #[inline(always)]
    pub fn mcwdt_lock(&self) -> MCWDT_LOCK_R {
        MCWDT_LOCK_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Prohibits writing control and configuration registers related to this MCWDT when not equal 0 (as specified in the other register descriptions). Requires at least two different writes to unlock. Note that this field is 2 bits to force multiple writes only. Each MCWDT has a separate local lock. LFCLK settings are locked by the global WDT_LOCK register, and this register has no effect on that."]
    #[inline(always)]
    #[must_use]
    pub fn mcwdt_lock(&mut self) -> MCWDT_LOCK_W<MCWDT_LOCK_SPEC> {
        MCWDT_LOCK_W::new(self, 30)
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
#[doc = "Multi-Counter Watchdog Counter Lock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcwdt_lock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcwdt_lock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCWDT_LOCK_SPEC;
impl crate::RegisterSpec for MCWDT_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcwdt_lock::R`](R) reader structure"]
impl crate::Readable for MCWDT_LOCK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcwdt_lock::W`](W) writer structure"]
impl crate::Writable for MCWDT_LOCK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCWDT_LOCK to value 0"]
impl crate::Resettable for MCWDT_LOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
