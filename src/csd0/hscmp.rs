#[doc = "Register `HSCMP` reader"]
pub type R = crate::R<HSCMP_SPEC>;
#[doc = "Register `HSCMP` writer"]
pub type W = crate::W<HSCMP_SPEC>;
#[doc = "Field `HSCMP_EN` reader - High Speed Comparator enable"]
pub type HSCMP_EN_R = crate::BitReader<HSCMP_EN_A>;
#[doc = "High Speed Comparator enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSCMP_EN_A {
    #[doc = "0: Disable comparator, output is zero"]
    OFF = 0,
    #[doc = "1: On, regular operation. Note that CONFIG.LP_MODE determines the power mode level"]
    ON = 1,
}
impl From<HSCMP_EN_A> for bool {
    #[inline(always)]
    fn from(variant: HSCMP_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl HSCMP_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSCMP_EN_A {
        match self.bits {
            false => HSCMP_EN_A::OFF,
            true => HSCMP_EN_A::ON,
        }
    }
    #[doc = "Disable comparator, output is zero"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == HSCMP_EN_A::OFF
    }
    #[doc = "On, regular operation. Note that CONFIG.LP_MODE determines the power mode level"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == HSCMP_EN_A::ON
    }
}
#[doc = "Field `HSCMP_EN` writer - High Speed Comparator enable"]
pub type HSCMP_EN_W<'a, REG> = crate::BitWriter<'a, REG, HSCMP_EN_A>;
impl<'a, REG> HSCMP_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable comparator, output is zero"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(HSCMP_EN_A::OFF)
    }
    #[doc = "On, regular operation. Note that CONFIG.LP_MODE determines the power mode level"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(HSCMP_EN_A::ON)
    }
}
#[doc = "Field `HSCMP_INVERT` reader - Invert the HSCMP output before it is used to control switches and the CSD sequencer. This bit does not affect the ADC sequencer or the STATUS.HSCMP_OUT"]
pub type HSCMP_INVERT_R = crate::BitReader;
#[doc = "Field `HSCMP_INVERT` writer - Invert the HSCMP output before it is used to control switches and the CSD sequencer. This bit does not affect the ADC sequencer or the STATUS.HSCMP_OUT"]
pub type HSCMP_INVERT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AZ_EN` reader - Auto-Zero enable, allow the Sequencer to Auto-Zero this component"]
pub type AZ_EN_R = crate::BitReader;
#[doc = "Field `AZ_EN` writer - Auto-Zero enable, allow the Sequencer to Auto-Zero this component"]
pub type AZ_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - High Speed Comparator enable"]
    #[inline(always)]
    pub fn hscmp_en(&self) -> HSCMP_EN_R {
        HSCMP_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Invert the HSCMP output before it is used to control switches and the CSD sequencer. This bit does not affect the ADC sequencer or the STATUS.HSCMP_OUT"]
    #[inline(always)]
    pub fn hscmp_invert(&self) -> HSCMP_INVERT_R {
        HSCMP_INVERT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 31 - Auto-Zero enable, allow the Sequencer to Auto-Zero this component"]
    #[inline(always)]
    pub fn az_en(&self) -> AZ_EN_R {
        AZ_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - High Speed Comparator enable"]
    #[inline(always)]
    #[must_use]
    pub fn hscmp_en(&mut self) -> HSCMP_EN_W<HSCMP_SPEC> {
        HSCMP_EN_W::new(self, 0)
    }
    #[doc = "Bit 4 - Invert the HSCMP output before it is used to control switches and the CSD sequencer. This bit does not affect the ADC sequencer or the STATUS.HSCMP_OUT"]
    #[inline(always)]
    #[must_use]
    pub fn hscmp_invert(&mut self) -> HSCMP_INVERT_W<HSCMP_SPEC> {
        HSCMP_INVERT_W::new(self, 4)
    }
    #[doc = "Bit 31 - Auto-Zero enable, allow the Sequencer to Auto-Zero this component"]
    #[inline(always)]
    #[must_use]
    pub fn az_en(&mut self) -> AZ_EN_W<HSCMP_SPEC> {
        AZ_EN_W::new(self, 31)
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
#[doc = "High Speed Comparator configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hscmp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hscmp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSCMP_SPEC;
impl crate::RegisterSpec for HSCMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hscmp::R`](R) reader structure"]
impl crate::Readable for HSCMP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hscmp::W`](W) writer structure"]
impl crate::Writable for HSCMP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSCMP to value 0"]
impl crate::Resettable for HSCMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
