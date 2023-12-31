#[doc = "Register `USBIO_CTL` reader"]
pub type R = crate::R<USBIO_CTL_SPEC>;
#[doc = "Register `USBIO_CTL` writer"]
pub type W = crate::W<USBIO_CTL_SPEC>;
#[doc = "Field `DM_P` reader - The GPIO Drive Mode for DP IO pad. This field only applies if USBIO_CR1.IOMODE =1. Data comes from the corresponding GPIO.DR register."]
pub type DM_P_R = crate::FieldReader<DM_P_A>;
#[doc = "The GPIO Drive Mode for DP IO pad. This field only applies if USBIO_CR1.IOMODE =1. Data comes from the corresponding GPIO.DR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DM_P_A {
    #[doc = "0: Mode 0: Output buffer off (high Z). Input buffer off."]
    OFF = 0,
    #[doc = "1: Mode 1: Output buffer off (high Z). Input buffer on. Other values, not supported."]
    INPUT = 1,
}
impl From<DM_P_A> for u8 {
    #[inline(always)]
    fn from(variant: DM_P_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DM_P_A {
    type Ux = u8;
}
impl DM_P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DM_P_A> {
        match self.bits {
            0 => Some(DM_P_A::OFF),
            1 => Some(DM_P_A::INPUT),
            _ => None,
        }
    }
    #[doc = "Mode 0: Output buffer off (high Z). Input buffer off."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == DM_P_A::OFF
    }
    #[doc = "Mode 1: Output buffer off (high Z). Input buffer on. Other values, not supported."]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DM_P_A::INPUT
    }
}
#[doc = "Field `DM_P` writer - The GPIO Drive Mode for DP IO pad. This field only applies if USBIO_CR1.IOMODE =1. Data comes from the corresponding GPIO.DR register."]
pub type DM_P_W<'a, REG> = crate::FieldWriter<'a, REG, 3, DM_P_A>;
impl<'a, REG> DM_P_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Mode 0: Output buffer off (high Z). Input buffer off."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(DM_P_A::OFF)
    }
    #[doc = "Mode 1: Output buffer off (high Z). Input buffer on. Other values, not supported."]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(DM_P_A::INPUT)
    }
}
#[doc = "Field `DM_M` reader - The GPIO Drive Mode for DM IO pad."]
pub type DM_M_R = crate::FieldReader;
#[doc = "Field `DM_M` writer - The GPIO Drive Mode for DM IO pad."]
pub type DM_M_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - The GPIO Drive Mode for DP IO pad. This field only applies if USBIO_CR1.IOMODE =1. Data comes from the corresponding GPIO.DR register."]
    #[inline(always)]
    pub fn dm_p(&self) -> DM_P_R {
        DM_P_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - The GPIO Drive Mode for DM IO pad."]
    #[inline(always)]
    pub fn dm_m(&self) -> DM_M_R {
        DM_M_R::new(((self.bits >> 3) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - The GPIO Drive Mode for DP IO pad. This field only applies if USBIO_CR1.IOMODE =1. Data comes from the corresponding GPIO.DR register."]
    #[inline(always)]
    #[must_use]
    pub fn dm_p(&mut self) -> DM_P_W<USBIO_CTL_SPEC> {
        DM_P_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - The GPIO Drive Mode for DM IO pad."]
    #[inline(always)]
    #[must_use]
    pub fn dm_m(&mut self) -> DM_M_W<USBIO_CTL_SPEC> {
        DM_M_W::new(self, 3)
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
#[doc = "USB IO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbio_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbio_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBIO_CTL_SPEC;
impl crate::RegisterSpec for USBIO_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbio_ctl::R`](R) reader structure"]
impl crate::Readable for USBIO_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbio_ctl::W`](W) writer structure"]
impl crate::Writable for USBIO_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBIO_CTL to value 0"]
impl crate::Resettable for USBIO_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
