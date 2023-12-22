#[doc = "Register `HOST_LVL1_SEL` reader"]
pub type R = crate::R<HOST_LVL1_SEL_SPEC>;
#[doc = "Register `HOST_LVL1_SEL` writer"]
pub type W = crate::W<HOST_LVL1_SEL_SPEC>;
#[doc = "Field `SOFIRQ_SEL` reader - These bits assign SOFIRQ interrupt flag to any interrupt signals."]
pub type SOFIRQ_SEL_R = crate::FieldReader<SOFIRQ_SEL_A>;
#[doc = "These bits assign SOFIRQ interrupt flag to any interrupt signals.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SOFIRQ_SEL_A {
    #[doc = "0: High priority interrupt"]
    HI = 0,
    #[doc = "1: Medium priority interrupt"]
    MED = 1,
    #[doc = "2: Low priority interrupt"]
    LO = 2,
    #[doc = "3: illegal"]
    RSVD = 3,
}
impl From<SOFIRQ_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SOFIRQ_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SOFIRQ_SEL_A {
    type Ux = u8;
}
impl SOFIRQ_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SOFIRQ_SEL_A {
        match self.bits {
            0 => SOFIRQ_SEL_A::HI,
            1 => SOFIRQ_SEL_A::MED,
            2 => SOFIRQ_SEL_A::LO,
            3 => SOFIRQ_SEL_A::RSVD,
            _ => unreachable!(),
        }
    }
    #[doc = "High priority interrupt"]
    #[inline(always)]
    pub fn is_hi(&self) -> bool {
        *self == SOFIRQ_SEL_A::HI
    }
    #[doc = "Medium priority interrupt"]
    #[inline(always)]
    pub fn is_med(&self) -> bool {
        *self == SOFIRQ_SEL_A::MED
    }
    #[doc = "Low priority interrupt"]
    #[inline(always)]
    pub fn is_lo(&self) -> bool {
        *self == SOFIRQ_SEL_A::LO
    }
    #[doc = "illegal"]
    #[inline(always)]
    pub fn is_rsvd(&self) -> bool {
        *self == SOFIRQ_SEL_A::RSVD
    }
}
#[doc = "Field `SOFIRQ_SEL` writer - These bits assign SOFIRQ interrupt flag to any interrupt signals."]
pub type SOFIRQ_SEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SOFIRQ_SEL_A>;
impl<'a, REG> SOFIRQ_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "High priority interrupt"]
    #[inline(always)]
    pub fn hi(self) -> &'a mut crate::W<REG> {
        self.variant(SOFIRQ_SEL_A::HI)
    }
    #[doc = "Medium priority interrupt"]
    #[inline(always)]
    pub fn med(self) -> &'a mut crate::W<REG> {
        self.variant(SOFIRQ_SEL_A::MED)
    }
    #[doc = "Low priority interrupt"]
    #[inline(always)]
    pub fn lo(self) -> &'a mut crate::W<REG> {
        self.variant(SOFIRQ_SEL_A::LO)
    }
    #[doc = "illegal"]
    #[inline(always)]
    pub fn rsvd(self) -> &'a mut crate::W<REG> {
        self.variant(SOFIRQ_SEL_A::RSVD)
    }
}
#[doc = "Field `DIRQ_SEL` reader - These bits assign DIRQ interrupt flag to any interrupt signals."]
pub type DIRQ_SEL_R = crate::FieldReader;
#[doc = "Field `DIRQ_SEL` writer - These bits assign DIRQ interrupt flag to any interrupt signals."]
pub type DIRQ_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CNNIRQ_SEL` reader - These bits assign CNNIRQ interrupt flag to any interrupt signals."]
pub type CNNIRQ_SEL_R = crate::FieldReader;
#[doc = "Field `CNNIRQ_SEL` writer - These bits assign CNNIRQ interrupt flag to any interrupt signals."]
pub type CNNIRQ_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMPIRQ_SEL` reader - These bits assign URIRQ interrupt flag to any interrupt signals."]
pub type CMPIRQ_SEL_R = crate::FieldReader;
#[doc = "Field `CMPIRQ_SEL` writer - These bits assign URIRQ interrupt flag to any interrupt signals."]
pub type CMPIRQ_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `URIRQ_SEL` reader - These bits assign URIRQ interrupt flag to any interrupt signals."]
pub type URIRQ_SEL_R = crate::FieldReader;
#[doc = "Field `URIRQ_SEL` writer - These bits assign URIRQ interrupt flag to any interrupt signals."]
pub type URIRQ_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RWKIRQ_SEL` reader - These bits assign RWKIRQ interrupt flag to any interrupt signals."]
pub type RWKIRQ_SEL_R = crate::FieldReader;
#[doc = "Field `RWKIRQ_SEL` writer - These bits assign RWKIRQ interrupt flag to any interrupt signals."]
pub type RWKIRQ_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RSVD_13_12` reader - N/A"]
pub type RSVD_13_12_R = crate::FieldReader;
#[doc = "Field `RSVD_13_12` writer - N/A"]
pub type RSVD_13_12_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TCAN_SEL` reader - These bits assign TCAN interrupt flag to any interrupt signals."]
pub type TCAN_SEL_R = crate::FieldReader;
#[doc = "Field `TCAN_SEL` writer - These bits assign TCAN interrupt flag to any interrupt signals."]
pub type TCAN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - These bits assign SOFIRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn sofirq_sel(&self) -> SOFIRQ_SEL_R {
        SOFIRQ_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - These bits assign DIRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn dirq_sel(&self) -> DIRQ_SEL_R {
        DIRQ_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - These bits assign CNNIRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn cnnirq_sel(&self) -> CNNIRQ_SEL_R {
        CNNIRQ_SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - These bits assign URIRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn cmpirq_sel(&self) -> CMPIRQ_SEL_R {
        CMPIRQ_SEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - These bits assign URIRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn urirq_sel(&self) -> URIRQ_SEL_R {
        URIRQ_SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - These bits assign RWKIRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn rwkirq_sel(&self) -> RWKIRQ_SEL_R {
        RWKIRQ_SEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - N/A"]
    #[inline(always)]
    pub fn rsvd_13_12(&self) -> RSVD_13_12_R {
        RSVD_13_12_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - These bits assign TCAN interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn tcan_sel(&self) -> TCAN_SEL_R {
        TCAN_SEL_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - These bits assign SOFIRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    #[must_use]
    pub fn sofirq_sel(&mut self) -> SOFIRQ_SEL_W<HOST_LVL1_SEL_SPEC> {
        SOFIRQ_SEL_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - These bits assign DIRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    #[must_use]
    pub fn dirq_sel(&mut self) -> DIRQ_SEL_W<HOST_LVL1_SEL_SPEC> {
        DIRQ_SEL_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - These bits assign CNNIRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    #[must_use]
    pub fn cnnirq_sel(&mut self) -> CNNIRQ_SEL_W<HOST_LVL1_SEL_SPEC> {
        CNNIRQ_SEL_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - These bits assign URIRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    #[must_use]
    pub fn cmpirq_sel(&mut self) -> CMPIRQ_SEL_W<HOST_LVL1_SEL_SPEC> {
        CMPIRQ_SEL_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - These bits assign URIRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    #[must_use]
    pub fn urirq_sel(&mut self) -> URIRQ_SEL_W<HOST_LVL1_SEL_SPEC> {
        URIRQ_SEL_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - These bits assign RWKIRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    #[must_use]
    pub fn rwkirq_sel(&mut self) -> RWKIRQ_SEL_W<HOST_LVL1_SEL_SPEC> {
        RWKIRQ_SEL_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn rsvd_13_12(&mut self) -> RSVD_13_12_W<HOST_LVL1_SEL_SPEC> {
        RSVD_13_12_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - These bits assign TCAN interrupt flag to any interrupt signals."]
    #[inline(always)]
    #[must_use]
    pub fn tcan_sel(&mut self) -> TCAN_SEL_W<HOST_LVL1_SEL_SPEC> {
        TCAN_SEL_W::new(self, 14)
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
#[doc = "Host Interrupt Level 1 Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_lvl1_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_lvl1_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_LVL1_SEL_SPEC;
impl crate::RegisterSpec for HOST_LVL1_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_lvl1_sel::R`](R) reader structure"]
impl crate::Readable for HOST_LVL1_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_lvl1_sel::W`](W) writer structure"]
impl crate::Writable for HOST_LVL1_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOST_LVL1_SEL to value 0"]
impl crate::Resettable for HOST_LVL1_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
