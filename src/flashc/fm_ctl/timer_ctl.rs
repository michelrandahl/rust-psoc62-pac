#[doc = "Register `TIMER_CTL` reader"]
pub type R = crate::R<TIMER_CTL_SPEC>;
#[doc = "Register `TIMER_CTL` writer"]
pub type W = crate::W<TIMER_CTL_SPEC>;
#[doc = "Field `PERIOD` reader - Timer period in either microseconds (SCALE is '0') or 100's of microseconds (SCALE is '1') multiples."]
pub type PERIOD_R = crate::FieldReader<u16>;
#[doc = "Field `PERIOD` writer - Timer period in either microseconds (SCALE is '0') or 100's of microseconds (SCALE is '1') multiples."]
pub type PERIOD_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `SCALE` reader - Timer tick scale: 0: 1 microsecond. 1: 100 microseconds."]
pub type SCALE_R = crate::BitReader;
#[doc = "Field `SCALE` writer - Timer tick scale: 0: 1 microsecond. 1: 100 microseconds."]
pub type SCALE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_SEQUENCE` reader - 1': Starts1 the HV automatic sequencing Cleared by HW"]
pub type AUTO_SEQUENCE_R = crate::BitReader;
#[doc = "Field `AUTO_SEQUENCE` writer - 1': Starts1 the HV automatic sequencing Cleared by HW"]
pub type AUTO_SEQUENCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRE_PROG` reader - 1 during pre-program operation"]
pub type PRE_PROG_R = crate::BitReader;
#[doc = "Field `PRE_PROG` writer - 1 during pre-program operation"]
pub type PRE_PROG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRE_PROG_CSL` reader - 0: CSL lines driven by CSL_DAC 1: CSL lines driven by VNEG_G"]
pub type PRE_PROG_CSL_R = crate::BitReader;
#[doc = "Field `PRE_PROG_CSL` writer - 0: CSL lines driven by CSL_DAC 1: CSL lines driven by VNEG_G"]
pub type PRE_PROG_CSL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUMP_EN` reader - Pump enable: 0: disabled 1: enabled (also requires FM_CTL.IF_SEL to be'1', this additional restriction is required to prevent non intentional clearing of the FM). SW sets this field to '1' to generate a single PE pulse. HW clears this field when timer is expired."]
pub type PUMP_EN_R = crate::BitReader;
#[doc = "Field `PUMP_EN` writer - Pump enable: 0: disabled 1: enabled (also requires FM_CTL.IF_SEL to be'1', this additional restriction is required to prevent non intentional clearing of the FM). SW sets this field to '1' to generate a single PE pulse. HW clears this field when timer is expired."]
pub type PUMP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_EN` reader - ACLK enable (generates a single cycle pulse for the FM): 0: disabled 1: enabled. SW set this field to '1' to generate a single cycle pulse. HW sets this field to '0' when the pulse is generated."]
pub type ACLK_EN_R = crate::BitReader;
#[doc = "Field `ACLK_EN` writer - ACLK enable (generates a single cycle pulse for the FM): 0: disabled 1: enabled. SW set this field to '1' to generate a single cycle pulse. HW sets this field to '0' when the pulse is generated."]
pub type ACLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER_EN` reader - Timer enable: 0: disabled 1: enabled. SW sets this field to '1' to start the timer. HW sets this field to '0' when the timer is expired."]
pub type TIMER_EN_R = crate::BitReader;
#[doc = "Field `TIMER_EN` writer - Timer enable: 0: disabled 1: enabled. SW sets this field to '1' to start the timer. HW sets this field to '0' when the timer is expired."]
pub type TIMER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:14 - Timer period in either microseconds (SCALE is '0') or 100's of microseconds (SCALE is '1') multiples."]
    #[inline(always)]
    pub fn period(&self) -> PERIOD_R {
        PERIOD_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 15 - Timer tick scale: 0: 1 microsecond. 1: 100 microseconds."]
    #[inline(always)]
    pub fn scale(&self) -> SCALE_R {
        SCALE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 24 - 1': Starts1 the HV automatic sequencing Cleared by HW"]
    #[inline(always)]
    pub fn auto_sequence(&self) -> AUTO_SEQUENCE_R {
        AUTO_SEQUENCE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 1 during pre-program operation"]
    #[inline(always)]
    pub fn pre_prog(&self) -> PRE_PROG_R {
        PRE_PROG_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 0: CSL lines driven by CSL_DAC 1: CSL lines driven by VNEG_G"]
    #[inline(always)]
    pub fn pre_prog_csl(&self) -> PRE_PROG_CSL_R {
        PRE_PROG_CSL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 29 - Pump enable: 0: disabled 1: enabled (also requires FM_CTL.IF_SEL to be'1', this additional restriction is required to prevent non intentional clearing of the FM). SW sets this field to '1' to generate a single PE pulse. HW clears this field when timer is expired."]
    #[inline(always)]
    pub fn pump_en(&self) -> PUMP_EN_R {
        PUMP_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - ACLK enable (generates a single cycle pulse for the FM): 0: disabled 1: enabled. SW set this field to '1' to generate a single cycle pulse. HW sets this field to '0' when the pulse is generated."]
    #[inline(always)]
    pub fn aclk_en(&self) -> ACLK_EN_R {
        ACLK_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Timer enable: 0: disabled 1: enabled. SW sets this field to '1' to start the timer. HW sets this field to '0' when the timer is expired."]
    #[inline(always)]
    pub fn timer_en(&self) -> TIMER_EN_R {
        TIMER_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - Timer period in either microseconds (SCALE is '0') or 100's of microseconds (SCALE is '1') multiples."]
    #[inline(always)]
    #[must_use]
    pub fn period(&mut self) -> PERIOD_W<TIMER_CTL_SPEC> {
        PERIOD_W::new(self, 0)
    }
    #[doc = "Bit 15 - Timer tick scale: 0: 1 microsecond. 1: 100 microseconds."]
    #[inline(always)]
    #[must_use]
    pub fn scale(&mut self) -> SCALE_W<TIMER_CTL_SPEC> {
        SCALE_W::new(self, 15)
    }
    #[doc = "Bit 24 - 1': Starts1 the HV automatic sequencing Cleared by HW"]
    #[inline(always)]
    #[must_use]
    pub fn auto_sequence(&mut self) -> AUTO_SEQUENCE_W<TIMER_CTL_SPEC> {
        AUTO_SEQUENCE_W::new(self, 24)
    }
    #[doc = "Bit 25 - 1 during pre-program operation"]
    #[inline(always)]
    #[must_use]
    pub fn pre_prog(&mut self) -> PRE_PROG_W<TIMER_CTL_SPEC> {
        PRE_PROG_W::new(self, 25)
    }
    #[doc = "Bit 26 - 0: CSL lines driven by CSL_DAC 1: CSL lines driven by VNEG_G"]
    #[inline(always)]
    #[must_use]
    pub fn pre_prog_csl(&mut self) -> PRE_PROG_CSL_W<TIMER_CTL_SPEC> {
        PRE_PROG_CSL_W::new(self, 26)
    }
    #[doc = "Bit 29 - Pump enable: 0: disabled 1: enabled (also requires FM_CTL.IF_SEL to be'1', this additional restriction is required to prevent non intentional clearing of the FM). SW sets this field to '1' to generate a single PE pulse. HW clears this field when timer is expired."]
    #[inline(always)]
    #[must_use]
    pub fn pump_en(&mut self) -> PUMP_EN_W<TIMER_CTL_SPEC> {
        PUMP_EN_W::new(self, 29)
    }
    #[doc = "Bit 30 - ACLK enable (generates a single cycle pulse for the FM): 0: disabled 1: enabled. SW set this field to '1' to generate a single cycle pulse. HW sets this field to '0' when the pulse is generated."]
    #[inline(always)]
    #[must_use]
    pub fn aclk_en(&mut self) -> ACLK_EN_W<TIMER_CTL_SPEC> {
        ACLK_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - Timer enable: 0: disabled 1: enabled. SW sets this field to '1' to start the timer. HW sets this field to '0' when the timer is expired."]
    #[inline(always)]
    #[must_use]
    pub fn timer_en(&mut self) -> TIMER_EN_W<TIMER_CTL_SPEC> {
        TIMER_EN_W::new(self, 31)
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
#[doc = "Timer control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER_CTL_SPEC;
impl crate::RegisterSpec for TIMER_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer_ctl::R`](R) reader structure"]
impl crate::Readable for TIMER_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer_ctl::W`](W) writer structure"]
impl crate::Writable for TIMER_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER_CTL to value 0x0400_0001"]
impl crate::Resettable for TIMER_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0400_0001;
}
