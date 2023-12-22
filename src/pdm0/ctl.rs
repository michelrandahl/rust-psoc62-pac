#[doc = "Register `CTL` reader"]
pub type R = crate::R<CTL_SPEC>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Field `PGA_R` reader - Right channel PGA gain: +1.5dB/step, -12dB ~ +10.5dB '0': -12 dB '1': -10.5 dB ... '15' +10.5 dB (Note: These bits are connected to AR36U12.PDM_CORE_CFG.PGA_R)"]
pub type PGA_R_R = crate::FieldReader;
#[doc = "Field `PGA_R` writer - Right channel PGA gain: +1.5dB/step, -12dB ~ +10.5dB '0': -12 dB '1': -10.5 dB ... '15' +10.5 dB (Note: These bits are connected to AR36U12.PDM_CORE_CFG.PGA_R)"]
pub type PGA_R_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PGA_L` reader - Left channel PGA gain: +1.5dB/step, -12dB ~ +10.5dB '0': -12 dB '1': -10.5 dB ... '15': +10.5 dB (Note: These bits are connected to AR36U12.PDM_CORE_CFG.PGA_L)"]
pub type PGA_L_R = crate::FieldReader;
#[doc = "Field `PGA_L` writer - Left channel PGA gain: +1.5dB/step, -12dB ~ +10.5dB '0': -12 dB '1': -10.5 dB ... '15': +10.5 dB (Note: These bits are connected to AR36U12.PDM_CORE_CFG.PGA_L)"]
pub type PGA_L_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SOFT_MUTE` reader - Soft mute function to mute the volume smoothly '0': Disabled. '1': Enabled. (Note: This bit is connected to AR36U12.PDM_CORE_CFG.SOFT_MUTE)"]
pub type SOFT_MUTE_R = crate::BitReader;
#[doc = "Field `SOFT_MUTE` writer - Soft mute function to mute the volume smoothly '0': Disabled. '1': Enabled. (Note: This bit is connected to AR36U12.PDM_CORE_CFG.SOFT_MUTE)"]
pub type SOFT_MUTE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STEP_SEL` reader - Set fine gain step for smooth PGA or Soft-Mute attenuation transition. '0': 0.13dB '1': 0.26dB (Note: This bit is connected to AR36U12.PDM_CORE2_CFG.SEL_STEP)"]
pub type STEP_SEL_R = crate::BitReader;
#[doc = "Field `STEP_SEL` writer - Set fine gain step for smooth PGA or Soft-Mute attenuation transition. '0': 0.13dB '1': 0.26dB (Note: This bit is connected to AR36U12.PDM_CORE2_CFG.SEL_STEP)"]
pub type STEP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLED` reader - Enables the PDM component: '0': Disabled. '1': Enabled."]
pub type ENABLED_R = crate::BitReader;
#[doc = "Field `ENABLED` writer - Enables the PDM component: '0': Disabled. '1': Enabled."]
pub type ENABLED_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Right channel PGA gain: +1.5dB/step, -12dB ~ +10.5dB '0': -12 dB '1': -10.5 dB ... '15' +10.5 dB (Note: These bits are connected to AR36U12.PDM_CORE_CFG.PGA_R)"]
    #[inline(always)]
    pub fn pga_r(&self) -> PGA_R_R {
        PGA_R_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Left channel PGA gain: +1.5dB/step, -12dB ~ +10.5dB '0': -12 dB '1': -10.5 dB ... '15': +10.5 dB (Note: These bits are connected to AR36U12.PDM_CORE_CFG.PGA_L)"]
    #[inline(always)]
    pub fn pga_l(&self) -> PGA_L_R {
        PGA_L_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Soft mute function to mute the volume smoothly '0': Disabled. '1': Enabled. (Note: This bit is connected to AR36U12.PDM_CORE_CFG.SOFT_MUTE)"]
    #[inline(always)]
    pub fn soft_mute(&self) -> SOFT_MUTE_R {
        SOFT_MUTE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Set fine gain step for smooth PGA or Soft-Mute attenuation transition. '0': 0.13dB '1': 0.26dB (Note: This bit is connected to AR36U12.PDM_CORE2_CFG.SEL_STEP)"]
    #[inline(always)]
    pub fn step_sel(&self) -> STEP_SEL_R {
        STEP_SEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 31 - Enables the PDM component: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Right channel PGA gain: +1.5dB/step, -12dB ~ +10.5dB '0': -12 dB '1': -10.5 dB ... '15' +10.5 dB (Note: These bits are connected to AR36U12.PDM_CORE_CFG.PGA_R)"]
    #[inline(always)]
    #[must_use]
    pub fn pga_r(&mut self) -> PGA_R_W<CTL_SPEC> {
        PGA_R_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Left channel PGA gain: +1.5dB/step, -12dB ~ +10.5dB '0': -12 dB '1': -10.5 dB ... '15': +10.5 dB (Note: These bits are connected to AR36U12.PDM_CORE_CFG.PGA_L)"]
    #[inline(always)]
    #[must_use]
    pub fn pga_l(&mut self) -> PGA_L_W<CTL_SPEC> {
        PGA_L_W::new(self, 8)
    }
    #[doc = "Bit 16 - Soft mute function to mute the volume smoothly '0': Disabled. '1': Enabled. (Note: This bit is connected to AR36U12.PDM_CORE_CFG.SOFT_MUTE)"]
    #[inline(always)]
    #[must_use]
    pub fn soft_mute(&mut self) -> SOFT_MUTE_W<CTL_SPEC> {
        SOFT_MUTE_W::new(self, 16)
    }
    #[doc = "Bit 17 - Set fine gain step for smooth PGA or Soft-Mute attenuation transition. '0': 0.13dB '1': 0.26dB (Note: This bit is connected to AR36U12.PDM_CORE2_CFG.SEL_STEP)"]
    #[inline(always)]
    #[must_use]
    pub fn step_sel(&mut self) -> STEP_SEL_W<CTL_SPEC> {
        STEP_SEL_W::new(self, 17)
    }
    #[doc = "Bit 31 - Enables the PDM component: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> ENABLED_W<CTL_SPEC> {
        ENABLED_W::new(self, 31)
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
#[doc = "Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0x0002_0808"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_0808;
}
