#[doc = "Register `SRSS_INTR_CFG` reader"]
pub type R = crate::R<SRSS_INTR_CFG_SPEC>;
#[doc = "Register `SRSS_INTR_CFG` writer"]
pub type W = crate::W<SRSS_INTR_CFG_SPEC>;
#[doc = "Field `HVLVD1_EDGE_SEL` reader - Sets which edge(s) will trigger an IRQ for HVLVD1"]
pub type HVLVD1_EDGE_SEL_R = crate::FieldReader<HVLVD1_EDGE_SEL_A>;
#[doc = "Sets which edge(s) will trigger an IRQ for HVLVD1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HVLVD1_EDGE_SEL_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Rising edge"]
    RISING = 1,
    #[doc = "2: Falling edge"]
    FALLING = 2,
    #[doc = "3: Both rising and falling edges"]
    BOTH = 3,
}
impl From<HVLVD1_EDGE_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: HVLVD1_EDGE_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HVLVD1_EDGE_SEL_A {
    type Ux = u8;
}
impl HVLVD1_EDGE_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HVLVD1_EDGE_SEL_A {
        match self.bits {
            0 => HVLVD1_EDGE_SEL_A::DISABLE,
            1 => HVLVD1_EDGE_SEL_A::RISING,
            2 => HVLVD1_EDGE_SEL_A::FALLING,
            3 => HVLVD1_EDGE_SEL_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HVLVD1_EDGE_SEL_A::DISABLE
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == HVLVD1_EDGE_SEL_A::RISING
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == HVLVD1_EDGE_SEL_A::FALLING
    }
    #[doc = "Both rising and falling edges"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == HVLVD1_EDGE_SEL_A::BOTH
    }
}
#[doc = "Field `HVLVD1_EDGE_SEL` writer - Sets which edge(s) will trigger an IRQ for HVLVD1"]
pub type HVLVD1_EDGE_SEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, HVLVD1_EDGE_SEL_A>;
impl<'a, REG> HVLVD1_EDGE_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(HVLVD1_EDGE_SEL_A::DISABLE)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(HVLVD1_EDGE_SEL_A::RISING)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(HVLVD1_EDGE_SEL_A::FALLING)
    }
    #[doc = "Both rising and falling edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(HVLVD1_EDGE_SEL_A::BOTH)
    }
}
impl R {
    #[doc = "Bits 0:1 - Sets which edge(s) will trigger an IRQ for HVLVD1"]
    #[inline(always)]
    pub fn hvlvd1_edge_sel(&self) -> HVLVD1_EDGE_SEL_R {
        HVLVD1_EDGE_SEL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Sets which edge(s) will trigger an IRQ for HVLVD1"]
    #[inline(always)]
    #[must_use]
    pub fn hvlvd1_edge_sel(&mut self) -> HVLVD1_EDGE_SEL_W<SRSS_INTR_CFG_SPEC> {
        HVLVD1_EDGE_SEL_W::new(self, 0)
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
#[doc = "SRSS Interrupt Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srss_intr_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srss_intr_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRSS_INTR_CFG_SPEC;
impl crate::RegisterSpec for SRSS_INTR_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srss_intr_cfg::R`](R) reader structure"]
impl crate::Readable for SRSS_INTR_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`srss_intr_cfg::W`](W) writer structure"]
impl crate::Writable for SRSS_INTR_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRSS_INTR_CFG to value 0"]
impl crate::Resettable for SRSS_INTR_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
