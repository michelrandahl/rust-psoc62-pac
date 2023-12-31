#[doc = "Register `CFG_IN` reader"]
pub type R = crate::R<CFG_IN_SPEC>;
#[doc = "Register `CFG_IN` writer"]
pub type W = crate::W<CFG_IN_SPEC>;
#[doc = "Field `VTRIP_SEL0_0` reader - N/A"]
pub type VTRIP_SEL0_0_R = crate::BitReader<VTRIP_SEL0_0_A>;
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VTRIP_SEL0_0_A {
    #[doc = "0: S40E full encoding is shown in CFG_IN_AUTOLVL.VTRIP_SEL0_1"]
    CMOS = 0,
    #[doc = "1: S40E full encoding is shown in CFG_IN_AUTOLVL.VTRIP_SEL0_1"]
    TTL = 1,
}
impl From<VTRIP_SEL0_0_A> for bool {
    #[inline(always)]
    fn from(variant: VTRIP_SEL0_0_A) -> Self {
        variant as u8 != 0
    }
}
impl VTRIP_SEL0_0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VTRIP_SEL0_0_A {
        match self.bits {
            false => VTRIP_SEL0_0_A::CMOS,
            true => VTRIP_SEL0_0_A::TTL,
        }
    }
    #[doc = "S40E full encoding is shown in CFG_IN_AUTOLVL.VTRIP_SEL0_1"]
    #[inline(always)]
    pub fn is_cmos(&self) -> bool {
        *self == VTRIP_SEL0_0_A::CMOS
    }
    #[doc = "S40E full encoding is shown in CFG_IN_AUTOLVL.VTRIP_SEL0_1"]
    #[inline(always)]
    pub fn is_ttl(&self) -> bool {
        *self == VTRIP_SEL0_0_A::TTL
    }
}
#[doc = "Field `VTRIP_SEL0_0` writer - N/A"]
pub type VTRIP_SEL0_0_W<'a, REG> = crate::BitWriter<'a, REG, VTRIP_SEL0_0_A>;
impl<'a, REG> VTRIP_SEL0_0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "S40E full encoding is shown in CFG_IN_AUTOLVL.VTRIP_SEL0_1"]
    #[inline(always)]
    pub fn cmos(self) -> &'a mut crate::W<REG> {
        self.variant(VTRIP_SEL0_0_A::CMOS)
    }
    #[doc = "S40E full encoding is shown in CFG_IN_AUTOLVL.VTRIP_SEL0_1"]
    #[inline(always)]
    pub fn ttl(self) -> &'a mut crate::W<REG> {
        self.variant(VTRIP_SEL0_0_A::TTL)
    }
}
#[doc = "Field `VTRIP_SEL1_0` reader - N/A"]
pub type VTRIP_SEL1_0_R = crate::BitReader;
#[doc = "Field `VTRIP_SEL1_0` writer - N/A"]
pub type VTRIP_SEL1_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTRIP_SEL2_0` reader - N/A"]
pub type VTRIP_SEL2_0_R = crate::BitReader;
#[doc = "Field `VTRIP_SEL2_0` writer - N/A"]
pub type VTRIP_SEL2_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTRIP_SEL3_0` reader - N/A"]
pub type VTRIP_SEL3_0_R = crate::BitReader;
#[doc = "Field `VTRIP_SEL3_0` writer - N/A"]
pub type VTRIP_SEL3_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTRIP_SEL4_0` reader - N/A"]
pub type VTRIP_SEL4_0_R = crate::BitReader;
#[doc = "Field `VTRIP_SEL4_0` writer - N/A"]
pub type VTRIP_SEL4_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTRIP_SEL5_0` reader - N/A"]
pub type VTRIP_SEL5_0_R = crate::BitReader;
#[doc = "Field `VTRIP_SEL5_0` writer - N/A"]
pub type VTRIP_SEL5_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTRIP_SEL6_0` reader - N/A"]
pub type VTRIP_SEL6_0_R = crate::BitReader;
#[doc = "Field `VTRIP_SEL6_0` writer - N/A"]
pub type VTRIP_SEL6_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTRIP_SEL7_0` reader - N/A"]
pub type VTRIP_SEL7_0_R = crate::BitReader;
#[doc = "Field `VTRIP_SEL7_0` writer - N/A"]
pub type VTRIP_SEL7_0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - N/A"]
    #[inline(always)]
    pub fn vtrip_sel0_0(&self) -> VTRIP_SEL0_0_R {
        VTRIP_SEL0_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn vtrip_sel1_0(&self) -> VTRIP_SEL1_0_R {
        VTRIP_SEL1_0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn vtrip_sel2_0(&self) -> VTRIP_SEL2_0_R {
        VTRIP_SEL2_0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn vtrip_sel3_0(&self) -> VTRIP_SEL3_0_R {
        VTRIP_SEL3_0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn vtrip_sel4_0(&self) -> VTRIP_SEL4_0_R {
        VTRIP_SEL4_0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn vtrip_sel5_0(&self) -> VTRIP_SEL5_0_R {
        VTRIP_SEL5_0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn vtrip_sel6_0(&self) -> VTRIP_SEL6_0_R {
        VTRIP_SEL6_0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn vtrip_sel7_0(&self) -> VTRIP_SEL7_0_R {
        VTRIP_SEL7_0_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn vtrip_sel0_0(&mut self) -> VTRIP_SEL0_0_W<CFG_IN_SPEC> {
        VTRIP_SEL0_0_W::new(self, 0)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn vtrip_sel1_0(&mut self) -> VTRIP_SEL1_0_W<CFG_IN_SPEC> {
        VTRIP_SEL1_0_W::new(self, 1)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn vtrip_sel2_0(&mut self) -> VTRIP_SEL2_0_W<CFG_IN_SPEC> {
        VTRIP_SEL2_0_W::new(self, 2)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn vtrip_sel3_0(&mut self) -> VTRIP_SEL3_0_W<CFG_IN_SPEC> {
        VTRIP_SEL3_0_W::new(self, 3)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn vtrip_sel4_0(&mut self) -> VTRIP_SEL4_0_W<CFG_IN_SPEC> {
        VTRIP_SEL4_0_W::new(self, 4)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn vtrip_sel5_0(&mut self) -> VTRIP_SEL5_0_W<CFG_IN_SPEC> {
        VTRIP_SEL5_0_W::new(self, 5)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn vtrip_sel6_0(&mut self) -> VTRIP_SEL6_0_W<CFG_IN_SPEC> {
        VTRIP_SEL6_0_W::new(self, 6)
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn vtrip_sel7_0(&mut self) -> VTRIP_SEL7_0_W<CFG_IN_SPEC> {
        VTRIP_SEL7_0_W::new(self, 7)
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
#[doc = "Port input buffer configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_in::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_in::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_IN_SPEC;
impl crate::RegisterSpec for CFG_IN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_in::R`](R) reader structure"]
impl crate::Readable for CFG_IN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg_in::W`](W) writer structure"]
impl crate::Writable for CFG_IN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG_IN to value 0"]
impl crate::Resettable for CFG_IN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
