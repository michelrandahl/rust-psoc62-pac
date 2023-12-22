#[doc = "Register `RAM0_PWR_MACRO_CTL[%s]` reader"]
pub type R = crate::R<RAM0_PWR_MACRO_CTL_SPEC>;
#[doc = "Register `RAM0_PWR_MACRO_CTL[%s]` writer"]
pub type W = crate::W<RAM0_PWR_MACRO_CTL_SPEC>;
#[doc = "Field `PWR_MODE` reader - SRAM Power mode."]
pub type PWR_MODE_R = crate::FieldReader<PWR_MODE_A>;
#[doc = "SRAM Power mode.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWR_MODE_A {
    #[doc = "0: Turn OFF the SRAM. This will trun OFF both array and periphery power of the SRAM and SRAM memory contents are lost."]
    OFF = 0,
    #[doc = "1: undefined"]
    RSVD = 1,
    #[doc = "2: Keep SRAM in Retained mode. This will turn OFF the SRAM periphery power, but array power is ON to retain memory contents. The SRAM contents will be retained in DeepSleep system power mode."]
    RETAINED = 2,
    #[doc = "3: Enable SRAM for regular operation. The SRAM contents will be retained in DeepSleep system power mode."]
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
            1 => PWR_MODE_A::RSVD,
            2 => PWR_MODE_A::RETAINED,
            3 => PWR_MODE_A::ENABLED,
            _ => unreachable!(),
        }
    }
    #[doc = "Turn OFF the SRAM. This will trun OFF both array and periphery power of the SRAM and SRAM memory contents are lost."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == PWR_MODE_A::OFF
    }
    #[doc = "undefined"]
    #[inline(always)]
    pub fn is_rsvd(&self) -> bool {
        *self == PWR_MODE_A::RSVD
    }
    #[doc = "Keep SRAM in Retained mode. This will turn OFF the SRAM periphery power, but array power is ON to retain memory contents. The SRAM contents will be retained in DeepSleep system power mode."]
    #[inline(always)]
    pub fn is_retained(&self) -> bool {
        *self == PWR_MODE_A::RETAINED
    }
    #[doc = "Enable SRAM for regular operation. The SRAM contents will be retained in DeepSleep system power mode."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PWR_MODE_A::ENABLED
    }
}
#[doc = "Field `PWR_MODE` writer - SRAM Power mode."]
pub type PWR_MODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PWR_MODE_A>;
impl<'a, REG> PWR_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Turn OFF the SRAM. This will trun OFF both array and periphery power of the SRAM and SRAM memory contents are lost."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(PWR_MODE_A::OFF)
    }
    #[doc = "undefined"]
    #[inline(always)]
    pub fn rsvd(self) -> &'a mut crate::W<REG> {
        self.variant(PWR_MODE_A::RSVD)
    }
    #[doc = "Keep SRAM in Retained mode. This will turn OFF the SRAM periphery power, but array power is ON to retain memory contents. The SRAM contents will be retained in DeepSleep system power mode."]
    #[inline(always)]
    pub fn retained(self) -> &'a mut crate::W<REG> {
        self.variant(PWR_MODE_A::RETAINED)
    }
    #[doc = "Enable SRAM for regular operation. The SRAM contents will be retained in DeepSleep system power mode."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWR_MODE_A::ENABLED)
    }
}
#[doc = "Field `VECTKEYSTAT` reader - Register key (to prevent accidental writes). - Should be written with a 0x05fa key value for the write to take effect. - Always reads as 0xfa05."]
pub type VECTKEYSTAT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:1 - SRAM Power mode."]
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
    #[doc = "Bits 0:1 - SRAM Power mode."]
    #[inline(always)]
    #[must_use]
    pub fn pwr_mode(&mut self) -> PWR_MODE_W<RAM0_PWR_MACRO_CTL_SPEC> {
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
#[doc = "RAM 0 power control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram0_pwr_macro_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram0_pwr_macro_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAM0_PWR_MACRO_CTL_SPEC;
impl crate::RegisterSpec for RAM0_PWR_MACRO_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ram0_pwr_macro_ctl::R`](R) reader structure"]
impl crate::Readable for RAM0_PWR_MACRO_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ram0_pwr_macro_ctl::W`](W) writer structure"]
impl crate::Writable for RAM0_PWR_MACRO_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RAM0_PWR_MACRO_CTL[%s]
to value 0xfa05_0003"]
impl crate::Resettable for RAM0_PWR_MACRO_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0xfa05_0003;
}
