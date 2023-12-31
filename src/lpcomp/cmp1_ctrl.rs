#[doc = "Register `CMP1_CTRL` reader"]
pub type R = crate::R<CMP1_CTRL_SPEC>;
#[doc = "Register `CMP1_CTRL` writer"]
pub type W = crate::W<CMP1_CTRL_SPEC>;
#[doc = "Field `MODE1` reader - Operating mode for the comparator"]
pub type MODE1_R = crate::FieldReader<MODE1_A>;
#[doc = "Operating mode for the comparator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE1_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: Ultra lowpower operating mode (uses less power, &lt; 300nA), must be used for DeepSleep or Hibernate. Only in this mode a local iref will be used."]
    ULP = 1,
    #[doc = "2: Low Power operating mode (uses more power, &lt;10uA @@@ TBD). In this mode the iref from SRSS will be used."]
    LP = 2,
    #[doc = "3: Normal, full speed power operating mode (uses &lt;150uA). In this mode the iref from SRSS will be used."]
    NORMAL = 3,
}
impl From<MODE1_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE1_A {
    type Ux = u8;
}
impl MODE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE1_A {
        match self.bits {
            0 => MODE1_A::OFF,
            1 => MODE1_A::ULP,
            2 => MODE1_A::LP,
            3 => MODE1_A::NORMAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == MODE1_A::OFF
    }
    #[doc = "Ultra lowpower operating mode (uses less power, &lt; 300nA), must be used for DeepSleep or Hibernate. Only in this mode a local iref will be used."]
    #[inline(always)]
    pub fn is_ulp(&self) -> bool {
        *self == MODE1_A::ULP
    }
    #[doc = "Low Power operating mode (uses more power, &lt;10uA @@@ TBD). In this mode the iref from SRSS will be used."]
    #[inline(always)]
    pub fn is_lp(&self) -> bool {
        *self == MODE1_A::LP
    }
    #[doc = "Normal, full speed power operating mode (uses &lt;150uA). In this mode the iref from SRSS will be used."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == MODE1_A::NORMAL
    }
}
#[doc = "Field `MODE1` writer - Operating mode for the comparator"]
pub type MODE1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, MODE1_A>;
impl<'a, REG> MODE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1_A::OFF)
    }
    #[doc = "Ultra lowpower operating mode (uses less power, &lt; 300nA), must be used for DeepSleep or Hibernate. Only in this mode a local iref will be used."]
    #[inline(always)]
    pub fn ulp(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1_A::ULP)
    }
    #[doc = "Low Power operating mode (uses more power, &lt;10uA @@@ TBD). In this mode the iref from SRSS will be used."]
    #[inline(always)]
    pub fn lp(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1_A::LP)
    }
    #[doc = "Normal, full speed power operating mode (uses &lt;150uA). In this mode the iref from SRSS will be used."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1_A::NORMAL)
    }
}
#[doc = "Field `HYST1` reader - Add 30mV hysteresis to the comparator 0= Disable Hysteresis 1= Enable Hysteresis"]
pub type HYST1_R = crate::BitReader;
#[doc = "Field `HYST1` writer - Add 30mV hysteresis to the comparator 0= Disable Hysteresis 1= Enable Hysteresis"]
pub type HYST1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTTYPE1` reader - Sets which edge will trigger an IRQ"]
pub type INTTYPE1_R = crate::FieldReader<INTTYPE1_A>;
#[doc = "Sets which edge will trigger an IRQ\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INTTYPE1_A {
    #[doc = "0: Disabled, no interrupts will be detected"]
    DISABLE = 0,
    #[doc = "1: Rising edge"]
    RISING = 1,
    #[doc = "2: Falling edge"]
    FALLING = 2,
    #[doc = "3: Both rising and falling edges"]
    BOTH = 3,
}
impl From<INTTYPE1_A> for u8 {
    #[inline(always)]
    fn from(variant: INTTYPE1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INTTYPE1_A {
    type Ux = u8;
}
impl INTTYPE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INTTYPE1_A {
        match self.bits {
            0 => INTTYPE1_A::DISABLE,
            1 => INTTYPE1_A::RISING,
            2 => INTTYPE1_A::FALLING,
            3 => INTTYPE1_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Disabled, no interrupts will be detected"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == INTTYPE1_A::DISABLE
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == INTTYPE1_A::RISING
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == INTTYPE1_A::FALLING
    }
    #[doc = "Both rising and falling edges"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == INTTYPE1_A::BOTH
    }
}
#[doc = "Field `INTTYPE1` writer - Sets which edge will trigger an IRQ"]
pub type INTTYPE1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, INTTYPE1_A>;
impl<'a, REG> INTTYPE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled, no interrupts will be detected"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(INTTYPE1_A::DISABLE)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(INTTYPE1_A::RISING)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(INTTYPE1_A::FALLING)
    }
    #[doc = "Both rising and falling edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(INTTYPE1_A::BOTH)
    }
}
#[doc = "Field `DSI_BYPASS1` reader - Asynchronous: bypass comparator output synchronization for DSI output: 0=synchronize (level or pulse), 1=bypass (output async). Note that in DeepSleep mode this bit needs to be set to observe the DSI output on the dedicated pin."]
pub type DSI_BYPASS1_R = crate::BitReader;
#[doc = "Field `DSI_BYPASS1` writer - Asynchronous: bypass comparator output synchronization for DSI output: 0=synchronize (level or pulse), 1=bypass (output async). Note that in DeepSleep mode this bit needs to be set to observe the DSI output on the dedicated pin."]
pub type DSI_BYPASS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSI_LEVEL1` reader - Synchronous comparator DSI (trigger) output : 0=pulse, 1=level"]
pub type DSI_LEVEL1_R = crate::BitReader;
#[doc = "Field `DSI_LEVEL1` writer - Synchronous comparator DSI (trigger) output : 0=pulse, 1=level"]
pub type DSI_LEVEL1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Operating mode for the comparator"]
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 5 - Add 30mV hysteresis to the comparator 0= Disable Hysteresis 1= Enable Hysteresis"]
    #[inline(always)]
    pub fn hyst1(&self) -> HYST1_R {
        HYST1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Sets which edge will trigger an IRQ"]
    #[inline(always)]
    pub fn inttype1(&self) -> INTTYPE1_R {
        INTTYPE1_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 10 - Asynchronous: bypass comparator output synchronization for DSI output: 0=synchronize (level or pulse), 1=bypass (output async). Note that in DeepSleep mode this bit needs to be set to observe the DSI output on the dedicated pin."]
    #[inline(always)]
    pub fn dsi_bypass1(&self) -> DSI_BYPASS1_R {
        DSI_BYPASS1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Synchronous comparator DSI (trigger) output : 0=pulse, 1=level"]
    #[inline(always)]
    pub fn dsi_level1(&self) -> DSI_LEVEL1_R {
        DSI_LEVEL1_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Operating mode for the comparator"]
    #[inline(always)]
    #[must_use]
    pub fn mode1(&mut self) -> MODE1_W<CMP1_CTRL_SPEC> {
        MODE1_W::new(self, 0)
    }
    #[doc = "Bit 5 - Add 30mV hysteresis to the comparator 0= Disable Hysteresis 1= Enable Hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn hyst1(&mut self) -> HYST1_W<CMP1_CTRL_SPEC> {
        HYST1_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Sets which edge will trigger an IRQ"]
    #[inline(always)]
    #[must_use]
    pub fn inttype1(&mut self) -> INTTYPE1_W<CMP1_CTRL_SPEC> {
        INTTYPE1_W::new(self, 6)
    }
    #[doc = "Bit 10 - Asynchronous: bypass comparator output synchronization for DSI output: 0=synchronize (level or pulse), 1=bypass (output async). Note that in DeepSleep mode this bit needs to be set to observe the DSI output on the dedicated pin."]
    #[inline(always)]
    #[must_use]
    pub fn dsi_bypass1(&mut self) -> DSI_BYPASS1_W<CMP1_CTRL_SPEC> {
        DSI_BYPASS1_W::new(self, 10)
    }
    #[doc = "Bit 11 - Synchronous comparator DSI (trigger) output : 0=pulse, 1=level"]
    #[inline(always)]
    #[must_use]
    pub fn dsi_level1(&mut self) -> DSI_LEVEL1_W<CMP1_CTRL_SPEC> {
        DSI_LEVEL1_W::new(self, 11)
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
#[doc = "Comparator 1 control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp1_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp1_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMP1_CTRL_SPEC;
impl crate::RegisterSpec for CMP1_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp1_ctrl::R`](R) reader structure"]
impl crate::Readable for CMP1_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmp1_ctrl::W`](W) writer structure"]
impl crate::Writable for CMP1_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMP1_CTRL to value 0"]
impl crate::Resettable for CMP1_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
