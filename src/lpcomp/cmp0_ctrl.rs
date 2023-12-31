#[doc = "Register `CMP0_CTRL` reader"]
pub type R = crate::R<CMP0_CTRL_SPEC>;
#[doc = "Register `CMP0_CTRL` writer"]
pub type W = crate::W<CMP0_CTRL_SPEC>;
#[doc = "Field `MODE0` reader - Operating mode for the comparator"]
pub type MODE0_R = crate::FieldReader<MODE0_A>;
#[doc = "Operating mode for the comparator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE0_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: Ultra lowpower operating mode (uses less power, &lt; 300nA), must be used for DeepSleep or Hibernate. Only in this mode a local iref will be used."]
    ULP = 1,
    #[doc = "2: Low Power operating mode (uses more power, &lt;10uA @@@ TBD). In this mode the iref from SRSS will be used."]
    LP = 2,
    #[doc = "3: Normal, full speed power operating mode (uses &lt;150uA). In this mode the iref from SRSS will be used."]
    NORMAL = 3,
}
impl From<MODE0_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE0_A {
    type Ux = u8;
}
impl MODE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE0_A {
        match self.bits {
            0 => MODE0_A::OFF,
            1 => MODE0_A::ULP,
            2 => MODE0_A::LP,
            3 => MODE0_A::NORMAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == MODE0_A::OFF
    }
    #[doc = "Ultra lowpower operating mode (uses less power, &lt; 300nA), must be used for DeepSleep or Hibernate. Only in this mode a local iref will be used."]
    #[inline(always)]
    pub fn is_ulp(&self) -> bool {
        *self == MODE0_A::ULP
    }
    #[doc = "Low Power operating mode (uses more power, &lt;10uA @@@ TBD). In this mode the iref from SRSS will be used."]
    #[inline(always)]
    pub fn is_lp(&self) -> bool {
        *self == MODE0_A::LP
    }
    #[doc = "Normal, full speed power operating mode (uses &lt;150uA). In this mode the iref from SRSS will be used."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == MODE0_A::NORMAL
    }
}
#[doc = "Field `MODE0` writer - Operating mode for the comparator"]
pub type MODE0_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, MODE0_A>;
impl<'a, REG> MODE0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0_A::OFF)
    }
    #[doc = "Ultra lowpower operating mode (uses less power, &lt; 300nA), must be used for DeepSleep or Hibernate. Only in this mode a local iref will be used."]
    #[inline(always)]
    pub fn ulp(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0_A::ULP)
    }
    #[doc = "Low Power operating mode (uses more power, &lt;10uA @@@ TBD). In this mode the iref from SRSS will be used."]
    #[inline(always)]
    pub fn lp(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0_A::LP)
    }
    #[doc = "Normal, full speed power operating mode (uses &lt;150uA). In this mode the iref from SRSS will be used."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0_A::NORMAL)
    }
}
#[doc = "Field `HYST0` reader - Add 30mV hysteresis to the comparator 0= Disable Hysteresis 1= Enable Hysteresis"]
pub type HYST0_R = crate::BitReader;
#[doc = "Field `HYST0` writer - Add 30mV hysteresis to the comparator 0= Disable Hysteresis 1= Enable Hysteresis"]
pub type HYST0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTTYPE0` reader - Sets which edge will trigger an IRQ"]
pub type INTTYPE0_R = crate::FieldReader<INTTYPE0_A>;
#[doc = "Sets which edge will trigger an IRQ\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INTTYPE0_A {
    #[doc = "0: Disabled, no interrupts will be detected"]
    DISABLE = 0,
    #[doc = "1: Rising edge"]
    RISING = 1,
    #[doc = "2: Falling edge"]
    FALLING = 2,
    #[doc = "3: Both rising and falling edges"]
    BOTH = 3,
}
impl From<INTTYPE0_A> for u8 {
    #[inline(always)]
    fn from(variant: INTTYPE0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INTTYPE0_A {
    type Ux = u8;
}
impl INTTYPE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INTTYPE0_A {
        match self.bits {
            0 => INTTYPE0_A::DISABLE,
            1 => INTTYPE0_A::RISING,
            2 => INTTYPE0_A::FALLING,
            3 => INTTYPE0_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Disabled, no interrupts will be detected"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == INTTYPE0_A::DISABLE
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == INTTYPE0_A::RISING
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == INTTYPE0_A::FALLING
    }
    #[doc = "Both rising and falling edges"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == INTTYPE0_A::BOTH
    }
}
#[doc = "Field `INTTYPE0` writer - Sets which edge will trigger an IRQ"]
pub type INTTYPE0_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, INTTYPE0_A>;
impl<'a, REG> INTTYPE0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled, no interrupts will be detected"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(INTTYPE0_A::DISABLE)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(INTTYPE0_A::RISING)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(INTTYPE0_A::FALLING)
    }
    #[doc = "Both rising and falling edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(INTTYPE0_A::BOTH)
    }
}
#[doc = "Field `DSI_BYPASS0` reader - Asynchronous: bypass comparator output synchronization for DSI output: 0=synchronize (level or pulse), 1=bypass (output async). Note that in DeepSleep mode this bit needs to be set to observe the DSI output on the dedicated pin."]
pub type DSI_BYPASS0_R = crate::BitReader;
#[doc = "Field `DSI_BYPASS0` writer - Asynchronous: bypass comparator output synchronization for DSI output: 0=synchronize (level or pulse), 1=bypass (output async). Note that in DeepSleep mode this bit needs to be set to observe the DSI output on the dedicated pin."]
pub type DSI_BYPASS0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSI_LEVEL0` reader - Synchronous comparator DSI (trigger) output : 0=pulse, 1=level"]
pub type DSI_LEVEL0_R = crate::BitReader;
#[doc = "Field `DSI_LEVEL0` writer - Synchronous comparator DSI (trigger) output : 0=pulse, 1=level"]
pub type DSI_LEVEL0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Operating mode for the comparator"]
    #[inline(always)]
    pub fn mode0(&self) -> MODE0_R {
        MODE0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 5 - Add 30mV hysteresis to the comparator 0= Disable Hysteresis 1= Enable Hysteresis"]
    #[inline(always)]
    pub fn hyst0(&self) -> HYST0_R {
        HYST0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Sets which edge will trigger an IRQ"]
    #[inline(always)]
    pub fn inttype0(&self) -> INTTYPE0_R {
        INTTYPE0_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 10 - Asynchronous: bypass comparator output synchronization for DSI output: 0=synchronize (level or pulse), 1=bypass (output async). Note that in DeepSleep mode this bit needs to be set to observe the DSI output on the dedicated pin."]
    #[inline(always)]
    pub fn dsi_bypass0(&self) -> DSI_BYPASS0_R {
        DSI_BYPASS0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Synchronous comparator DSI (trigger) output : 0=pulse, 1=level"]
    #[inline(always)]
    pub fn dsi_level0(&self) -> DSI_LEVEL0_R {
        DSI_LEVEL0_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Operating mode for the comparator"]
    #[inline(always)]
    #[must_use]
    pub fn mode0(&mut self) -> MODE0_W<CMP0_CTRL_SPEC> {
        MODE0_W::new(self, 0)
    }
    #[doc = "Bit 5 - Add 30mV hysteresis to the comparator 0= Disable Hysteresis 1= Enable Hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn hyst0(&mut self) -> HYST0_W<CMP0_CTRL_SPEC> {
        HYST0_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Sets which edge will trigger an IRQ"]
    #[inline(always)]
    #[must_use]
    pub fn inttype0(&mut self) -> INTTYPE0_W<CMP0_CTRL_SPEC> {
        INTTYPE0_W::new(self, 6)
    }
    #[doc = "Bit 10 - Asynchronous: bypass comparator output synchronization for DSI output: 0=synchronize (level or pulse), 1=bypass (output async). Note that in DeepSleep mode this bit needs to be set to observe the DSI output on the dedicated pin."]
    #[inline(always)]
    #[must_use]
    pub fn dsi_bypass0(&mut self) -> DSI_BYPASS0_W<CMP0_CTRL_SPEC> {
        DSI_BYPASS0_W::new(self, 10)
    }
    #[doc = "Bit 11 - Synchronous comparator DSI (trigger) output : 0=pulse, 1=level"]
    #[inline(always)]
    #[must_use]
    pub fn dsi_level0(&mut self) -> DSI_LEVEL0_W<CMP0_CTRL_SPEC> {
        DSI_LEVEL0_W::new(self, 11)
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
#[doc = "Comparator 0 control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp0_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp0_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMP0_CTRL_SPEC;
impl crate::RegisterSpec for CMP0_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp0_ctrl::R`](R) reader structure"]
impl crate::Readable for CMP0_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmp0_ctrl::W`](W) writer structure"]
impl crate::Writable for CMP0_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMP0_CTRL to value 0"]
impl crate::Resettable for CMP0_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
