#[doc = "Register `CLK_FLL_CONFIG4` reader"]
pub type R = crate::R<CLK_FLL_CONFIG4_SPEC>;
#[doc = "Register `CLK_FLL_CONFIG4` writer"]
pub type W = crate::W<CLK_FLL_CONFIG4_SPEC>;
#[doc = "Field `CCO_LIMIT` reader - Maximum CCO offset allowed (used to prevent FLL dynamics from selecting an CCO frequency that the logic cannot support)"]
pub type CCO_LIMIT_R = crate::FieldReader;
#[doc = "Field `CCO_LIMIT` writer - Maximum CCO offset allowed (used to prevent FLL dynamics from selecting an CCO frequency that the logic cannot support)"]
pub type CCO_LIMIT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CCO_RANGE` reader - Frequency range of CCO"]
pub type CCO_RANGE_R = crate::FieldReader<CCO_RANGE_A>;
#[doc = "Frequency range of CCO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CCO_RANGE_A {
    #[doc = "0: Target frequency is in range \\[48, 64) MHz"]
    RANGE0 = 0,
    #[doc = "1: Target frequency is in range \\[64, 85) MHz"]
    RANGE1 = 1,
    #[doc = "2: Target frequency is in range \\[85, 113) MHz"]
    RANGE2 = 2,
    #[doc = "3: Target frequency is in range \\[113, 150) MHz"]
    RANGE3 = 3,
    #[doc = "4: Target frequency is in range \\[150, 200\\]
MHz"]
    RANGE4 = 4,
}
impl From<CCO_RANGE_A> for u8 {
    #[inline(always)]
    fn from(variant: CCO_RANGE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CCO_RANGE_A {
    type Ux = u8;
}
impl CCO_RANGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CCO_RANGE_A> {
        match self.bits {
            0 => Some(CCO_RANGE_A::RANGE0),
            1 => Some(CCO_RANGE_A::RANGE1),
            2 => Some(CCO_RANGE_A::RANGE2),
            3 => Some(CCO_RANGE_A::RANGE3),
            4 => Some(CCO_RANGE_A::RANGE4),
            _ => None,
        }
    }
    #[doc = "Target frequency is in range \\[48, 64) MHz"]
    #[inline(always)]
    pub fn is_range0(&self) -> bool {
        *self == CCO_RANGE_A::RANGE0
    }
    #[doc = "Target frequency is in range \\[64, 85) MHz"]
    #[inline(always)]
    pub fn is_range1(&self) -> bool {
        *self == CCO_RANGE_A::RANGE1
    }
    #[doc = "Target frequency is in range \\[85, 113) MHz"]
    #[inline(always)]
    pub fn is_range2(&self) -> bool {
        *self == CCO_RANGE_A::RANGE2
    }
    #[doc = "Target frequency is in range \\[113, 150) MHz"]
    #[inline(always)]
    pub fn is_range3(&self) -> bool {
        *self == CCO_RANGE_A::RANGE3
    }
    #[doc = "Target frequency is in range \\[150, 200\\]
MHz"]
    #[inline(always)]
    pub fn is_range4(&self) -> bool {
        *self == CCO_RANGE_A::RANGE4
    }
}
#[doc = "Field `CCO_RANGE` writer - Frequency range of CCO"]
pub type CCO_RANGE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CCO_RANGE_A>;
impl<'a, REG> CCO_RANGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Target frequency is in range \\[48, 64) MHz"]
    #[inline(always)]
    pub fn range0(self) -> &'a mut crate::W<REG> {
        self.variant(CCO_RANGE_A::RANGE0)
    }
    #[doc = "Target frequency is in range \\[64, 85) MHz"]
    #[inline(always)]
    pub fn range1(self) -> &'a mut crate::W<REG> {
        self.variant(CCO_RANGE_A::RANGE1)
    }
    #[doc = "Target frequency is in range \\[85, 113) MHz"]
    #[inline(always)]
    pub fn range2(self) -> &'a mut crate::W<REG> {
        self.variant(CCO_RANGE_A::RANGE2)
    }
    #[doc = "Target frequency is in range \\[113, 150) MHz"]
    #[inline(always)]
    pub fn range3(self) -> &'a mut crate::W<REG> {
        self.variant(CCO_RANGE_A::RANGE3)
    }
    #[doc = "Target frequency is in range \\[150, 200\\]
MHz"]
    #[inline(always)]
    pub fn range4(self) -> &'a mut crate::W<REG> {
        self.variant(CCO_RANGE_A::RANGE4)
    }
}
#[doc = "Field `CCO_FREQ` reader - CCO frequency code. This is updated by HW when the FLL is enabled. It can be manually updated to use the CCO in an open loop configuration. The meaning of each frequency code depends on the range."]
pub type CCO_FREQ_R = crate::FieldReader<u16>;
#[doc = "Field `CCO_FREQ` writer - CCO frequency code. This is updated by HW when the FLL is enabled. It can be manually updated to use the CCO in an open loop configuration. The meaning of each frequency code depends on the range."]
pub type CCO_FREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `CCO_HW_UPDATE_DIS` reader - Disable CCO frequency update by FLL hardware 0: Hardware update of CCO settings is allowed. Use this setting for normal FLL operation. 1: Hardware update of CCO settings is disabled. Use this setting for open-loop FLL operation."]
pub type CCO_HW_UPDATE_DIS_R = crate::BitReader;
#[doc = "Field `CCO_HW_UPDATE_DIS` writer - Disable CCO frequency update by FLL hardware 0: Hardware update of CCO settings is allowed. Use this setting for normal FLL operation. 1: Hardware update of CCO settings is disabled. Use this setting for open-loop FLL operation."]
pub type CCO_HW_UPDATE_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCO_ENABLE` reader - Enable the CCO. It is required to enable the CCO before using the FLL. 0: Block is powered off 1: Block is powered on"]
pub type CCO_ENABLE_R = crate::BitReader;
#[doc = "Field `CCO_ENABLE` writer - Enable the CCO. It is required to enable the CCO before using the FLL. 0: Block is powered off 1: Block is powered on"]
pub type CCO_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Maximum CCO offset allowed (used to prevent FLL dynamics from selecting an CCO frequency that the logic cannot support)"]
    #[inline(always)]
    pub fn cco_limit(&self) -> CCO_LIMIT_R {
        CCO_LIMIT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Frequency range of CCO"]
    #[inline(always)]
    pub fn cco_range(&self) -> CCO_RANGE_R {
        CCO_RANGE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:24 - CCO frequency code. This is updated by HW when the FLL is enabled. It can be manually updated to use the CCO in an open loop configuration. The meaning of each frequency code depends on the range."]
    #[inline(always)]
    pub fn cco_freq(&self) -> CCO_FREQ_R {
        CCO_FREQ_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 30 - Disable CCO frequency update by FLL hardware 0: Hardware update of CCO settings is allowed. Use this setting for normal FLL operation. 1: Hardware update of CCO settings is disabled. Use this setting for open-loop FLL operation."]
    #[inline(always)]
    pub fn cco_hw_update_dis(&self) -> CCO_HW_UPDATE_DIS_R {
        CCO_HW_UPDATE_DIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable the CCO. It is required to enable the CCO before using the FLL. 0: Block is powered off 1: Block is powered on"]
    #[inline(always)]
    pub fn cco_enable(&self) -> CCO_ENABLE_R {
        CCO_ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Maximum CCO offset allowed (used to prevent FLL dynamics from selecting an CCO frequency that the logic cannot support)"]
    #[inline(always)]
    #[must_use]
    pub fn cco_limit(&mut self) -> CCO_LIMIT_W<CLK_FLL_CONFIG4_SPEC> {
        CCO_LIMIT_W::new(self, 0)
    }
    #[doc = "Bits 8:10 - Frequency range of CCO"]
    #[inline(always)]
    #[must_use]
    pub fn cco_range(&mut self) -> CCO_RANGE_W<CLK_FLL_CONFIG4_SPEC> {
        CCO_RANGE_W::new(self, 8)
    }
    #[doc = "Bits 16:24 - CCO frequency code. This is updated by HW when the FLL is enabled. It can be manually updated to use the CCO in an open loop configuration. The meaning of each frequency code depends on the range."]
    #[inline(always)]
    #[must_use]
    pub fn cco_freq(&mut self) -> CCO_FREQ_W<CLK_FLL_CONFIG4_SPEC> {
        CCO_FREQ_W::new(self, 16)
    }
    #[doc = "Bit 30 - Disable CCO frequency update by FLL hardware 0: Hardware update of CCO settings is allowed. Use this setting for normal FLL operation. 1: Hardware update of CCO settings is disabled. Use this setting for open-loop FLL operation."]
    #[inline(always)]
    #[must_use]
    pub fn cco_hw_update_dis(&mut self) -> CCO_HW_UPDATE_DIS_W<CLK_FLL_CONFIG4_SPEC> {
        CCO_HW_UPDATE_DIS_W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable the CCO. It is required to enable the CCO before using the FLL. 0: Block is powered off 1: Block is powered on"]
    #[inline(always)]
    #[must_use]
    pub fn cco_enable(&mut self) -> CCO_ENABLE_W<CLK_FLL_CONFIG4_SPEC> {
        CCO_ENABLE_W::new(self, 31)
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
#[doc = "FLL Configuration Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_fll_config4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_fll_config4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_FLL_CONFIG4_SPEC;
impl crate::RegisterSpec for CLK_FLL_CONFIG4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_fll_config4::R`](R) reader structure"]
impl crate::Readable for CLK_FLL_CONFIG4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_fll_config4::W`](W) writer structure"]
impl crate::Writable for CLK_FLL_CONFIG4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_FLL_CONFIG4 to value 0xff"]
impl crate::Resettable for CLK_FLL_CONFIG4_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
