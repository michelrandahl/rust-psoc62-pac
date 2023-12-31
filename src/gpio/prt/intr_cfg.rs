#[doc = "Register `INTR_CFG` reader"]
pub type R = crate::R<INTR_CFG_SPEC>;
#[doc = "Register `INTR_CFG` writer"]
pub type W = crate::W<INTR_CFG_SPEC>;
#[doc = "Field `EDGE0_SEL` reader - Sets which edge will trigger an IRQ for IO pin 0"]
pub type EDGE0_SEL_R = crate::FieldReader<EDGE0_SEL_A>;
#[doc = "Sets which edge will trigger an IRQ for IO pin 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EDGE0_SEL_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Rising edge"]
    RISING = 1,
    #[doc = "2: Falling edge"]
    FALLING = 2,
    #[doc = "3: Both rising and falling edges"]
    BOTH = 3,
}
impl From<EDGE0_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: EDGE0_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EDGE0_SEL_A {
    type Ux = u8;
}
impl EDGE0_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EDGE0_SEL_A {
        match self.bits {
            0 => EDGE0_SEL_A::DISABLE,
            1 => EDGE0_SEL_A::RISING,
            2 => EDGE0_SEL_A::FALLING,
            3 => EDGE0_SEL_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EDGE0_SEL_A::DISABLE
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == EDGE0_SEL_A::RISING
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == EDGE0_SEL_A::FALLING
    }
    #[doc = "Both rising and falling edges"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == EDGE0_SEL_A::BOTH
    }
}
#[doc = "Field `EDGE0_SEL` writer - Sets which edge will trigger an IRQ for IO pin 0"]
pub type EDGE0_SEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, EDGE0_SEL_A>;
impl<'a, REG> EDGE0_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EDGE0_SEL_A::DISABLE)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(EDGE0_SEL_A::RISING)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(EDGE0_SEL_A::FALLING)
    }
    #[doc = "Both rising and falling edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(EDGE0_SEL_A::BOTH)
    }
}
#[doc = "Field `EDGE1_SEL` reader - Sets which edge will trigger an IRQ for IO pin 1"]
pub type EDGE1_SEL_R = crate::FieldReader;
#[doc = "Field `EDGE1_SEL` writer - Sets which edge will trigger an IRQ for IO pin 1"]
pub type EDGE1_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EDGE2_SEL` reader - Sets which edge will trigger an IRQ for IO pin 2"]
pub type EDGE2_SEL_R = crate::FieldReader;
#[doc = "Field `EDGE2_SEL` writer - Sets which edge will trigger an IRQ for IO pin 2"]
pub type EDGE2_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EDGE3_SEL` reader - Sets which edge will trigger an IRQ for IO pin 3"]
pub type EDGE3_SEL_R = crate::FieldReader;
#[doc = "Field `EDGE3_SEL` writer - Sets which edge will trigger an IRQ for IO pin 3"]
pub type EDGE3_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EDGE4_SEL` reader - Sets which edge will trigger an IRQ for IO pin 4"]
pub type EDGE4_SEL_R = crate::FieldReader;
#[doc = "Field `EDGE4_SEL` writer - Sets which edge will trigger an IRQ for IO pin 4"]
pub type EDGE4_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EDGE5_SEL` reader - Sets which edge will trigger an IRQ for IO pin 5"]
pub type EDGE5_SEL_R = crate::FieldReader;
#[doc = "Field `EDGE5_SEL` writer - Sets which edge will trigger an IRQ for IO pin 5"]
pub type EDGE5_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EDGE6_SEL` reader - Sets which edge will trigger an IRQ for IO pin 6"]
pub type EDGE6_SEL_R = crate::FieldReader;
#[doc = "Field `EDGE6_SEL` writer - Sets which edge will trigger an IRQ for IO pin 6"]
pub type EDGE6_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EDGE7_SEL` reader - Sets which edge will trigger an IRQ for IO pin 7"]
pub type EDGE7_SEL_R = crate::FieldReader;
#[doc = "Field `EDGE7_SEL` writer - Sets which edge will trigger an IRQ for IO pin 7"]
pub type EDGE7_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FLT_EDGE_SEL` reader - Sets which edge will trigger an IRQ for the glitch filtered pin (selected by INTR_CFG.FLT_SEL"]
pub type FLT_EDGE_SEL_R = crate::FieldReader<FLT_EDGE_SEL_A>;
#[doc = "Sets which edge will trigger an IRQ for the glitch filtered pin (selected by INTR_CFG.FLT_SEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLT_EDGE_SEL_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Rising edge"]
    RISING = 1,
    #[doc = "2: Falling edge"]
    FALLING = 2,
    #[doc = "3: Both rising and falling edges"]
    BOTH = 3,
}
impl From<FLT_EDGE_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FLT_EDGE_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FLT_EDGE_SEL_A {
    type Ux = u8;
}
impl FLT_EDGE_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FLT_EDGE_SEL_A {
        match self.bits {
            0 => FLT_EDGE_SEL_A::DISABLE,
            1 => FLT_EDGE_SEL_A::RISING,
            2 => FLT_EDGE_SEL_A::FALLING,
            3 => FLT_EDGE_SEL_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FLT_EDGE_SEL_A::DISABLE
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == FLT_EDGE_SEL_A::RISING
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == FLT_EDGE_SEL_A::FALLING
    }
    #[doc = "Both rising and falling edges"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == FLT_EDGE_SEL_A::BOTH
    }
}
#[doc = "Field `FLT_EDGE_SEL` writer - Sets which edge will trigger an IRQ for the glitch filtered pin (selected by INTR_CFG.FLT_SEL"]
pub type FLT_EDGE_SEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, FLT_EDGE_SEL_A>;
impl<'a, REG> FLT_EDGE_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(FLT_EDGE_SEL_A::DISABLE)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(FLT_EDGE_SEL_A::RISING)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(FLT_EDGE_SEL_A::FALLING)
    }
    #[doc = "Both rising and falling edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(FLT_EDGE_SEL_A::BOTH)
    }
}
#[doc = "Field `FLT_SEL` reader - Selects which pin is routed through the 50ns glitch filter to provide a glitch-safe interrupt."]
pub type FLT_SEL_R = crate::FieldReader;
#[doc = "Field `FLT_SEL` writer - Selects which pin is routed through the 50ns glitch filter to provide a glitch-safe interrupt."]
pub type FLT_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - Sets which edge will trigger an IRQ for IO pin 0"]
    #[inline(always)]
    pub fn edge0_sel(&self) -> EDGE0_SEL_R {
        EDGE0_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Sets which edge will trigger an IRQ for IO pin 1"]
    #[inline(always)]
    pub fn edge1_sel(&self) -> EDGE1_SEL_R {
        EDGE1_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Sets which edge will trigger an IRQ for IO pin 2"]
    #[inline(always)]
    pub fn edge2_sel(&self) -> EDGE2_SEL_R {
        EDGE2_SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Sets which edge will trigger an IRQ for IO pin 3"]
    #[inline(always)]
    pub fn edge3_sel(&self) -> EDGE3_SEL_R {
        EDGE3_SEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Sets which edge will trigger an IRQ for IO pin 4"]
    #[inline(always)]
    pub fn edge4_sel(&self) -> EDGE4_SEL_R {
        EDGE4_SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Sets which edge will trigger an IRQ for IO pin 5"]
    #[inline(always)]
    pub fn edge5_sel(&self) -> EDGE5_SEL_R {
        EDGE5_SEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Sets which edge will trigger an IRQ for IO pin 6"]
    #[inline(always)]
    pub fn edge6_sel(&self) -> EDGE6_SEL_R {
        EDGE6_SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Sets which edge will trigger an IRQ for IO pin 7"]
    #[inline(always)]
    pub fn edge7_sel(&self) -> EDGE7_SEL_R {
        EDGE7_SEL_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Sets which edge will trigger an IRQ for the glitch filtered pin (selected by INTR_CFG.FLT_SEL"]
    #[inline(always)]
    pub fn flt_edge_sel(&self) -> FLT_EDGE_SEL_R {
        FLT_EDGE_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Selects which pin is routed through the 50ns glitch filter to provide a glitch-safe interrupt."]
    #[inline(always)]
    pub fn flt_sel(&self) -> FLT_SEL_R {
        FLT_SEL_R::new(((self.bits >> 18) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Sets which edge will trigger an IRQ for IO pin 0"]
    #[inline(always)]
    #[must_use]
    pub fn edge0_sel(&mut self) -> EDGE0_SEL_W<INTR_CFG_SPEC> {
        EDGE0_SEL_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Sets which edge will trigger an IRQ for IO pin 1"]
    #[inline(always)]
    #[must_use]
    pub fn edge1_sel(&mut self) -> EDGE1_SEL_W<INTR_CFG_SPEC> {
        EDGE1_SEL_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Sets which edge will trigger an IRQ for IO pin 2"]
    #[inline(always)]
    #[must_use]
    pub fn edge2_sel(&mut self) -> EDGE2_SEL_W<INTR_CFG_SPEC> {
        EDGE2_SEL_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Sets which edge will trigger an IRQ for IO pin 3"]
    #[inline(always)]
    #[must_use]
    pub fn edge3_sel(&mut self) -> EDGE3_SEL_W<INTR_CFG_SPEC> {
        EDGE3_SEL_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Sets which edge will trigger an IRQ for IO pin 4"]
    #[inline(always)]
    #[must_use]
    pub fn edge4_sel(&mut self) -> EDGE4_SEL_W<INTR_CFG_SPEC> {
        EDGE4_SEL_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Sets which edge will trigger an IRQ for IO pin 5"]
    #[inline(always)]
    #[must_use]
    pub fn edge5_sel(&mut self) -> EDGE5_SEL_W<INTR_CFG_SPEC> {
        EDGE5_SEL_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Sets which edge will trigger an IRQ for IO pin 6"]
    #[inline(always)]
    #[must_use]
    pub fn edge6_sel(&mut self) -> EDGE6_SEL_W<INTR_CFG_SPEC> {
        EDGE6_SEL_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Sets which edge will trigger an IRQ for IO pin 7"]
    #[inline(always)]
    #[must_use]
    pub fn edge7_sel(&mut self) -> EDGE7_SEL_W<INTR_CFG_SPEC> {
        EDGE7_SEL_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Sets which edge will trigger an IRQ for the glitch filtered pin (selected by INTR_CFG.FLT_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn flt_edge_sel(&mut self) -> FLT_EDGE_SEL_W<INTR_CFG_SPEC> {
        FLT_EDGE_SEL_W::new(self, 16)
    }
    #[doc = "Bits 18:20 - Selects which pin is routed through the 50ns glitch filter to provide a glitch-safe interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn flt_sel(&mut self) -> FLT_SEL_W<INTR_CFG_SPEC> {
        FLT_SEL_W::new(self, 18)
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
#[doc = "Port interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_CFG_SPEC;
impl crate::RegisterSpec for INTR_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_cfg::R`](R) reader structure"]
impl crate::Readable for INTR_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr_cfg::W`](W) writer structure"]
impl crate::Writable for INTR_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_CFG to value 0"]
impl crate::Resettable for INTR_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
