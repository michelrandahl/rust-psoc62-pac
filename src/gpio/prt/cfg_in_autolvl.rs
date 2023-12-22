#[doc = "Register `CFG_IN_AUTOLVL` reader"]
pub type R = crate::R<CFG_IN_AUTOLVL_SPEC>;
#[doc = "Register `CFG_IN_AUTOLVL` writer"]
pub type W = crate::W<CFG_IN_AUTOLVL_SPEC>;
#[doc = "Field `VTRIP_SEL0_1` reader - Configures the input buffer mode (trip points and hysteresis) for S40E GPIO upper bit. Lower bit is still selected by CFG_IN.VTRIP_SEL0_0 field. This field is used along with CFG_IN.VTRIP_SEL0_0 field as below: {CFG_IN_AUTOLVL.VTRIP_SEL0_1,CFG_IN.VTRIP_SEL0_0}: 0,0: CMOS 0,1: TTL 1,0: input buffer is compatible with automotive. 1,1: input buffer is compatible with automotvie"]
pub type VTRIP_SEL0_1_R = crate::BitReader<VTRIP_SEL0_1_A>;
#[doc = "Configures the input buffer mode (trip points and hysteresis) for S40E GPIO upper bit. Lower bit is still selected by CFG_IN.VTRIP_SEL0_0 field. This field is used along with CFG_IN.VTRIP_SEL0_0 field as below: {CFG_IN_AUTOLVL.VTRIP_SEL0_1,CFG_IN.VTRIP_SEL0_0}: 0,0: CMOS 0,1: TTL 1,0: input buffer is compatible with automotive. 1,1: input buffer is compatible with automotvie\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VTRIP_SEL0_1_A {
    #[doc = "0: Input buffer compatible with CMOS/TTL interfaces as described in CFG_IN.VTRIP_SEL0_0."]
    CMOS_OR_TTL = 0,
    #[doc = "1: Input buffer compatible with AUTO (elevated Vil) interfaces when used along with CFG_IN.VTRIP_SEL0_0."]
    AUTO = 1,
}
impl From<VTRIP_SEL0_1_A> for bool {
    #[inline(always)]
    fn from(variant: VTRIP_SEL0_1_A) -> Self {
        variant as u8 != 0
    }
}
impl VTRIP_SEL0_1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VTRIP_SEL0_1_A {
        match self.bits {
            false => VTRIP_SEL0_1_A::CMOS_OR_TTL,
            true => VTRIP_SEL0_1_A::AUTO,
        }
    }
    #[doc = "Input buffer compatible with CMOS/TTL interfaces as described in CFG_IN.VTRIP_SEL0_0."]
    #[inline(always)]
    pub fn is_cmos_or_ttl(&self) -> bool {
        *self == VTRIP_SEL0_1_A::CMOS_OR_TTL
    }
    #[doc = "Input buffer compatible with AUTO (elevated Vil) interfaces when used along with CFG_IN.VTRIP_SEL0_0."]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == VTRIP_SEL0_1_A::AUTO
    }
}
#[doc = "Field `VTRIP_SEL0_1` writer - Configures the input buffer mode (trip points and hysteresis) for S40E GPIO upper bit. Lower bit is still selected by CFG_IN.VTRIP_SEL0_0 field. This field is used along with CFG_IN.VTRIP_SEL0_0 field as below: {CFG_IN_AUTOLVL.VTRIP_SEL0_1,CFG_IN.VTRIP_SEL0_0}: 0,0: CMOS 0,1: TTL 1,0: input buffer is compatible with automotive. 1,1: input buffer is compatible with automotvie"]
pub type VTRIP_SEL0_1_W<'a, REG> = crate::BitWriter<'a, REG, VTRIP_SEL0_1_A>;
impl<'a, REG> VTRIP_SEL0_1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input buffer compatible with CMOS/TTL interfaces as described in CFG_IN.VTRIP_SEL0_0."]
    #[inline(always)]
    pub fn cmos_or_ttl(self) -> &'a mut crate::W<REG> {
        self.variant(VTRIP_SEL0_1_A::CMOS_OR_TTL)
    }
    #[doc = "Input buffer compatible with AUTO (elevated Vil) interfaces when used along with CFG_IN.VTRIP_SEL0_0."]
    #[inline(always)]
    pub fn auto(self) -> &'a mut crate::W<REG> {
        self.variant(VTRIP_SEL0_1_A::AUTO)
    }
}
#[doc = "Field `VTRIP_SEL1_1` reader - Input buffer compatible with automotive (elevated Vil) interfaces."]
pub type VTRIP_SEL1_1_R = crate::BitReader;
#[doc = "Field `VTRIP_SEL1_1` writer - Input buffer compatible with automotive (elevated Vil) interfaces."]
pub type VTRIP_SEL1_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTRIP_SEL2_1` reader - Input buffer compatible with automotive (elevated Vil) interfaces."]
pub type VTRIP_SEL2_1_R = crate::BitReader;
#[doc = "Field `VTRIP_SEL2_1` writer - Input buffer compatible with automotive (elevated Vil) interfaces."]
pub type VTRIP_SEL2_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTRIP_SEL3_1` reader - Input buffer compatible with automotive (elevated Vil) interfaces."]
pub type VTRIP_SEL3_1_R = crate::BitReader;
#[doc = "Field `VTRIP_SEL3_1` writer - Input buffer compatible with automotive (elevated Vil) interfaces."]
pub type VTRIP_SEL3_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTRIP_SEL4_1` reader - Input buffer compatible with automotive (elevated Vil) interfaces."]
pub type VTRIP_SEL4_1_R = crate::BitReader;
#[doc = "Field `VTRIP_SEL4_1` writer - Input buffer compatible with automotive (elevated Vil) interfaces."]
pub type VTRIP_SEL4_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTRIP_SEL5_1` reader - Input buffer compatible with automotive (elevated Vil) interfaces."]
pub type VTRIP_SEL5_1_R = crate::BitReader;
#[doc = "Field `VTRIP_SEL5_1` writer - Input buffer compatible with automotive (elevated Vil) interfaces."]
pub type VTRIP_SEL5_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTRIP_SEL6_1` reader - Input buffer compatible with automotive (elevated Vil) interfaces."]
pub type VTRIP_SEL6_1_R = crate::BitReader;
#[doc = "Field `VTRIP_SEL6_1` writer - Input buffer compatible with automotive (elevated Vil) interfaces."]
pub type VTRIP_SEL6_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTRIP_SEL7_1` reader - Input buffer compatible with automotive (elevated Vil) interfaces."]
pub type VTRIP_SEL7_1_R = crate::BitReader;
#[doc = "Field `VTRIP_SEL7_1` writer - Input buffer compatible with automotive (elevated Vil) interfaces."]
pub type VTRIP_SEL7_1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures the input buffer mode (trip points and hysteresis) for S40E GPIO upper bit. Lower bit is still selected by CFG_IN.VTRIP_SEL0_0 field. This field is used along with CFG_IN.VTRIP_SEL0_0 field as below: {CFG_IN_AUTOLVL.VTRIP_SEL0_1,CFG_IN.VTRIP_SEL0_0}: 0,0: CMOS 0,1: TTL 1,0: input buffer is compatible with automotive. 1,1: input buffer is compatible with automotvie"]
    #[inline(always)]
    pub fn vtrip_sel0_1(&self) -> VTRIP_SEL0_1_R {
        VTRIP_SEL0_1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn vtrip_sel1_1(&self) -> VTRIP_SEL1_1_R {
        VTRIP_SEL1_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn vtrip_sel2_1(&self) -> VTRIP_SEL2_1_R {
        VTRIP_SEL2_1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn vtrip_sel3_1(&self) -> VTRIP_SEL3_1_R {
        VTRIP_SEL3_1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn vtrip_sel4_1(&self) -> VTRIP_SEL4_1_R {
        VTRIP_SEL4_1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn vtrip_sel5_1(&self) -> VTRIP_SEL5_1_R {
        VTRIP_SEL5_1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn vtrip_sel6_1(&self) -> VTRIP_SEL6_1_R {
        VTRIP_SEL6_1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn vtrip_sel7_1(&self) -> VTRIP_SEL7_1_R {
        VTRIP_SEL7_1_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configures the input buffer mode (trip points and hysteresis) for S40E GPIO upper bit. Lower bit is still selected by CFG_IN.VTRIP_SEL0_0 field. This field is used along with CFG_IN.VTRIP_SEL0_0 field as below: {CFG_IN_AUTOLVL.VTRIP_SEL0_1,CFG_IN.VTRIP_SEL0_0}: 0,0: CMOS 0,1: TTL 1,0: input buffer is compatible with automotive. 1,1: input buffer is compatible with automotvie"]
    #[inline(always)]
    #[must_use]
    pub fn vtrip_sel0_1(&mut self) -> VTRIP_SEL0_1_W<CFG_IN_AUTOLVL_SPEC> {
        VTRIP_SEL0_1_W::new(self, 0)
    }
    #[doc = "Bit 1 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    #[must_use]
    pub fn vtrip_sel1_1(&mut self) -> VTRIP_SEL1_1_W<CFG_IN_AUTOLVL_SPEC> {
        VTRIP_SEL1_1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    #[must_use]
    pub fn vtrip_sel2_1(&mut self) -> VTRIP_SEL2_1_W<CFG_IN_AUTOLVL_SPEC> {
        VTRIP_SEL2_1_W::new(self, 2)
    }
    #[doc = "Bit 3 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    #[must_use]
    pub fn vtrip_sel3_1(&mut self) -> VTRIP_SEL3_1_W<CFG_IN_AUTOLVL_SPEC> {
        VTRIP_SEL3_1_W::new(self, 3)
    }
    #[doc = "Bit 4 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    #[must_use]
    pub fn vtrip_sel4_1(&mut self) -> VTRIP_SEL4_1_W<CFG_IN_AUTOLVL_SPEC> {
        VTRIP_SEL4_1_W::new(self, 4)
    }
    #[doc = "Bit 5 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    #[must_use]
    pub fn vtrip_sel5_1(&mut self) -> VTRIP_SEL5_1_W<CFG_IN_AUTOLVL_SPEC> {
        VTRIP_SEL5_1_W::new(self, 5)
    }
    #[doc = "Bit 6 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    #[must_use]
    pub fn vtrip_sel6_1(&mut self) -> VTRIP_SEL6_1_W<CFG_IN_AUTOLVL_SPEC> {
        VTRIP_SEL6_1_W::new(self, 6)
    }
    #[doc = "Bit 7 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    #[must_use]
    pub fn vtrip_sel7_1(&mut self) -> VTRIP_SEL7_1_W<CFG_IN_AUTOLVL_SPEC> {
        VTRIP_SEL7_1_W::new(self, 7)
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
#[doc = "Port input buffer AUTOLVL configuration register for S40E GPIO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_in_autolvl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_in_autolvl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_IN_AUTOLVL_SPEC;
impl crate::RegisterSpec for CFG_IN_AUTOLVL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_in_autolvl::R`](R) reader structure"]
impl crate::Readable for CFG_IN_AUTOLVL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg_in_autolvl::W`](W) writer structure"]
impl crate::Writable for CFG_IN_AUTOLVL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG_IN_AUTOLVL to value 0"]
impl crate::Resettable for CFG_IN_AUTOLVL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
