#[doc = "Register `CLK_TRIM_ECO_CTL` reader"]
pub type R = crate::R<CLK_TRIM_ECO_CTL_SPEC>;
#[doc = "Register `CLK_TRIM_ECO_CTL` writer"]
pub type W = crate::W<CLK_TRIM_ECO_CTL_SPEC>;
#[doc = "Field `WDTRIM` reader - Watch Dog Trim - Delta voltage below steady state level 0x0 - 50mV 0x1 - 75mV 0x2 - 100mV 0x3 - 125mV 0x4 - 150mV 0x5 - 175mV 0x6 - 200mV 0x7 - 225mV"]
pub type WDTRIM_R = crate::FieldReader;
#[doc = "Field `WDTRIM` writer - Watch Dog Trim - Delta voltage below steady state level 0x0 - 50mV 0x1 - 75mV 0x2 - 100mV 0x3 - 125mV 0x4 - 150mV 0x5 - 175mV 0x6 - 200mV 0x7 - 225mV"]
pub type WDTRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ATRIM` reader - Amplitude trim to set the crystal drive level when ECO_CONFIG.AGC_EN=1. WARNING: use care when setting this field because driving a crystal beyond its rated limit can permanently damage the crystal. 0x0 - 150mV 0x1 - 175mV 0x2 - 200mV 0x3 - 225mV 0x4 - 250mV 0x5 - 275mV 0x6 - 300mV 0x7 - 325mV 0x8 - 350mV 0x9 - 375mV 0xA - 400mV 0xB - 425mV 0xC - 450mV 0xD - 475mV 0xE - 500mV 0xF - 525mV"]
pub type ATRIM_R = crate::FieldReader;
#[doc = "Field `ATRIM` writer - Amplitude trim to set the crystal drive level when ECO_CONFIG.AGC_EN=1. WARNING: use care when setting this field because driving a crystal beyond its rated limit can permanently damage the crystal. 0x0 - 150mV 0x1 - 175mV 0x2 - 200mV 0x3 - 225mV 0x4 - 250mV 0x5 - 275mV 0x6 - 300mV 0x7 - 325mV 0x8 - 350mV 0x9 - 375mV 0xA - 400mV 0xB - 425mV 0xC - 450mV 0xD - 475mV 0xE - 500mV 0xF - 525mV"]
pub type ATRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FTRIM` reader - Filter Trim - 3rd harmonic oscillation"]
pub type FTRIM_R = crate::FieldReader;
#[doc = "Field `FTRIM` writer - Filter Trim - 3rd harmonic oscillation"]
pub type FTRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RTRIM` reader - Feedback resistor Trim"]
pub type RTRIM_R = crate::FieldReader;
#[doc = "Field `RTRIM` writer - Feedback resistor Trim"]
pub type RTRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GTRIM` reader - Gain Trim - Startup time"]
pub type GTRIM_R = crate::FieldReader;
#[doc = "Field `GTRIM` writer - Gain Trim - Startup time"]
pub type GTRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ITRIM` reader - Current Trim"]
pub type ITRIM_R = crate::FieldReader;
#[doc = "Field `ITRIM` writer - Current Trim"]
pub type ITRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:2 - Watch Dog Trim - Delta voltage below steady state level 0x0 - 50mV 0x1 - 75mV 0x2 - 100mV 0x3 - 125mV 0x4 - 150mV 0x5 - 175mV 0x6 - 200mV 0x7 - 225mV"]
    #[inline(always)]
    pub fn wdtrim(&self) -> WDTRIM_R {
        WDTRIM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:7 - Amplitude trim to set the crystal drive level when ECO_CONFIG.AGC_EN=1. WARNING: use care when setting this field because driving a crystal beyond its rated limit can permanently damage the crystal. 0x0 - 150mV 0x1 - 175mV 0x2 - 200mV 0x3 - 225mV 0x4 - 250mV 0x5 - 275mV 0x6 - 300mV 0x7 - 325mV 0x8 - 350mV 0x9 - 375mV 0xA - 400mV 0xB - 425mV 0xC - 450mV 0xD - 475mV 0xE - 500mV 0xF - 525mV"]
    #[inline(always)]
    pub fn atrim(&self) -> ATRIM_R {
        ATRIM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Filter Trim - 3rd harmonic oscillation"]
    #[inline(always)]
    pub fn ftrim(&self) -> FTRIM_R {
        FTRIM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Feedback resistor Trim"]
    #[inline(always)]
    pub fn rtrim(&self) -> RTRIM_R {
        RTRIM_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Gain Trim - Startup time"]
    #[inline(always)]
    pub fn gtrim(&self) -> GTRIM_R {
        GTRIM_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:21 - Current Trim"]
    #[inline(always)]
    pub fn itrim(&self) -> ITRIM_R {
        ITRIM_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Watch Dog Trim - Delta voltage below steady state level 0x0 - 50mV 0x1 - 75mV 0x2 - 100mV 0x3 - 125mV 0x4 - 150mV 0x5 - 175mV 0x6 - 200mV 0x7 - 225mV"]
    #[inline(always)]
    #[must_use]
    pub fn wdtrim(&mut self) -> WDTRIM_W<CLK_TRIM_ECO_CTL_SPEC> {
        WDTRIM_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Amplitude trim to set the crystal drive level when ECO_CONFIG.AGC_EN=1. WARNING: use care when setting this field because driving a crystal beyond its rated limit can permanently damage the crystal. 0x0 - 150mV 0x1 - 175mV 0x2 - 200mV 0x3 - 225mV 0x4 - 250mV 0x5 - 275mV 0x6 - 300mV 0x7 - 325mV 0x8 - 350mV 0x9 - 375mV 0xA - 400mV 0xB - 425mV 0xC - 450mV 0xD - 475mV 0xE - 500mV 0xF - 525mV"]
    #[inline(always)]
    #[must_use]
    pub fn atrim(&mut self) -> ATRIM_W<CLK_TRIM_ECO_CTL_SPEC> {
        ATRIM_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - Filter Trim - 3rd harmonic oscillation"]
    #[inline(always)]
    #[must_use]
    pub fn ftrim(&mut self) -> FTRIM_W<CLK_TRIM_ECO_CTL_SPEC> {
        FTRIM_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Feedback resistor Trim"]
    #[inline(always)]
    #[must_use]
    pub fn rtrim(&mut self) -> RTRIM_W<CLK_TRIM_ECO_CTL_SPEC> {
        RTRIM_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Gain Trim - Startup time"]
    #[inline(always)]
    #[must_use]
    pub fn gtrim(&mut self) -> GTRIM_W<CLK_TRIM_ECO_CTL_SPEC> {
        GTRIM_W::new(self, 12)
    }
    #[doc = "Bits 16:21 - Current Trim"]
    #[inline(always)]
    #[must_use]
    pub fn itrim(&mut self) -> ITRIM_W<CLK_TRIM_ECO_CTL_SPEC> {
        ITRIM_W::new(self, 16)
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
#[doc = "ECO Trim Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_trim_eco_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_trim_eco_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_TRIM_ECO_CTL_SPEC;
impl crate::RegisterSpec for CLK_TRIM_ECO_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_trim_eco_ctl::R`](R) reader structure"]
impl crate::Readable for CLK_TRIM_ECO_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_trim_eco_ctl::W`](W) writer structure"]
impl crate::Writable for CLK_TRIM_ECO_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_TRIM_ECO_CTL to value 0x001f_0003"]
impl crate::Resettable for CLK_TRIM_ECO_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x001f_0003;
}
