#[doc = "Register `CLK_PILO_CONFIG` reader"]
pub type R = crate::R<CLK_PILO_CONFIG_SPEC>;
#[doc = "Register `CLK_PILO_CONFIG` writer"]
pub type W = crate::W<CLK_PILO_CONFIG_SPEC>;
#[doc = "Field `PILO_FFREQ` reader - Fine frequency trim allowing +/-250ppm accuracy with periodic calibration. The nominal step size of the LSB is 8Hz."]
pub type PILO_FFREQ_R = crate::FieldReader<u16>;
#[doc = "Field `PILO_FFREQ` writer - Fine frequency trim allowing +/-250ppm accuracy with periodic calibration. The nominal step size of the LSB is 8Hz."]
pub type PILO_FFREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PILO_CLK_EN` reader - Enable the PILO clock output. See PILO_EN field for required sequencing."]
pub type PILO_CLK_EN_R = crate::BitReader;
#[doc = "Field `PILO_CLK_EN` writer - Enable the PILO clock output. See PILO_EN field for required sequencing."]
pub type PILO_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PILO_RESET_N` reader - Reset the PILO. See PILO_EN field for required sequencing."]
pub type PILO_RESET_N_R = crate::BitReader;
#[doc = "Field `PILO_RESET_N` writer - Reset the PILO. See PILO_EN field for required sequencing."]
pub type PILO_RESET_N_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PILO_EN` reader - Enable PILO. When enabling PILO, set PILO_EN=1, wait 1ms, then PILO_RESET_N=1 and PILO_CLK_EN=1. When disabling PILO, clear PILO_EN=0, PILO_RESET_N=0, and PLO_CLK_EN=0 in the same write cycle."]
pub type PILO_EN_R = crate::BitReader;
#[doc = "Field `PILO_EN` writer - Enable PILO. When enabling PILO, set PILO_EN=1, wait 1ms, then PILO_RESET_N=1 and PILO_CLK_EN=1. When disabling PILO, clear PILO_EN=0, PILO_RESET_N=0, and PLO_CLK_EN=0 in the same write cycle."]
pub type PILO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Fine frequency trim allowing +/-250ppm accuracy with periodic calibration. The nominal step size of the LSB is 8Hz."]
    #[inline(always)]
    pub fn pilo_ffreq(&self) -> PILO_FFREQ_R {
        PILO_FFREQ_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 29 - Enable the PILO clock output. See PILO_EN field for required sequencing."]
    #[inline(always)]
    pub fn pilo_clk_en(&self) -> PILO_CLK_EN_R {
        PILO_CLK_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Reset the PILO. See PILO_EN field for required sequencing."]
    #[inline(always)]
    pub fn pilo_reset_n(&self) -> PILO_RESET_N_R {
        PILO_RESET_N_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable PILO. When enabling PILO, set PILO_EN=1, wait 1ms, then PILO_RESET_N=1 and PILO_CLK_EN=1. When disabling PILO, clear PILO_EN=0, PILO_RESET_N=0, and PLO_CLK_EN=0 in the same write cycle."]
    #[inline(always)]
    pub fn pilo_en(&self) -> PILO_EN_R {
        PILO_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Fine frequency trim allowing +/-250ppm accuracy with periodic calibration. The nominal step size of the LSB is 8Hz."]
    #[inline(always)]
    #[must_use]
    pub fn pilo_ffreq(&mut self) -> PILO_FFREQ_W<CLK_PILO_CONFIG_SPEC> {
        PILO_FFREQ_W::new(self, 0)
    }
    #[doc = "Bit 29 - Enable the PILO clock output. See PILO_EN field for required sequencing."]
    #[inline(always)]
    #[must_use]
    pub fn pilo_clk_en(&mut self) -> PILO_CLK_EN_W<CLK_PILO_CONFIG_SPEC> {
        PILO_CLK_EN_W::new(self, 29)
    }
    #[doc = "Bit 30 - Reset the PILO. See PILO_EN field for required sequencing."]
    #[inline(always)]
    #[must_use]
    pub fn pilo_reset_n(&mut self) -> PILO_RESET_N_W<CLK_PILO_CONFIG_SPEC> {
        PILO_RESET_N_W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable PILO. When enabling PILO, set PILO_EN=1, wait 1ms, then PILO_RESET_N=1 and PILO_CLK_EN=1. When disabling PILO, clear PILO_EN=0, PILO_RESET_N=0, and PLO_CLK_EN=0 in the same write cycle."]
    #[inline(always)]
    #[must_use]
    pub fn pilo_en(&mut self) -> PILO_EN_W<CLK_PILO_CONFIG_SPEC> {
        PILO_EN_W::new(self, 31)
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
#[doc = "Precision ILO Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_pilo_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_pilo_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_PILO_CONFIG_SPEC;
impl crate::RegisterSpec for CLK_PILO_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_pilo_config::R`](R) reader structure"]
impl crate::Readable for CLK_PILO_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_pilo_config::W`](W) writer structure"]
impl crate::Writable for CLK_PILO_CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_PILO_CONFIG to value 0x80"]
impl crate::Resettable for CLK_PILO_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
