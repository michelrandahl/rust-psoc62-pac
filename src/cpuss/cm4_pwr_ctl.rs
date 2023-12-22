#[doc = "Register `CM4_PWR_CTL` reader"]
pub type R = crate::R<CM4_PWR_CTL_SPEC>;
#[doc = "Register `CM4_PWR_CTL` writer"]
pub type W = crate::W<CM4_PWR_CTL_SPEC>;
#[doc = "Field `PWR_MODE` reader - Power mode."]
pub type PWR_MODE_R = crate::FieldReader<PWR_MODE_A>;
#[doc = "Power mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWR_MODE_A {
    #[doc = "0: Switch CM4 off Power off, clock off, isolate, reset and no retain."]
    OFF = 0,
    #[doc = "1: Reset CM4 Clock off, no isolated, no retain and reset. Note: The CM4 CPU has a AIRCR.SYSRESETREQ register field that allows the CM4 to reset the complete device (RESET only resets the CM4), resulting in a warm boot."]
    RESET = 1,
    #[doc = "2: Put CM4 in Retained mode This can only become effective if CM4 is in SleepDeep mode. Check PWR_DONE flag to see if CM4 RETAINED state has been reached. Power off, clock off, isolate, no reset and retain."]
    RETAINED = 2,
    #[doc = "3: Switch CM4 on. Power on, clock on, no isolate, no reset and no retain."]
    ENABLED = 3,
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
    pub const fn variant(&self) -> PWR_MODE_A {
        match self.bits {
            0 => PWR_MODE_A::OFF,
            1 => PWR_MODE_A::RESET,
            2 => PWR_MODE_A::RETAINED,
            3 => PWR_MODE_A::ENABLED,
            _ => unreachable!(),
        }
    }
    #[doc = "Switch CM4 off Power off, clock off, isolate, reset and no retain."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == PWR_MODE_A::OFF
    }
    #[doc = "Reset CM4 Clock off, no isolated, no retain and reset. Note: The CM4 CPU has a AIRCR.SYSRESETREQ register field that allows the CM4 to reset the complete device (RESET only resets the CM4), resulting in a warm boot."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == PWR_MODE_A::RESET
    }
    #[doc = "Put CM4 in Retained mode This can only become effective if CM4 is in SleepDeep mode. Check PWR_DONE flag to see if CM4 RETAINED state has been reached. Power off, clock off, isolate, no reset and retain."]
    #[inline(always)]
    pub fn is_retained(&self) -> bool {
        *self == PWR_MODE_A::RETAINED
    }
    #[doc = "Switch CM4 on. Power on, clock on, no isolate, no reset and no retain."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PWR_MODE_A::ENABLED
    }
}
#[doc = "Field `PWR_MODE` writer - Power mode."]
pub type PWR_MODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PWR_MODE_A>;
impl<'a, REG> PWR_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Switch CM4 off Power off, clock off, isolate, reset and no retain."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(PWR_MODE_A::OFF)
    }
    #[doc = "Reset CM4 Clock off, no isolated, no retain and reset. Note: The CM4 CPU has a AIRCR.SYSRESETREQ register field that allows the CM4 to reset the complete device (RESET only resets the CM4), resulting in a warm boot."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(PWR_MODE_A::RESET)
    }
    #[doc = "Put CM4 in Retained mode This can only become effective if CM4 is in SleepDeep mode. Check PWR_DONE flag to see if CM4 RETAINED state has been reached. Power off, clock off, isolate, no reset and retain."]
    #[inline(always)]
    pub fn retained(self) -> &'a mut crate::W<REG> {
        self.variant(PWR_MODE_A::RETAINED)
    }
    #[doc = "Switch CM4 on. Power on, clock on, no isolate, no reset and no retain."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWR_MODE_A::ENABLED)
    }
}
#[doc = "Field `VECTKEYSTAT` reader - Register key (to prevent accidental writes). - Should be written with a 0x05fa key value for the write to take effect. - Always reads as 0xfa05."]
pub type VECTKEYSTAT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:1 - Power mode."]
    #[inline(always)]
    pub fn pwr_mode(&self) -> PWR_MODE_R {
        PWR_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:31 - Register key (to prevent accidental writes). - Should be written with a 0x05fa key value for the write to take effect. - Always reads as 0xfa05."]
    #[inline(always)]
    pub fn vectkeystat(&self) -> VECTKEYSTAT_R {
        VECTKEYSTAT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - Power mode."]
    #[inline(always)]
    #[must_use]
    pub fn pwr_mode(&mut self) -> PWR_MODE_W<CM4_PWR_CTL_SPEC> {
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
#[doc = "CM4 power control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm4_pwr_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm4_pwr_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CM4_PWR_CTL_SPEC;
impl crate::RegisterSpec for CM4_PWR_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm4_pwr_ctl::R`](R) reader structure"]
impl crate::Readable for CM4_PWR_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cm4_pwr_ctl::W`](W) writer structure"]
impl crate::Writable for CM4_PWR_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CM4_PWR_CTL to value 0xfa05_0001"]
impl crate::Resettable for CM4_PWR_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0xfa05_0001;
}
