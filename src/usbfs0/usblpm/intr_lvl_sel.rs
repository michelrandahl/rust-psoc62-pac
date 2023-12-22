#[doc = "Register `INTR_LVL_SEL` reader"]
pub type R = crate::R<INTR_LVL_SEL_SPEC>;
#[doc = "Register `INTR_LVL_SEL` writer"]
pub type W = crate::W<INTR_LVL_SEL_SPEC>;
#[doc = "Field `SOF_LVL_SEL` reader - USB SOF Interrupt level select"]
pub type SOF_LVL_SEL_R = crate::FieldReader<SOF_LVL_SEL_A>;
#[doc = "USB SOF Interrupt level select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SOF_LVL_SEL_A {
    #[doc = "0: High priority interrupt"]
    HI = 0,
    #[doc = "1: Medium priority interrupt"]
    MED = 1,
    #[doc = "2: Low priority interrupt"]
    LO = 2,
    #[doc = "3: illegal"]
    RSVD = 3,
}
impl From<SOF_LVL_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SOF_LVL_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SOF_LVL_SEL_A {
    type Ux = u8;
}
impl SOF_LVL_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SOF_LVL_SEL_A {
        match self.bits {
            0 => SOF_LVL_SEL_A::HI,
            1 => SOF_LVL_SEL_A::MED,
            2 => SOF_LVL_SEL_A::LO,
            3 => SOF_LVL_SEL_A::RSVD,
            _ => unreachable!(),
        }
    }
    #[doc = "High priority interrupt"]
    #[inline(always)]
    pub fn is_hi(&self) -> bool {
        *self == SOF_LVL_SEL_A::HI
    }
    #[doc = "Medium priority interrupt"]
    #[inline(always)]
    pub fn is_med(&self) -> bool {
        *self == SOF_LVL_SEL_A::MED
    }
    #[doc = "Low priority interrupt"]
    #[inline(always)]
    pub fn is_lo(&self) -> bool {
        *self == SOF_LVL_SEL_A::LO
    }
    #[doc = "illegal"]
    #[inline(always)]
    pub fn is_rsvd(&self) -> bool {
        *self == SOF_LVL_SEL_A::RSVD
    }
}
#[doc = "Field `SOF_LVL_SEL` writer - USB SOF Interrupt level select"]
pub type SOF_LVL_SEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SOF_LVL_SEL_A>;
impl<'a, REG> SOF_LVL_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "High priority interrupt"]
    #[inline(always)]
    pub fn hi(self) -> &'a mut crate::W<REG> {
        self.variant(SOF_LVL_SEL_A::HI)
    }
    #[doc = "Medium priority interrupt"]
    #[inline(always)]
    pub fn med(self) -> &'a mut crate::W<REG> {
        self.variant(SOF_LVL_SEL_A::MED)
    }
    #[doc = "Low priority interrupt"]
    #[inline(always)]
    pub fn lo(self) -> &'a mut crate::W<REG> {
        self.variant(SOF_LVL_SEL_A::LO)
    }
    #[doc = "illegal"]
    #[inline(always)]
    pub fn rsvd(self) -> &'a mut crate::W<REG> {
        self.variant(SOF_LVL_SEL_A::RSVD)
    }
}
#[doc = "Field `BUS_RESET_LVL_SEL` reader - BUS RESET Interrupt level select"]
pub type BUS_RESET_LVL_SEL_R = crate::FieldReader;
#[doc = "Field `BUS_RESET_LVL_SEL` writer - BUS RESET Interrupt level select"]
pub type BUS_RESET_LVL_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EP0_LVL_SEL` reader - EP0 Interrupt level select"]
pub type EP0_LVL_SEL_R = crate::FieldReader;
#[doc = "Field `EP0_LVL_SEL` writer - EP0 Interrupt level select"]
pub type EP0_LVL_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPM_LVL_SEL` reader - LPM Interrupt level select"]
pub type LPM_LVL_SEL_R = crate::FieldReader;
#[doc = "Field `LPM_LVL_SEL` writer - LPM Interrupt level select"]
pub type LPM_LVL_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RESUME_LVL_SEL` reader - Resume Interrupt level select"]
pub type RESUME_LVL_SEL_R = crate::FieldReader;
#[doc = "Field `RESUME_LVL_SEL` writer - Resume Interrupt level select"]
pub type RESUME_LVL_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ARB_EP_LVL_SEL` reader - Arbiter Endpoint Interrupt level select"]
pub type ARB_EP_LVL_SEL_R = crate::FieldReader;
#[doc = "Field `ARB_EP_LVL_SEL` writer - Arbiter Endpoint Interrupt level select"]
pub type ARB_EP_LVL_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EP1_LVL_SEL` reader - EP1 Interrupt level select"]
pub type EP1_LVL_SEL_R = crate::FieldReader;
#[doc = "Field `EP1_LVL_SEL` writer - EP1 Interrupt level select"]
pub type EP1_LVL_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EP2_LVL_SEL` reader - EP2 Interrupt level select"]
pub type EP2_LVL_SEL_R = crate::FieldReader;
#[doc = "Field `EP2_LVL_SEL` writer - EP2 Interrupt level select"]
pub type EP2_LVL_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EP3_LVL_SEL` reader - EP3 Interrupt level select"]
pub type EP3_LVL_SEL_R = crate::FieldReader;
#[doc = "Field `EP3_LVL_SEL` writer - EP3 Interrupt level select"]
pub type EP3_LVL_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EP4_LVL_SEL` reader - EP4 Interrupt level select"]
pub type EP4_LVL_SEL_R = crate::FieldReader;
#[doc = "Field `EP4_LVL_SEL` writer - EP4 Interrupt level select"]
pub type EP4_LVL_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EP5_LVL_SEL` reader - EP5 Interrupt level select"]
pub type EP5_LVL_SEL_R = crate::FieldReader;
#[doc = "Field `EP5_LVL_SEL` writer - EP5 Interrupt level select"]
pub type EP5_LVL_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EP6_LVL_SEL` reader - EP6 Interrupt level select"]
pub type EP6_LVL_SEL_R = crate::FieldReader;
#[doc = "Field `EP6_LVL_SEL` writer - EP6 Interrupt level select"]
pub type EP6_LVL_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EP7_LVL_SEL` reader - EP7 Interrupt level select"]
pub type EP7_LVL_SEL_R = crate::FieldReader;
#[doc = "Field `EP7_LVL_SEL` writer - EP7 Interrupt level select"]
pub type EP7_LVL_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EP8_LVL_SEL` reader - EP8 Interrupt level select"]
pub type EP8_LVL_SEL_R = crate::FieldReader;
#[doc = "Field `EP8_LVL_SEL` writer - EP8 Interrupt level select"]
pub type EP8_LVL_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - USB SOF Interrupt level select"]
    #[inline(always)]
    pub fn sof_lvl_sel(&self) -> SOF_LVL_SEL_R {
        SOF_LVL_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - BUS RESET Interrupt level select"]
    #[inline(always)]
    pub fn bus_reset_lvl_sel(&self) -> BUS_RESET_LVL_SEL_R {
        BUS_RESET_LVL_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - EP0 Interrupt level select"]
    #[inline(always)]
    pub fn ep0_lvl_sel(&self) -> EP0_LVL_SEL_R {
        EP0_LVL_SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - LPM Interrupt level select"]
    #[inline(always)]
    pub fn lpm_lvl_sel(&self) -> LPM_LVL_SEL_R {
        LPM_LVL_SEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Resume Interrupt level select"]
    #[inline(always)]
    pub fn resume_lvl_sel(&self) -> RESUME_LVL_SEL_R {
        RESUME_LVL_SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Arbiter Endpoint Interrupt level select"]
    #[inline(always)]
    pub fn arb_ep_lvl_sel(&self) -> ARB_EP_LVL_SEL_R {
        ARB_EP_LVL_SEL_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - EP1 Interrupt level select"]
    #[inline(always)]
    pub fn ep1_lvl_sel(&self) -> EP1_LVL_SEL_R {
        EP1_LVL_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - EP2 Interrupt level select"]
    #[inline(always)]
    pub fn ep2_lvl_sel(&self) -> EP2_LVL_SEL_R {
        EP2_LVL_SEL_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - EP3 Interrupt level select"]
    #[inline(always)]
    pub fn ep3_lvl_sel(&self) -> EP3_LVL_SEL_R {
        EP3_LVL_SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - EP4 Interrupt level select"]
    #[inline(always)]
    pub fn ep4_lvl_sel(&self) -> EP4_LVL_SEL_R {
        EP4_LVL_SEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - EP5 Interrupt level select"]
    #[inline(always)]
    pub fn ep5_lvl_sel(&self) -> EP5_LVL_SEL_R {
        EP5_LVL_SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - EP6 Interrupt level select"]
    #[inline(always)]
    pub fn ep6_lvl_sel(&self) -> EP6_LVL_SEL_R {
        EP6_LVL_SEL_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - EP7 Interrupt level select"]
    #[inline(always)]
    pub fn ep7_lvl_sel(&self) -> EP7_LVL_SEL_R {
        EP7_LVL_SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - EP8 Interrupt level select"]
    #[inline(always)]
    pub fn ep8_lvl_sel(&self) -> EP8_LVL_SEL_R {
        EP8_LVL_SEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - USB SOF Interrupt level select"]
    #[inline(always)]
    #[must_use]
    pub fn sof_lvl_sel(&mut self) -> SOF_LVL_SEL_W<INTR_LVL_SEL_SPEC> {
        SOF_LVL_SEL_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - BUS RESET Interrupt level select"]
    #[inline(always)]
    #[must_use]
    pub fn bus_reset_lvl_sel(&mut self) -> BUS_RESET_LVL_SEL_W<INTR_LVL_SEL_SPEC> {
        BUS_RESET_LVL_SEL_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - EP0 Interrupt level select"]
    #[inline(always)]
    #[must_use]
    pub fn ep0_lvl_sel(&mut self) -> EP0_LVL_SEL_W<INTR_LVL_SEL_SPEC> {
        EP0_LVL_SEL_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - LPM Interrupt level select"]
    #[inline(always)]
    #[must_use]
    pub fn lpm_lvl_sel(&mut self) -> LPM_LVL_SEL_W<INTR_LVL_SEL_SPEC> {
        LPM_LVL_SEL_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Resume Interrupt level select"]
    #[inline(always)]
    #[must_use]
    pub fn resume_lvl_sel(&mut self) -> RESUME_LVL_SEL_W<INTR_LVL_SEL_SPEC> {
        RESUME_LVL_SEL_W::new(self, 8)
    }
    #[doc = "Bits 14:15 - Arbiter Endpoint Interrupt level select"]
    #[inline(always)]
    #[must_use]
    pub fn arb_ep_lvl_sel(&mut self) -> ARB_EP_LVL_SEL_W<INTR_LVL_SEL_SPEC> {
        ARB_EP_LVL_SEL_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - EP1 Interrupt level select"]
    #[inline(always)]
    #[must_use]
    pub fn ep1_lvl_sel(&mut self) -> EP1_LVL_SEL_W<INTR_LVL_SEL_SPEC> {
        EP1_LVL_SEL_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - EP2 Interrupt level select"]
    #[inline(always)]
    #[must_use]
    pub fn ep2_lvl_sel(&mut self) -> EP2_LVL_SEL_W<INTR_LVL_SEL_SPEC> {
        EP2_LVL_SEL_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - EP3 Interrupt level select"]
    #[inline(always)]
    #[must_use]
    pub fn ep3_lvl_sel(&mut self) -> EP3_LVL_SEL_W<INTR_LVL_SEL_SPEC> {
        EP3_LVL_SEL_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - EP4 Interrupt level select"]
    #[inline(always)]
    #[must_use]
    pub fn ep4_lvl_sel(&mut self) -> EP4_LVL_SEL_W<INTR_LVL_SEL_SPEC> {
        EP4_LVL_SEL_W::new(self, 22)
    }
    #[doc = "Bits 24:25 - EP5 Interrupt level select"]
    #[inline(always)]
    #[must_use]
    pub fn ep5_lvl_sel(&mut self) -> EP5_LVL_SEL_W<INTR_LVL_SEL_SPEC> {
        EP5_LVL_SEL_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - EP6 Interrupt level select"]
    #[inline(always)]
    #[must_use]
    pub fn ep6_lvl_sel(&mut self) -> EP6_LVL_SEL_W<INTR_LVL_SEL_SPEC> {
        EP6_LVL_SEL_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - EP7 Interrupt level select"]
    #[inline(always)]
    #[must_use]
    pub fn ep7_lvl_sel(&mut self) -> EP7_LVL_SEL_W<INTR_LVL_SEL_SPEC> {
        EP7_LVL_SEL_W::new(self, 28)
    }
    #[doc = "Bits 30:31 - EP8 Interrupt level select"]
    #[inline(always)]
    #[must_use]
    pub fn ep8_lvl_sel(&mut self) -> EP8_LVL_SEL_W<INTR_LVL_SEL_SPEC> {
        EP8_LVL_SEL_W::new(self, 30)
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
#[doc = "Select interrupt level for each interrupt source\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_lvl_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_lvl_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_LVL_SEL_SPEC;
impl crate::RegisterSpec for INTR_LVL_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_lvl_sel::R`](R) reader structure"]
impl crate::Readable for INTR_LVL_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr_lvl_sel::W`](W) writer structure"]
impl crate::Writable for INTR_LVL_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_LVL_SEL to value 0"]
impl crate::Resettable for INTR_LVL_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
