#[doc = "Register `MCWDT_CONFIG` reader"]
pub type R = crate::R<MCWDT_CONFIG_SPEC>;
#[doc = "Register `MCWDT_CONFIG` writer"]
pub type W = crate::W<MCWDT_CONFIG_SPEC>;
#[doc = "Field `WDT_MODE0` reader - Watchdog Counter Action on Match. Action is taken on the next increment after the values match (WDT_CTR0=WDT_MATCH0)."]
pub type WDT_MODE0_R = crate::FieldReader<WDT_MODE0_A>;
#[doc = "Watchdog Counter Action on Match. Action is taken on the next increment after the values match (WDT_CTR0=WDT_MATCH0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDT_MODE0_A {
    #[doc = "0: Do nothing"]
    NOTHING = 0,
    #[doc = "1: Assert WDT_INTx"]
    INT = 1,
    #[doc = "2: Assert WDT Reset"]
    RESET = 2,
    #[doc = "3: Assert WDT_INTx, assert WDT Reset after 3rd unhandled interrupt"]
    INT_THEN_RESET = 3,
}
impl From<WDT_MODE0_A> for u8 {
    #[inline(always)]
    fn from(variant: WDT_MODE0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WDT_MODE0_A {
    type Ux = u8;
}
impl WDT_MODE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WDT_MODE0_A {
        match self.bits {
            0 => WDT_MODE0_A::NOTHING,
            1 => WDT_MODE0_A::INT,
            2 => WDT_MODE0_A::RESET,
            3 => WDT_MODE0_A::INT_THEN_RESET,
            _ => unreachable!(),
        }
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn is_nothing(&self) -> bool {
        *self == WDT_MODE0_A::NOTHING
    }
    #[doc = "Assert WDT_INTx"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == WDT_MODE0_A::INT
    }
    #[doc = "Assert WDT Reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == WDT_MODE0_A::RESET
    }
    #[doc = "Assert WDT_INTx, assert WDT Reset after 3rd unhandled interrupt"]
    #[inline(always)]
    pub fn is_int_then_reset(&self) -> bool {
        *self == WDT_MODE0_A::INT_THEN_RESET
    }
}
#[doc = "Field `WDT_MODE0` writer - Watchdog Counter Action on Match. Action is taken on the next increment after the values match (WDT_CTR0=WDT_MATCH0)."]
pub type WDT_MODE0_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, WDT_MODE0_A>;
impl<'a, REG> WDT_MODE0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn nothing(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_MODE0_A::NOTHING)
    }
    #[doc = "Assert WDT_INTx"]
    #[inline(always)]
    pub fn int(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_MODE0_A::INT)
    }
    #[doc = "Assert WDT Reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_MODE0_A::RESET)
    }
    #[doc = "Assert WDT_INTx, assert WDT Reset after 3rd unhandled interrupt"]
    #[inline(always)]
    pub fn int_then_reset(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_MODE0_A::INT_THEN_RESET)
    }
}
#[doc = "Field `WDT_CLEAR0` reader - Clear Watchdog Counter when WDT_CTR0=WDT_MATCH0. In other words WDT_CTR0 divides LFCLK by (WDT_MATCH0+1). 0: Free running counter 1: Clear on match. In this mode, the minimum legal setting of WDT_MATCH0 is 1."]
pub type WDT_CLEAR0_R = crate::BitReader;
#[doc = "Field `WDT_CLEAR0` writer - Clear Watchdog Counter when WDT_CTR0=WDT_MATCH0. In other words WDT_CTR0 divides LFCLK by (WDT_MATCH0+1). 0: Free running counter 1: Clear on match. In this mode, the minimum legal setting of WDT_MATCH0 is 1."]
pub type WDT_CLEAR0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_CASCADE0_1` reader - Cascade Watchdog Counters 0,1. Counter 1 increments the cycle after WDT_CTR0=WDT_MATCH0. 0: Independent counters 1: Cascaded counters"]
pub type WDT_CASCADE0_1_R = crate::BitReader;
#[doc = "Field `WDT_CASCADE0_1` writer - Cascade Watchdog Counters 0,1. Counter 1 increments the cycle after WDT_CTR0=WDT_MATCH0. 0: Independent counters 1: Cascaded counters"]
pub type WDT_CASCADE0_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_MODE1` reader - Watchdog Counter Action on Match. Action is taken on the next increment after the values match (WDT_CTR1=WDT_MATCH1)."]
pub type WDT_MODE1_R = crate::FieldReader<WDT_MODE1_A>;
#[doc = "Watchdog Counter Action on Match. Action is taken on the next increment after the values match (WDT_CTR1=WDT_MATCH1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDT_MODE1_A {
    #[doc = "0: Do nothing"]
    NOTHING = 0,
    #[doc = "1: Assert WDT_INTx"]
    INT = 1,
    #[doc = "2: Assert WDT Reset"]
    RESET = 2,
    #[doc = "3: Assert WDT_INTx, assert WDT Reset after 3rd unhandled interrupt"]
    INT_THEN_RESET = 3,
}
impl From<WDT_MODE1_A> for u8 {
    #[inline(always)]
    fn from(variant: WDT_MODE1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WDT_MODE1_A {
    type Ux = u8;
}
impl WDT_MODE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WDT_MODE1_A {
        match self.bits {
            0 => WDT_MODE1_A::NOTHING,
            1 => WDT_MODE1_A::INT,
            2 => WDT_MODE1_A::RESET,
            3 => WDT_MODE1_A::INT_THEN_RESET,
            _ => unreachable!(),
        }
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn is_nothing(&self) -> bool {
        *self == WDT_MODE1_A::NOTHING
    }
    #[doc = "Assert WDT_INTx"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == WDT_MODE1_A::INT
    }
    #[doc = "Assert WDT Reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == WDT_MODE1_A::RESET
    }
    #[doc = "Assert WDT_INTx, assert WDT Reset after 3rd unhandled interrupt"]
    #[inline(always)]
    pub fn is_int_then_reset(&self) -> bool {
        *self == WDT_MODE1_A::INT_THEN_RESET
    }
}
#[doc = "Field `WDT_MODE1` writer - Watchdog Counter Action on Match. Action is taken on the next increment after the values match (WDT_CTR1=WDT_MATCH1)."]
pub type WDT_MODE1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, WDT_MODE1_A>;
impl<'a, REG> WDT_MODE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn nothing(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_MODE1_A::NOTHING)
    }
    #[doc = "Assert WDT_INTx"]
    #[inline(always)]
    pub fn int(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_MODE1_A::INT)
    }
    #[doc = "Assert WDT Reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_MODE1_A::RESET)
    }
    #[doc = "Assert WDT_INTx, assert WDT Reset after 3rd unhandled interrupt"]
    #[inline(always)]
    pub fn int_then_reset(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_MODE1_A::INT_THEN_RESET)
    }
}
#[doc = "Field `WDT_CLEAR1` reader - Clear Watchdog Counter when WDT_CTR1==WDT_MATCH1. In other words WDT_CTR1 divides LFCLK by (WDT_MATCH1+1). 0: Free running counter 1: Clear on match. In this mode, the minimum legal setting of WDT_MATCH1 is 1."]
pub type WDT_CLEAR1_R = crate::BitReader;
#[doc = "Field `WDT_CLEAR1` writer - Clear Watchdog Counter when WDT_CTR1==WDT_MATCH1. In other words WDT_CTR1 divides LFCLK by (WDT_MATCH1+1). 0: Free running counter 1: Clear on match. In this mode, the minimum legal setting of WDT_MATCH1 is 1."]
pub type WDT_CLEAR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_CASCADE1_2` reader - Cascade Watchdog Counters 1,2. Counter 2 increments the cycle after WDT_CTR1=WDT_MATCH1. It is allowed to cascade all three WDT counters. 0: Independent counters 1: Cascaded counters. When cascading all three counters, WDT_CLEAR1 must be 1."]
pub type WDT_CASCADE1_2_R = crate::BitReader;
#[doc = "Field `WDT_CASCADE1_2` writer - Cascade Watchdog Counters 1,2. Counter 2 increments the cycle after WDT_CTR1=WDT_MATCH1. It is allowed to cascade all three WDT counters. 0: Independent counters 1: Cascaded counters. When cascading all three counters, WDT_CLEAR1 must be 1."]
pub type WDT_CASCADE1_2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_MODE2` reader - Watchdog Counter 2 Mode."]
pub type WDT_MODE2_R = crate::BitReader<WDT_MODE2_A>;
#[doc = "Watchdog Counter 2 Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDT_MODE2_A {
    #[doc = "0: Free running counter with no interrupt requests"]
    NOTHING = 0,
    #[doc = "1: Free running counter with interrupt request that occurs one LFCLK cycle after the specified bit in CTR2 toggles (see WDT_BITS2)."]
    INT = 1,
}
impl From<WDT_MODE2_A> for bool {
    #[inline(always)]
    fn from(variant: WDT_MODE2_A) -> Self {
        variant as u8 != 0
    }
}
impl WDT_MODE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WDT_MODE2_A {
        match self.bits {
            false => WDT_MODE2_A::NOTHING,
            true => WDT_MODE2_A::INT,
        }
    }
    #[doc = "Free running counter with no interrupt requests"]
    #[inline(always)]
    pub fn is_nothing(&self) -> bool {
        *self == WDT_MODE2_A::NOTHING
    }
    #[doc = "Free running counter with interrupt request that occurs one LFCLK cycle after the specified bit in CTR2 toggles (see WDT_BITS2)."]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == WDT_MODE2_A::INT
    }
}
#[doc = "Field `WDT_MODE2` writer - Watchdog Counter 2 Mode."]
pub type WDT_MODE2_W<'a, REG> = crate::BitWriter<'a, REG, WDT_MODE2_A>;
impl<'a, REG> WDT_MODE2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Free running counter with no interrupt requests"]
    #[inline(always)]
    pub fn nothing(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_MODE2_A::NOTHING)
    }
    #[doc = "Free running counter with interrupt request that occurs one LFCLK cycle after the specified bit in CTR2 toggles (see WDT_BITS2)."]
    #[inline(always)]
    pub fn int(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_MODE2_A::INT)
    }
}
#[doc = "Field `WDT_BITS2` reader - Bit to observe for WDT_INT2: 0: Assert after bit0 of WDT_CTR2 toggles (one int every tick) ... 31: Assert after bit31 of WDT_CTR2 toggles (one int every 2^31 ticks)"]
pub type WDT_BITS2_R = crate::FieldReader;
#[doc = "Field `WDT_BITS2` writer - Bit to observe for WDT_INT2: 0: Assert after bit0 of WDT_CTR2 toggles (one int every tick) ... 31: Assert after bit31 of WDT_CTR2 toggles (one int every 2^31 ticks)"]
pub type WDT_BITS2_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:1 - Watchdog Counter Action on Match. Action is taken on the next increment after the values match (WDT_CTR0=WDT_MATCH0)."]
    #[inline(always)]
    pub fn wdt_mode0(&self) -> WDT_MODE0_R {
        WDT_MODE0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Clear Watchdog Counter when WDT_CTR0=WDT_MATCH0. In other words WDT_CTR0 divides LFCLK by (WDT_MATCH0+1). 0: Free running counter 1: Clear on match. In this mode, the minimum legal setting of WDT_MATCH0 is 1."]
    #[inline(always)]
    pub fn wdt_clear0(&self) -> WDT_CLEAR0_R {
        WDT_CLEAR0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Cascade Watchdog Counters 0,1. Counter 1 increments the cycle after WDT_CTR0=WDT_MATCH0. 0: Independent counters 1: Cascaded counters"]
    #[inline(always)]
    pub fn wdt_cascade0_1(&self) -> WDT_CASCADE0_1_R {
        WDT_CASCADE0_1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Watchdog Counter Action on Match. Action is taken on the next increment after the values match (WDT_CTR1=WDT_MATCH1)."]
    #[inline(always)]
    pub fn wdt_mode1(&self) -> WDT_MODE1_R {
        WDT_MODE1_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Clear Watchdog Counter when WDT_CTR1==WDT_MATCH1. In other words WDT_CTR1 divides LFCLK by (WDT_MATCH1+1). 0: Free running counter 1: Clear on match. In this mode, the minimum legal setting of WDT_MATCH1 is 1."]
    #[inline(always)]
    pub fn wdt_clear1(&self) -> WDT_CLEAR1_R {
        WDT_CLEAR1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Cascade Watchdog Counters 1,2. Counter 2 increments the cycle after WDT_CTR1=WDT_MATCH1. It is allowed to cascade all three WDT counters. 0: Independent counters 1: Cascaded counters. When cascading all three counters, WDT_CLEAR1 must be 1."]
    #[inline(always)]
    pub fn wdt_cascade1_2(&self) -> WDT_CASCADE1_2_R {
        WDT_CASCADE1_2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Watchdog Counter 2 Mode."]
    #[inline(always)]
    pub fn wdt_mode2(&self) -> WDT_MODE2_R {
        WDT_MODE2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:28 - Bit to observe for WDT_INT2: 0: Assert after bit0 of WDT_CTR2 toggles (one int every tick) ... 31: Assert after bit31 of WDT_CTR2 toggles (one int every 2^31 ticks)"]
    #[inline(always)]
    pub fn wdt_bits2(&self) -> WDT_BITS2_R {
        WDT_BITS2_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Watchdog Counter Action on Match. Action is taken on the next increment after the values match (WDT_CTR0=WDT_MATCH0)."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_mode0(&mut self) -> WDT_MODE0_W<MCWDT_CONFIG_SPEC> {
        WDT_MODE0_W::new(self, 0)
    }
    #[doc = "Bit 2 - Clear Watchdog Counter when WDT_CTR0=WDT_MATCH0. In other words WDT_CTR0 divides LFCLK by (WDT_MATCH0+1). 0: Free running counter 1: Clear on match. In this mode, the minimum legal setting of WDT_MATCH0 is 1."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_clear0(&mut self) -> WDT_CLEAR0_W<MCWDT_CONFIG_SPEC> {
        WDT_CLEAR0_W::new(self, 2)
    }
    #[doc = "Bit 3 - Cascade Watchdog Counters 0,1. Counter 1 increments the cycle after WDT_CTR0=WDT_MATCH0. 0: Independent counters 1: Cascaded counters"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_cascade0_1(&mut self) -> WDT_CASCADE0_1_W<MCWDT_CONFIG_SPEC> {
        WDT_CASCADE0_1_W::new(self, 3)
    }
    #[doc = "Bits 8:9 - Watchdog Counter Action on Match. Action is taken on the next increment after the values match (WDT_CTR1=WDT_MATCH1)."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_mode1(&mut self) -> WDT_MODE1_W<MCWDT_CONFIG_SPEC> {
        WDT_MODE1_W::new(self, 8)
    }
    #[doc = "Bit 10 - Clear Watchdog Counter when WDT_CTR1==WDT_MATCH1. In other words WDT_CTR1 divides LFCLK by (WDT_MATCH1+1). 0: Free running counter 1: Clear on match. In this mode, the minimum legal setting of WDT_MATCH1 is 1."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_clear1(&mut self) -> WDT_CLEAR1_W<MCWDT_CONFIG_SPEC> {
        WDT_CLEAR1_W::new(self, 10)
    }
    #[doc = "Bit 11 - Cascade Watchdog Counters 1,2. Counter 2 increments the cycle after WDT_CTR1=WDT_MATCH1. It is allowed to cascade all three WDT counters. 0: Independent counters 1: Cascaded counters. When cascading all three counters, WDT_CLEAR1 must be 1."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_cascade1_2(&mut self) -> WDT_CASCADE1_2_W<MCWDT_CONFIG_SPEC> {
        WDT_CASCADE1_2_W::new(self, 11)
    }
    #[doc = "Bit 16 - Watchdog Counter 2 Mode."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_mode2(&mut self) -> WDT_MODE2_W<MCWDT_CONFIG_SPEC> {
        WDT_MODE2_W::new(self, 16)
    }
    #[doc = "Bits 24:28 - Bit to observe for WDT_INT2: 0: Assert after bit0 of WDT_CTR2 toggles (one int every tick) ... 31: Assert after bit31 of WDT_CTR2 toggles (one int every 2^31 ticks)"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_bits2(&mut self) -> WDT_BITS2_W<MCWDT_CONFIG_SPEC> {
        WDT_BITS2_W::new(self, 24)
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
#[doc = "Multi-Counter Watchdog Counter Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcwdt_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcwdt_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCWDT_CONFIG_SPEC;
impl crate::RegisterSpec for MCWDT_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcwdt_config::R`](R) reader structure"]
impl crate::Readable for MCWDT_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcwdt_config::W`](W) writer structure"]
impl crate::Writable for MCWDT_CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCWDT_CONFIG to value 0"]
impl crate::Resettable for MCWDT_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
