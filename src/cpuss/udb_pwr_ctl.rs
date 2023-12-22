#[doc = "Register `UDB_PWR_CTL` reader"]
pub type R = crate::R<UDB_PWR_CTL_SPEC>;
#[doc = "Register `UDB_PWR_CTL` writer"]
pub type W = crate::W<UDB_PWR_CTL_SPEC>;
#[doc = "Field `PWR_MODE` reader - Set Power mode for UDBs"]
pub type PWR_MODE_R = crate::FieldReader<PWR_MODE_A>;
#[doc = "Set Power mode for UDBs\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWR_MODE_A {
    #[doc = "0: See CM4_PWR_CTL"]
    OFF = 0,
    #[doc = "1: See CM4_PWR_CTL"]
    RESET = 1,
    #[doc = "2: See CM4_PWR_CTL"]
    RETAINED = 2,
    #[doc = "3: See CM4_PWR_CTL"]
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
    #[doc = "See CM4_PWR_CTL"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == PWR_MODE_A::OFF
    }
    #[doc = "See CM4_PWR_CTL"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == PWR_MODE_A::RESET
    }
    #[doc = "See CM4_PWR_CTL"]
    #[inline(always)]
    pub fn is_retained(&self) -> bool {
        *self == PWR_MODE_A::RETAINED
    }
    #[doc = "See CM4_PWR_CTL"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PWR_MODE_A::ENABLED
    }
}
#[doc = "Field `PWR_MODE` writer - Set Power mode for UDBs"]
pub type PWR_MODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PWR_MODE_A>;
impl<'a, REG> PWR_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "See CM4_PWR_CTL"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(PWR_MODE_A::OFF)
    }
    #[doc = "See CM4_PWR_CTL"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(PWR_MODE_A::RESET)
    }
    #[doc = "See CM4_PWR_CTL"]
    #[inline(always)]
    pub fn retained(self) -> &'a mut crate::W<REG> {
        self.variant(PWR_MODE_A::RETAINED)
    }
    #[doc = "See CM4_PWR_CTL"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWR_MODE_A::ENABLED)
    }
}
#[doc = "Field `VECTKEYSTAT` reader - Register key (to prevent accidental writes). - Should be written with a 0x05fa key value for the write to take effect. - Always reads as 0xfa05."]
pub type VECTKEYSTAT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:1 - Set Power mode for UDBs"]
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
    #[doc = "Bits 0:1 - Set Power mode for UDBs"]
    #[inline(always)]
    #[must_use]
    pub fn pwr_mode(&mut self) -> PWR_MODE_W<UDB_PWR_CTL_SPEC> {
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
#[doc = "UDB power control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udb_pwr_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udb_pwr_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UDB_PWR_CTL_SPEC;
impl crate::RegisterSpec for UDB_PWR_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udb_pwr_ctl::R`](R) reader structure"]
impl crate::Readable for UDB_PWR_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`udb_pwr_ctl::W`](W) writer structure"]
impl crate::Writable for UDB_PWR_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UDB_PWR_CTL to value 0xfa05_0001"]
impl crate::Resettable for UDB_PWR_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0xfa05_0001;
}
