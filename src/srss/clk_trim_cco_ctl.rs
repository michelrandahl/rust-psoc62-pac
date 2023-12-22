#[doc = "Register `CLK_TRIM_CCO_CTL` reader"]
pub type R = crate::R<CLK_TRIM_CCO_CTL_SPEC>;
#[doc = "Register `CLK_TRIM_CCO_CTL` writer"]
pub type W = crate::W<CLK_TRIM_CCO_CTL_SPEC>;
#[doc = "Field `CCO_RCSTRIM` reader - CCO reference current source trim."]
pub type CCO_RCSTRIM_R = crate::FieldReader;
#[doc = "Field `CCO_RCSTRIM` writer - CCO reference current source trim."]
pub type CCO_RCSTRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CCO_STABLE_CNT` reader - Terminal count for the stabilization counter from CCO_ENABLE until stable."]
pub type CCO_STABLE_CNT_R = crate::FieldReader;
#[doc = "Field `CCO_STABLE_CNT` writer - Terminal count for the stabilization counter from CCO_ENABLE until stable."]
pub type CCO_STABLE_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ENABLE_CNT` reader - Enables the automatic stabilization counter."]
pub type ENABLE_CNT_R = crate::BitReader;
#[doc = "Field `ENABLE_CNT` writer - Enables the automatic stabilization counter."]
pub type ENABLE_CNT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - CCO reference current source trim."]
    #[inline(always)]
    pub fn cco_rcstrim(&self) -> CCO_RCSTRIM_R {
        CCO_RCSTRIM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Terminal count for the stabilization counter from CCO_ENABLE until stable."]
    #[inline(always)]
    pub fn cco_stable_cnt(&self) -> CCO_STABLE_CNT_R {
        CCO_STABLE_CNT_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - Enables the automatic stabilization counter."]
    #[inline(always)]
    pub fn enable_cnt(&self) -> ENABLE_CNT_R {
        ENABLE_CNT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - CCO reference current source trim."]
    #[inline(always)]
    #[must_use]
    pub fn cco_rcstrim(&mut self) -> CCO_RCSTRIM_W<CLK_TRIM_CCO_CTL_SPEC> {
        CCO_RCSTRIM_W::new(self, 0)
    }
    #[doc = "Bits 24:29 - Terminal count for the stabilization counter from CCO_ENABLE until stable."]
    #[inline(always)]
    #[must_use]
    pub fn cco_stable_cnt(&mut self) -> CCO_STABLE_CNT_W<CLK_TRIM_CCO_CTL_SPEC> {
        CCO_STABLE_CNT_W::new(self, 24)
    }
    #[doc = "Bit 31 - Enables the automatic stabilization counter."]
    #[inline(always)]
    #[must_use]
    pub fn enable_cnt(&mut self) -> ENABLE_CNT_W<CLK_TRIM_CCO_CTL_SPEC> {
        ENABLE_CNT_W::new(self, 31)
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
#[doc = "CCO Trim Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_trim_cco_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_trim_cco_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_TRIM_CCO_CTL_SPEC;
impl crate::RegisterSpec for CLK_TRIM_CCO_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_trim_cco_ctl::R`](R) reader structure"]
impl crate::Readable for CLK_TRIM_CCO_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_trim_cco_ctl::W`](W) writer structure"]
impl crate::Writable for CLK_TRIM_CCO_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_TRIM_CCO_CTL to value 0xa700_0020"]
impl crate::Resettable for CLK_TRIM_CCO_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0xa700_0020;
}
