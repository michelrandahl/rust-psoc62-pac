#[doc = "Register `CLK_FLL_CONFIG3` reader"]
pub type R = crate::R<CLK_FLL_CONFIG3_SPEC>;
#[doc = "Register `CLK_FLL_CONFIG3` writer"]
pub type W = crate::W<CLK_FLL_CONFIG3_SPEC>;
#[doc = "Field `FLL_LF_IGAIN` reader - FLL Loop Filter Gain Setting #1. The proportional gain is the sum of FLL_LF_IGAIN and FLL_LF_PGAIN. 0: 1/256 1: 1/128 2: 1/64 3: 1/32 4: 1/16 5: 1/8 6: 1/4 7: 1/2 8: 1.0 9: 2.0 10: 4.0 11: 8.0 >=12: illegal"]
pub type FLL_LF_IGAIN_R = crate::FieldReader;
#[doc = "Field `FLL_LF_IGAIN` writer - FLL Loop Filter Gain Setting #1. The proportional gain is the sum of FLL_LF_IGAIN and FLL_LF_PGAIN. 0: 1/256 1: 1/128 2: 1/64 3: 1/32 4: 1/16 5: 1/8 6: 1/4 7: 1/2 8: 1.0 9: 2.0 10: 4.0 11: 8.0 >=12: illegal"]
pub type FLL_LF_IGAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FLL_LF_PGAIN` reader - FLL Loop Filter Gain Setting #2. The proportional gain is the sum of FLL_LF_IGAIN and FLL_LF_PGAIN. 0: 1/256 1: 1/128 2: 1/64 3: 1/32 4: 1/16 5: 1/8 6: 1/4 7: 1/2 8: 1.0 9: 2.0 10: 4.0 11: 8.0 >=12: illegal"]
pub type FLL_LF_PGAIN_R = crate::FieldReader;
#[doc = "Field `FLL_LF_PGAIN` writer - FLL Loop Filter Gain Setting #2. The proportional gain is the sum of FLL_LF_IGAIN and FLL_LF_PGAIN. 0: 1/256 1: 1/128 2: 1/64 3: 1/32 4: 1/16 5: 1/8 6: 1/4 7: 1/2 8: 1.0 9: 2.0 10: 4.0 11: 8.0 >=12: illegal"]
pub type FLL_LF_PGAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SETTLING_COUNT` reader - Number of undivided reference clock cycles to wait after changing the CCO trim until the loop measurement restarts. A delay allows the CCO output to settle and gives a more accurate measurement. The default is tuned to an 8MHz reference clock since the IMO is expected to be the most common use case. 0: no settling time 1: wait one reference clock cycle ... 8191: wait 8191 reference clock cycles"]
pub type SETTLING_COUNT_R = crate::FieldReader<u16>;
#[doc = "Field `SETTLING_COUNT` writer - Number of undivided reference clock cycles to wait after changing the CCO trim until the loop measurement restarts. A delay allows the CCO output to settle and gives a more accurate measurement. The default is tuned to an 8MHz reference clock since the IMO is expected to be the most common use case. 0: no settling time 1: wait one reference clock cycle ... 8191: wait 8191 reference clock cycles"]
pub type SETTLING_COUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `BYPASS_SEL` reader - Bypass mux located just after FLL output. See FLL_ENABLE description for instructions on how to use this field when enabling/disabling the FLL."]
pub type BYPASS_SEL_R = crate::FieldReader<BYPASS_SEL_A>;
#[doc = "Bypass mux located just after FLL output. See FLL_ENABLE description for instructions on how to use this field when enabling/disabling the FLL.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BYPASS_SEL_A {
    #[doc = "0: N/A"]
    AUTO = 0,
    #[doc = "1: N/A"]
    AUTO1 = 1,
    #[doc = "2: Select FLL reference input (bypass mode). Ignores lock indicator"]
    FLL_REF = 2,
    #[doc = "3: Select FLL output. Ignores lock indicator."]
    FLL_OUT = 3,
}
impl From<BYPASS_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: BYPASS_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BYPASS_SEL_A {
    type Ux = u8;
}
impl BYPASS_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BYPASS_SEL_A {
        match self.bits {
            0 => BYPASS_SEL_A::AUTO,
            1 => BYPASS_SEL_A::AUTO1,
            2 => BYPASS_SEL_A::FLL_REF,
            3 => BYPASS_SEL_A::FLL_OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == BYPASS_SEL_A::AUTO
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_auto1(&self) -> bool {
        *self == BYPASS_SEL_A::AUTO1
    }
    #[doc = "Select FLL reference input (bypass mode). Ignores lock indicator"]
    #[inline(always)]
    pub fn is_fll_ref(&self) -> bool {
        *self == BYPASS_SEL_A::FLL_REF
    }
    #[doc = "Select FLL output. Ignores lock indicator."]
    #[inline(always)]
    pub fn is_fll_out(&self) -> bool {
        *self == BYPASS_SEL_A::FLL_OUT
    }
}
#[doc = "Field `BYPASS_SEL` writer - Bypass mux located just after FLL output. See FLL_ENABLE description for instructions on how to use this field when enabling/disabling the FLL."]
pub type BYPASS_SEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, BYPASS_SEL_A>;
impl<'a, REG> BYPASS_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "N/A"]
    #[inline(always)]
    pub fn auto(self) -> &'a mut crate::W<REG> {
        self.variant(BYPASS_SEL_A::AUTO)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn auto1(self) -> &'a mut crate::W<REG> {
        self.variant(BYPASS_SEL_A::AUTO1)
    }
    #[doc = "Select FLL reference input (bypass mode). Ignores lock indicator"]
    #[inline(always)]
    pub fn fll_ref(self) -> &'a mut crate::W<REG> {
        self.variant(BYPASS_SEL_A::FLL_REF)
    }
    #[doc = "Select FLL output. Ignores lock indicator."]
    #[inline(always)]
    pub fn fll_out(self) -> &'a mut crate::W<REG> {
        self.variant(BYPASS_SEL_A::FLL_OUT)
    }
}
impl R {
    #[doc = "Bits 0:3 - FLL Loop Filter Gain Setting #1. The proportional gain is the sum of FLL_LF_IGAIN and FLL_LF_PGAIN. 0: 1/256 1: 1/128 2: 1/64 3: 1/32 4: 1/16 5: 1/8 6: 1/4 7: 1/2 8: 1.0 9: 2.0 10: 4.0 11: 8.0 >=12: illegal"]
    #[inline(always)]
    pub fn fll_lf_igain(&self) -> FLL_LF_IGAIN_R {
        FLL_LF_IGAIN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - FLL Loop Filter Gain Setting #2. The proportional gain is the sum of FLL_LF_IGAIN and FLL_LF_PGAIN. 0: 1/256 1: 1/128 2: 1/64 3: 1/32 4: 1/16 5: 1/8 6: 1/4 7: 1/2 8: 1.0 9: 2.0 10: 4.0 11: 8.0 >=12: illegal"]
    #[inline(always)]
    pub fn fll_lf_pgain(&self) -> FLL_LF_PGAIN_R {
        FLL_LF_PGAIN_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:20 - Number of undivided reference clock cycles to wait after changing the CCO trim until the loop measurement restarts. A delay allows the CCO output to settle and gives a more accurate measurement. The default is tuned to an 8MHz reference clock since the IMO is expected to be the most common use case. 0: no settling time 1: wait one reference clock cycle ... 8191: wait 8191 reference clock cycles"]
    #[inline(always)]
    pub fn settling_count(&self) -> SETTLING_COUNT_R {
        SETTLING_COUNT_R::new(((self.bits >> 8) & 0x1fff) as u16)
    }
    #[doc = "Bits 28:29 - Bypass mux located just after FLL output. See FLL_ENABLE description for instructions on how to use this field when enabling/disabling the FLL."]
    #[inline(always)]
    pub fn bypass_sel(&self) -> BYPASS_SEL_R {
        BYPASS_SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - FLL Loop Filter Gain Setting #1. The proportional gain is the sum of FLL_LF_IGAIN and FLL_LF_PGAIN. 0: 1/256 1: 1/128 2: 1/64 3: 1/32 4: 1/16 5: 1/8 6: 1/4 7: 1/2 8: 1.0 9: 2.0 10: 4.0 11: 8.0 >=12: illegal"]
    #[inline(always)]
    #[must_use]
    pub fn fll_lf_igain(&mut self) -> FLL_LF_IGAIN_W<CLK_FLL_CONFIG3_SPEC> {
        FLL_LF_IGAIN_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - FLL Loop Filter Gain Setting #2. The proportional gain is the sum of FLL_LF_IGAIN and FLL_LF_PGAIN. 0: 1/256 1: 1/128 2: 1/64 3: 1/32 4: 1/16 5: 1/8 6: 1/4 7: 1/2 8: 1.0 9: 2.0 10: 4.0 11: 8.0 >=12: illegal"]
    #[inline(always)]
    #[must_use]
    pub fn fll_lf_pgain(&mut self) -> FLL_LF_PGAIN_W<CLK_FLL_CONFIG3_SPEC> {
        FLL_LF_PGAIN_W::new(self, 4)
    }
    #[doc = "Bits 8:20 - Number of undivided reference clock cycles to wait after changing the CCO trim until the loop measurement restarts. A delay allows the CCO output to settle and gives a more accurate measurement. The default is tuned to an 8MHz reference clock since the IMO is expected to be the most common use case. 0: no settling time 1: wait one reference clock cycle ... 8191: wait 8191 reference clock cycles"]
    #[inline(always)]
    #[must_use]
    pub fn settling_count(&mut self) -> SETTLING_COUNT_W<CLK_FLL_CONFIG3_SPEC> {
        SETTLING_COUNT_W::new(self, 8)
    }
    #[doc = "Bits 28:29 - Bypass mux located just after FLL output. See FLL_ENABLE description for instructions on how to use this field when enabling/disabling the FLL."]
    #[inline(always)]
    #[must_use]
    pub fn bypass_sel(&mut self) -> BYPASS_SEL_W<CLK_FLL_CONFIG3_SPEC> {
        BYPASS_SEL_W::new(self, 28)
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
#[doc = "FLL Configuration Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_fll_config3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_fll_config3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_FLL_CONFIG3_SPEC;
impl crate::RegisterSpec for CLK_FLL_CONFIG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_fll_config3::R`](R) reader structure"]
impl crate::Readable for CLK_FLL_CONFIG3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_fll_config3::W`](W) writer structure"]
impl crate::Writable for CLK_FLL_CONFIG3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_FLL_CONFIG3 to value 0x2800"]
impl crate::Resettable for CLK_FLL_CONFIG3_SPEC {
    const RESET_VALUE: Self::Ux = 0x2800;
}
