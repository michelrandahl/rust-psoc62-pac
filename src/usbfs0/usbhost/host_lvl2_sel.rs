#[doc = "Register `HOST_LVL2_SEL` reader"]
pub type R = crate::R<HOST_LVL2_SEL_SPEC>;
#[doc = "Register `HOST_LVL2_SEL` writer"]
pub type W = crate::W<HOST_LVL2_SEL_SPEC>;
#[doc = "Field `EP1_DRQ_SEL` reader - These bits assign EP1_DRQ interrupt flag to any interrupt signals."]
pub type EP1_DRQ_SEL_R = crate::FieldReader<EP1_DRQ_SEL_A>;
#[doc = "These bits assign EP1_DRQ interrupt flag to any interrupt signals.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EP1_DRQ_SEL_A {
    #[doc = "0: High priority interrupt"]
    HI = 0,
    #[doc = "1: Medium priority interrupt"]
    MED = 1,
    #[doc = "2: Low priority interrupt"]
    LO = 2,
    #[doc = "3: illegal"]
    RSVD = 3,
}
impl From<EP1_DRQ_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: EP1_DRQ_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EP1_DRQ_SEL_A {
    type Ux = u8;
}
impl EP1_DRQ_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EP1_DRQ_SEL_A {
        match self.bits {
            0 => EP1_DRQ_SEL_A::HI,
            1 => EP1_DRQ_SEL_A::MED,
            2 => EP1_DRQ_SEL_A::LO,
            3 => EP1_DRQ_SEL_A::RSVD,
            _ => unreachable!(),
        }
    }
    #[doc = "High priority interrupt"]
    #[inline(always)]
    pub fn is_hi(&self) -> bool {
        *self == EP1_DRQ_SEL_A::HI
    }
    #[doc = "Medium priority interrupt"]
    #[inline(always)]
    pub fn is_med(&self) -> bool {
        *self == EP1_DRQ_SEL_A::MED
    }
    #[doc = "Low priority interrupt"]
    #[inline(always)]
    pub fn is_lo(&self) -> bool {
        *self == EP1_DRQ_SEL_A::LO
    }
    #[doc = "illegal"]
    #[inline(always)]
    pub fn is_rsvd(&self) -> bool {
        *self == EP1_DRQ_SEL_A::RSVD
    }
}
#[doc = "Field `EP1_DRQ_SEL` writer - These bits assign EP1_DRQ interrupt flag to any interrupt signals."]
pub type EP1_DRQ_SEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, EP1_DRQ_SEL_A>;
impl<'a, REG> EP1_DRQ_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "High priority interrupt"]
    #[inline(always)]
    pub fn hi(self) -> &'a mut crate::W<REG> {
        self.variant(EP1_DRQ_SEL_A::HI)
    }
    #[doc = "Medium priority interrupt"]
    #[inline(always)]
    pub fn med(self) -> &'a mut crate::W<REG> {
        self.variant(EP1_DRQ_SEL_A::MED)
    }
    #[doc = "Low priority interrupt"]
    #[inline(always)]
    pub fn lo(self) -> &'a mut crate::W<REG> {
        self.variant(EP1_DRQ_SEL_A::LO)
    }
    #[doc = "illegal"]
    #[inline(always)]
    pub fn rsvd(self) -> &'a mut crate::W<REG> {
        self.variant(EP1_DRQ_SEL_A::RSVD)
    }
}
#[doc = "Field `EP1_SPK_SEL` reader - These bits assign EP1_SPK interrupt flag to any interrupt signals."]
pub type EP1_SPK_SEL_R = crate::FieldReader;
#[doc = "Field `EP1_SPK_SEL` writer - These bits assign EP1_SPK interrupt flag to any interrupt signals."]
pub type EP1_SPK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EP2_DRQ_SEL` reader - These bits assign EP2_DRQ interrupt flag to any interrupt signals."]
pub type EP2_DRQ_SEL_R = crate::FieldReader;
#[doc = "Field `EP2_DRQ_SEL` writer - These bits assign EP2_DRQ interrupt flag to any interrupt signals."]
pub type EP2_DRQ_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EP2_SPK_SEL` reader - These bits assign EP2_SPK interrupt flag to any interrupt signals."]
pub type EP2_SPK_SEL_R = crate::FieldReader;
#[doc = "Field `EP2_SPK_SEL` writer - These bits assign EP2_SPK interrupt flag to any interrupt signals."]
pub type EP2_SPK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 4:5 - These bits assign EP1_DRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn ep1_drq_sel(&self) -> EP1_DRQ_SEL_R {
        EP1_DRQ_SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - These bits assign EP1_SPK interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn ep1_spk_sel(&self) -> EP1_SPK_SEL_R {
        EP1_SPK_SEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - These bits assign EP2_DRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn ep2_drq_sel(&self) -> EP2_DRQ_SEL_R {
        EP2_DRQ_SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - These bits assign EP2_SPK interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn ep2_spk_sel(&self) -> EP2_SPK_SEL_R {
        EP2_SPK_SEL_R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - These bits assign EP1_DRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    #[must_use]
    pub fn ep1_drq_sel(&mut self) -> EP1_DRQ_SEL_W<HOST_LVL2_SEL_SPEC> {
        EP1_DRQ_SEL_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - These bits assign EP1_SPK interrupt flag to any interrupt signals."]
    #[inline(always)]
    #[must_use]
    pub fn ep1_spk_sel(&mut self) -> EP1_SPK_SEL_W<HOST_LVL2_SEL_SPEC> {
        EP1_SPK_SEL_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - These bits assign EP2_DRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    #[must_use]
    pub fn ep2_drq_sel(&mut self) -> EP2_DRQ_SEL_W<HOST_LVL2_SEL_SPEC> {
        EP2_DRQ_SEL_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - These bits assign EP2_SPK interrupt flag to any interrupt signals."]
    #[inline(always)]
    #[must_use]
    pub fn ep2_spk_sel(&mut self) -> EP2_SPK_SEL_W<HOST_LVL2_SEL_SPEC> {
        EP2_SPK_SEL_W::new(self, 10)
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
#[doc = "Host Interrupt Level 2 Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_lvl2_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_lvl2_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_LVL2_SEL_SPEC;
impl crate::RegisterSpec for HOST_LVL2_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_lvl2_sel::R`](R) reader structure"]
impl crate::Readable for HOST_LVL2_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_lvl2_sel::W`](W) writer structure"]
impl crate::Writable for HOST_LVL2_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOST_LVL2_SEL to value 0"]
impl crate::Resettable for HOST_LVL2_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
