#[doc = "Register `CLK_ECO_CONFIG` reader"]
pub type R = crate::R<CLK_ECO_CONFIG_SPEC>;
#[doc = "Register `CLK_ECO_CONFIG` writer"]
pub type W = crate::W<CLK_ECO_CONFIG_SPEC>;
#[doc = "Field `AGC_EN` reader - Automatic Gain Control (AGC) enable. When set, the oscillation amplitude is controlled to the level selected by ECO_TRIM0.ATRIM. When low, the amplitude is not explicitly controlled and can be as high as the vddd supply. WARNING: use care when disabling AGC because driving a crystal beyond its rated limit can permanently damage the crystal."]
pub type AGC_EN_R = crate::BitReader;
#[doc = "Field `AGC_EN` writer - Automatic Gain Control (AGC) enable. When set, the oscillation amplitude is controlled to the level selected by ECO_TRIM0.ATRIM. When low, the amplitude is not explicitly controlled and can be as high as the vddd supply. WARNING: use care when disabling AGC because driving a crystal beyond its rated limit can permanently damage the crystal."]
pub type AGC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECO_EN` reader - Master enable for ECO oscillator."]
pub type ECO_EN_R = crate::BitReader;
#[doc = "Field `ECO_EN` writer - Master enable for ECO oscillator."]
pub type ECO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Automatic Gain Control (AGC) enable. When set, the oscillation amplitude is controlled to the level selected by ECO_TRIM0.ATRIM. When low, the amplitude is not explicitly controlled and can be as high as the vddd supply. WARNING: use care when disabling AGC because driving a crystal beyond its rated limit can permanently damage the crystal."]
    #[inline(always)]
    pub fn agc_en(&self) -> AGC_EN_R {
        AGC_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 31 - Master enable for ECO oscillator."]
    #[inline(always)]
    pub fn eco_en(&self) -> ECO_EN_R {
        ECO_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Automatic Gain Control (AGC) enable. When set, the oscillation amplitude is controlled to the level selected by ECO_TRIM0.ATRIM. When low, the amplitude is not explicitly controlled and can be as high as the vddd supply. WARNING: use care when disabling AGC because driving a crystal beyond its rated limit can permanently damage the crystal."]
    #[inline(always)]
    #[must_use]
    pub fn agc_en(&mut self) -> AGC_EN_W<CLK_ECO_CONFIG_SPEC> {
        AGC_EN_W::new(self, 1)
    }
    #[doc = "Bit 31 - Master enable for ECO oscillator."]
    #[inline(always)]
    #[must_use]
    pub fn eco_en(&mut self) -> ECO_EN_W<CLK_ECO_CONFIG_SPEC> {
        ECO_EN_W::new(self, 31)
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
#[doc = "ECO Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_eco_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_eco_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_ECO_CONFIG_SPEC;
impl crate::RegisterSpec for CLK_ECO_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_eco_config::R`](R) reader structure"]
impl crate::Readable for CLK_ECO_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_eco_config::W`](W) writer structure"]
impl crate::Writable for CLK_ECO_CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_ECO_CONFIG to value 0x02"]
impl crate::Resettable for CLK_ECO_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
