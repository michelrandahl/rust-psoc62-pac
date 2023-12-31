#[doc = "Register `MS6_CTL` reader"]
pub type R = crate::R<MS6_CTL_SPEC>;
#[doc = "Register `MS6_CTL` writer"]
pub type W = crate::W<MS6_CTL_SPEC>;
#[doc = "Field `P` reader - See MS0_CTL.P."]
pub type P_R = crate::BitReader;
#[doc = "Field `P` writer - See MS0_CTL.P."]
pub type P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NS` reader - See MS0_CTL.NS."]
pub type NS_R = crate::BitReader;
#[doc = "Field `NS` writer - See MS0_CTL.NS."]
pub type NS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIO` reader - See MS0_CTL.PRIO"]
pub type PRIO_R = crate::FieldReader;
#[doc = "Field `PRIO` writer - See MS0_CTL.PRIO"]
pub type PRIO_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC_MASK_0` reader - See MS0_CTL.PC_MASK_0."]
pub type PC_MASK_0_R = crate::BitReader;
#[doc = "Field `PC_MASK_15_TO_1` reader - See MS0_CTL.PC_MASK_15_TO_1."]
pub type PC_MASK_15_TO_1_R = crate::FieldReader<u16>;
#[doc = "Field `PC_MASK_15_TO_1` writer - See MS0_CTL.PC_MASK_15_TO_1."]
pub type PC_MASK_15_TO_1_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bit 0 - See MS0_CTL.P."]
    #[inline(always)]
    pub fn p(&self) -> P_R {
        P_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - See MS0_CTL.NS."]
    #[inline(always)]
    pub fn ns(&self) -> NS_R {
        NS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:9 - See MS0_CTL.PRIO"]
    #[inline(always)]
    pub fn prio(&self) -> PRIO_R {
        PRIO_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - See MS0_CTL.PC_MASK_0."]
    #[inline(always)]
    pub fn pc_mask_0(&self) -> PC_MASK_0_R {
        PC_MASK_0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31 - See MS0_CTL.PC_MASK_15_TO_1."]
    #[inline(always)]
    pub fn pc_mask_15_to_1(&self) -> PC_MASK_15_TO_1_R {
        PC_MASK_15_TO_1_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - See MS0_CTL.P."]
    #[inline(always)]
    #[must_use]
    pub fn p(&mut self) -> P_W<MS6_CTL_SPEC> {
        P_W::new(self, 0)
    }
    #[doc = "Bit 1 - See MS0_CTL.NS."]
    #[inline(always)]
    #[must_use]
    pub fn ns(&mut self) -> NS_W<MS6_CTL_SPEC> {
        NS_W::new(self, 1)
    }
    #[doc = "Bits 8:9 - See MS0_CTL.PRIO"]
    #[inline(always)]
    #[must_use]
    pub fn prio(&mut self) -> PRIO_W<MS6_CTL_SPEC> {
        PRIO_W::new(self, 8)
    }
    #[doc = "Bits 17:31 - See MS0_CTL.PC_MASK_15_TO_1."]
    #[inline(always)]
    #[must_use]
    pub fn pc_mask_15_to_1(&mut self) -> PC_MASK_15_TO_1_W<MS6_CTL_SPEC> {
        PC_MASK_15_TO_1_W::new(self, 17)
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
#[doc = "Master 6 protection context control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ms6_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ms6_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MS6_CTL_SPEC;
impl crate::RegisterSpec for MS6_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ms6_ctl::R`](R) reader structure"]
impl crate::Readable for MS6_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ms6_ctl::W`](W) writer structure"]
impl crate::Writable for MS6_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MS6_CTL to value 0x0303"]
impl crate::Resettable for MS6_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0303;
}
