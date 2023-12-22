#[doc = "Register `TR_CTL[%s]` reader"]
pub type R = crate::R<TR_CTL_SPEC>;
#[doc = "Register `TR_CTL[%s]` writer"]
pub type W = crate::W<TR_CTL_SPEC>;
#[doc = "Field `TR_SEL` reader - Specifies input trigger: '0'': constant signal level '0'. '1': input trigger."]
pub type TR_SEL_R = crate::BitReader;
#[doc = "Field `TR_SEL` writer - Specifies input trigger: '0'': constant signal level '0'. '1': input trigger."]
pub type TR_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR_INV` reader - Specifies if the output trigger is inverted."]
pub type TR_INV_R = crate::BitReader;
#[doc = "Field `TR_INV` writer - Specifies if the output trigger is inverted."]
pub type TR_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR_EDGE` reader - Specifies if the (inverted) output trigger is treated as a level sensitive or edge sensitive trigger. '0': level sensitive. '1': edge sensitive trigger. The (inverted) output trigger duration needs to be at least 2 cycles on the consumer clock. the(inverted) output trigger is synchronized to the consumer clock and a two cycle pulse is generated on the consumer clock."]
pub type TR_EDGE_R = crate::BitReader;
#[doc = "Field `TR_EDGE` writer - Specifies if the (inverted) output trigger is treated as a level sensitive or edge sensitive trigger. '0': level sensitive. '1': edge sensitive trigger. The (inverted) output trigger duration needs to be at least 2 cycles on the consumer clock. the(inverted) output trigger is synchronized to the consumer clock and a two cycle pulse is generated on the consumer clock."]
pub type TR_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_FREEZE_EN` reader - Specifies if the output trigger is blocked in debug mode. When set high tr_dbg_freeze will block the output trigger generation."]
pub type DBG_FREEZE_EN_R = crate::BitReader;
#[doc = "Field `DBG_FREEZE_EN` writer - Specifies if the output trigger is blocked in debug mode. When set high tr_dbg_freeze will block the output trigger generation."]
pub type DBG_FREEZE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Specifies input trigger: '0'': constant signal level '0'. '1': input trigger."]
    #[inline(always)]
    pub fn tr_sel(&self) -> TR_SEL_R {
        TR_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Specifies if the output trigger is inverted."]
    #[inline(always)]
    pub fn tr_inv(&self) -> TR_INV_R {
        TR_INV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Specifies if the (inverted) output trigger is treated as a level sensitive or edge sensitive trigger. '0': level sensitive. '1': edge sensitive trigger. The (inverted) output trigger duration needs to be at least 2 cycles on the consumer clock. the(inverted) output trigger is synchronized to the consumer clock and a two cycle pulse is generated on the consumer clock."]
    #[inline(always)]
    pub fn tr_edge(&self) -> TR_EDGE_R {
        TR_EDGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Specifies if the output trigger is blocked in debug mode. When set high tr_dbg_freeze will block the output trigger generation."]
    #[inline(always)]
    pub fn dbg_freeze_en(&self) -> DBG_FREEZE_EN_R {
        DBG_FREEZE_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Specifies input trigger: '0'': constant signal level '0'. '1': input trigger."]
    #[inline(always)]
    #[must_use]
    pub fn tr_sel(&mut self) -> TR_SEL_W<TR_CTL_SPEC> {
        TR_SEL_W::new(self, 0)
    }
    #[doc = "Bit 8 - Specifies if the output trigger is inverted."]
    #[inline(always)]
    #[must_use]
    pub fn tr_inv(&mut self) -> TR_INV_W<TR_CTL_SPEC> {
        TR_INV_W::new(self, 8)
    }
    #[doc = "Bit 9 - Specifies if the (inverted) output trigger is treated as a level sensitive or edge sensitive trigger. '0': level sensitive. '1': edge sensitive trigger. The (inverted) output trigger duration needs to be at least 2 cycles on the consumer clock. the(inverted) output trigger is synchronized to the consumer clock and a two cycle pulse is generated on the consumer clock."]
    #[inline(always)]
    #[must_use]
    pub fn tr_edge(&mut self) -> TR_EDGE_W<TR_CTL_SPEC> {
        TR_EDGE_W::new(self, 9)
    }
    #[doc = "Bit 12 - Specifies if the output trigger is blocked in debug mode. When set high tr_dbg_freeze will block the output trigger generation."]
    #[inline(always)]
    #[must_use]
    pub fn dbg_freeze_en(&mut self) -> DBG_FREEZE_EN_W<TR_CTL_SPEC> {
        DBG_FREEZE_EN_W::new(self, 12)
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
#[doc = "Trigger control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tr_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tr_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TR_CTL_SPEC;
impl crate::RegisterSpec for TR_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr_ctl::R`](R) reader structure"]
impl crate::Readable for TR_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tr_ctl::W`](W) writer structure"]
impl crate::Writable for TR_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TR_CTL[%s]
to value 0"]
impl crate::Resettable for TR_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
